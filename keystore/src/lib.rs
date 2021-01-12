#![no_std]
#![feature(default_alloc_error_handler)]
extern crate alloc;

use core::cell::RefCell;

use alloc::{
    borrow::Cow,
    boxed::Box,
    collections::BTreeMap,
    string::{String, ToString},
    vec,
    vec::Vec,
};
use common::{err::Err, hash_utils, proto_utils};
use mw_rt::actor::Actor;

mod cypher;
// mod interface;
mod proto;
mod sql;
mod test;

// #[macro_use]
// extern crate lazy_static;
lazy_static! {
    pub static ref STATEMAP: StateMap = StateMap::new();
}

/// 0:unlock
/// 1:lock
pub struct StateMap {
    map: RefCell<BTreeMap<Vec<u8>, i32>>,
}

impl StateMap {
    fn new() -> Self {
        let map = RefCell::new(BTreeMap::new());

        Self { map }
    }

    pub fn insert(&self, key: Vec<u8>, state: i32) {
        self.map.borrow_mut().insert(key, state);
    }

    pub fn init(&self) {
        self.map.borrow_mut().clear();
    }

    pub fn get(&self, key: Vec<u8>) -> Option<i32> {
        match self.map.borrow_mut().get(&key) {
            Some(v) => Some(*v),
            None => None,
        }
    }
}

unsafe impl Sync for StateMap {}

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[mw_rt::actor::actor]
pub struct Keystore {}

#[async_trait::async_trait]
impl Actor for Keystore {
    fn new() -> Self {
        // 调用js提供的查表是否存在的方法
        let runtime = mw_rt::runtime::Runtime::new();
        runtime.spawn(async move {
            let flag = mw_std::sql::sql_table_exist("keystore".as_bytes()).await;
            //TODO debug
            mw_std::debug::println(&alloc::format!("{}", flag));
            match flag {
                0 => {
                    //TODO debug
                    mw_std::debug::println("keystore exist");
                    STATEMAP.init();
                }
                1 => {
                    //TODO debug
                    mw_std::debug::println("keystore not exist");
                    let mut sql = proto::common::Sql::default();
                    sql.sql = Cow::Borrowed(sql::_CREATE_KEYSTORE_TABLE);
                    match proto_utils::qb_serialize(&sql) {
                        Ok(v) => {
                            let result = mw_std::sql::sql_execute(v.as_slice(), 0).await;
                            match String::from_utf8(result) {
                                Ok(str) => match str.as_str() {
                                    "ok" => {
                                        STATEMAP.init();
                                    }
                                    "fail" => {}
                                    _ => {}
                                },
                                Err(err) => {
                                    let pair = Err::FromUtf8Error(err).get();
                                    mw_std::debug::println(&pair.1);
                                }
                            };
                        }
                        Err(err) => {
                            let pair = Err::ProtoErrors(err).get();
                            mw_std::debug::println(&pair.1);
                        }
                    };
                }
                _ => {}
            }
            //TODO debug
            mw_std::debug::println("end");
        });

        Keystore {}
    }

    async fn init(&mut self) {}
}

#[mw_rt::actor::expose]
impl Keystore {
    #[mw_rt::actor::method]
    pub async fn list_accounts(&mut self, page: usize, item: usize, order: usize) -> Vec<u8> {
        let mut sql = proto::common::Sql::default();
        sql.sql = alloc::format!(
            "select * from keystore limit {} offset {}",
            item,
            item * (page - 1)
        )
        .into();
        match proto_utils::qb_serialize(&sql) {
            Ok(v) => {
                let v = mw_std::sql::sql_execute(v.as_slice(), 1).await;
                match quick_protobuf::deserialize_from_slice::<proto::keystore::KeystoreList>(
                    v.as_slice(),
                ) {
                    Ok(keystore_list) => {
                        let mut keypair_list = proto::keystore::KeypairDisplayList::default();
                        for keystore in keystore_list.keystore_list {
                            let mut keypair = proto::keystore::KeypairDisplay::default();
                            keypair.public_key = keystore.public_key;
                            keypair.account = keystore.account;
                            keypair.ty = keystore.public_encrypt_type;
                            keypair.cert = keystore.cert;

                            keypair_list.keypair_display_list.push(keypair);
                        }

                        match proto_utils::qb_serialize(&keypair_list) {
                            Ok(v) => {
                                return v;
                            }
                            Err(err) => {
                                let _pair = Err::ProtoErrors(err).get();
                                return vec![];
                            }
                        };
                    }
                    Err(err) => {
                        let _pair = Err::ProtoErrors(err).get();
                        return vec![];
                    }
                };
            }
            Err(err) => {
                let _pair = Err::ProtoErrors(err).get();
                return vec![];
            }
        }
    }

    #[mw_rt::actor::method]
    pub async fn get_account(&mut self, account: &[u8]) -> Vec<u8> {
        let mut sql = proto::common::Sql::default();
        sql.sql = "select * from keystore where id = $1".into();
        sql.params.push(account.into());

        match proto_utils::qb_serialize(&sql) {
            Ok(v) => {
                let result = mw_std::sql::sql_execute(v.as_slice(), 1).await;
                match quick_protobuf::deserialize_from_slice::<proto::keystore::Keystore>(
                    result.as_slice(),
                ) {
                    Ok(keystore) => {
                        let mut keypair = proto::keystore::KeypairDisplay::default();
                        keypair.public_key = keystore.public_key;
                        keypair.account = keystore.account;
                        keypair.ty = keystore.public_encrypt_type;
                        keypair.cert = keypair.cert;

                        match proto_utils::qb_serialize(&keypair) {
                            Ok(v) => {
                                return v;
                            }
                            Err(err) => {
                                let _pair = Err::ProtoErrors(err).get();
                                return vec![];
                            }
                        };
                    }
                    Err(err) => {
                        let _pair = Err::ProtoErrors(err).get();
                        return vec![];
                    }
                };
            }
            Err(err) => {
                let _pair = Err::ProtoErrors(err).get();
                return vec![];
            }
        };
    }

    #[mw_rt::actor::method]
    pub async fn import_account(&mut self, bytes: &[u8]) -> i32 {
        match quick_protobuf::deserialize_from_slice::<proto::keystore::Keypair>(bytes) {
            Ok(keypair) => {
                let timestamp = mw_std::time::get_timestamp();

                let mut sql = proto::common::Sql::default();
                sql.sql = alloc::format!(
                    r#"
                inser into keystore(
                    account,
                    seed,
                    encrypt_code,
                    public_key,
                    secret_key,
                    cert,
                    nonce,
                    public_encrypt_type,
                    secret_encrypt_type,
                    timestamp
                ) values(
                    $1,$2,$3,$4,$5,$6,$7,{},{},{}
                )
                "#,
                    "SM2",
                    "ED25519",
                    timestamp
                )
                .into();

                sql.params.push(keypair.account);
                sql.params.push(keypair.seed);
                sql.params.push(keypair.encrypt_code);
                sql.params.push(keypair.public_key);
                sql.params.push(keypair.secret_key);
                sql.params.push("cert".as_bytes().into());
                sql.params.push(keypair.nonce);

                match proto_utils::qb_serialize(&sql) {
                    Ok(v) => {
                        let result = mw_std::sql::sql_execute(v.as_slice(), 0).await;
                        match String::from_utf8(result) {
                            Ok(str) => {
                                match str.as_str() {
                                    "ok" => {
                                        return 0;
                                    }
                                    "fail" => {
                                        let pair =
                                            Err::SqlExecture("import account fail".to_string())
                                                .get();
                                        return pair.0 as i32;
                                    }
                                    _ => {
                                        let pair =
                                            Err::SqlExecture("unknown return code".to_string())
                                                .get();
                                        return pair.0 as i32;
                                    }
                                };
                            }
                            Err(err) => {
                                let pair = Err::FromUtf8Error(err).get();
                                return pair.0 as i32;
                            }
                        };
                    }
                    Err(err) => {
                        let pair = Err::ProtoErrors(err).get();
                        return pair.0 as i32;
                    }
                };
            }
            Err(err) => {
                let pair = Err::ProtoErrors(err).get();
                return pair.0 as i32;
            }
        }
    }

    #[mw_rt::actor::method]
    pub async fn export_accounts(&mut self, account: &[u8]) -> Vec<u8> {
        let mut sql = proto::common::Sql::default();
        sql.sql = "select * from keystore where account = $1".into();
        sql.params.push(account.into());

        match proto_utils::qb_serialize(&sql) {
            Ok(v) => {
                let result = mw_std::sql::sql_execute(v.as_slice(), 1).await;
                return result;
            }
            Err(err) => {
                let _pair = Err::ProtoErrors(err).get();
                return vec![];
            }
        };
    }

    #[mw_rt::actor::method]
    pub async fn new_account(&mut self, bytes: &[u8]) -> i32 {
        match quick_protobuf::deserialize_from_slice::<proto::keystore::AccountMsg>(bytes) {
            Ok(msg) => {
                let secret = crate::cypher::ed_25519::Secret::gen().await;

                if secret.secret_key.is_none()
                    || secret.public_key.is_none()
                    || secret.seed.is_none()
                {
                    let pair = Err::Null(
                        "secret_key or public_key or seed seem null of gen secret".to_string(),
                    )
                    .get();
                    return pair.0 as i32;
                }
                let timestamp = mw_std::time::get_timestamp();
                let nonce = mw_std::rand::gen_rand32().await;

                let mut sql = proto::common::Sql::default();
                sql.sql = alloc::format!(
                    r#"
                inser into keystore(
                    account,
                    seed,
                    encrypt_code,
                    public_key,
                    secret_key,
                    cert,
                    nonce,
                    public_encrypt_type,
                    secret_encrypt_type,
                    timestamp
                ) values(
                    $1,$2,$3,$4,$5,$6,$7,{},{},{}
                )
                "#,
                    "SM2",
                    "ED25519",
                    timestamp
                )
                .into();

                let encrypt_code = msg.encrypt_code.to_vec();
                let secret_key = secret.secret_key.unwrap();
                let public_key = secret.public_key.unwrap();
                let seed = secret.seed.unwrap();

                //根据encrypt_code加密
                let secret_key_encode = cypher::xchacha20::xchacha20_encryption(
                    encrypt_code.as_slice(),
                    nonce.as_slice(),
                    secret_key.as_slice(),
                );
                let public_key_encode = cypher::xchacha20::xchacha20_encryption(
                    encrypt_code.as_slice(),
                    nonce.as_slice(),
                    public_key.as_slice(),
                );
                let seed_encode = cypher::xchacha20::xchacha20_encryption(
                    encrypt_code.as_slice(),
                    nonce.as_slice(),
                    seed.as_slice(),
                );
                let encrypt_code_encode = cypher::xchacha20::xchacha20_encryption(
                    encrypt_code.as_slice(),
                    nonce.as_slice(),
                    encrypt_code.as_slice(),
                );

                sql.params.push(msg.account);
                sql.params.push(seed_encode.into());
                sql.params.push(encrypt_code_encode.into());
                sql.params.push(public_key_encode.into());
                sql.params.push(secret_key_encode.into());
                sql.params.push("cert".as_bytes().into());
                sql.params.push(nonce.into());

                match proto_utils::qb_serialize(&sql) {
                    Ok(v) => {
                        let result = mw_std::sql::sql_execute(v.as_slice(), 0).await;
                        match String::from_utf8(result) {
                            Ok(str) => {
                                match str.as_str() {
                                    "ok" => {
                                        return 0;
                                    }
                                    "fail" => {
                                        let pair =
                                            Err::SqlExecture("import account fail".to_string())
                                                .get();
                                        return pair.0 as i32;
                                    }
                                    _ => {
                                        let pair =
                                            Err::SqlExecture("unknown return code".to_string())
                                                .get();
                                        return pair.0 as i32;
                                    }
                                };
                            }
                            Err(err) => {
                                let pair = Err::FromUtf8Error(err).get();
                                return pair.0 as i32;
                            }
                        };
                    }
                    Err(err) => {
                        let pair = Err::ProtoErrors(err).get();
                        return pair.0 as i32;
                    }
                }
            }
            Err(err) => {
                let pair = Err::ProtoErrors(err).get();
                return pair.0 as i32;
            }
        };
    }

    #[mw_rt::actor::method]
    pub async fn sign_message(&mut self, bytes: &[u8]) -> Vec<u8> {
        match quick_protobuf::deserialize_from_slice::<proto::keystore::Sign>(bytes) {
            Ok(sign) => {
                if sign.account_msg.is_none() {
                    let _pair =
                        Err::Null("the account msg seem null in sign message".to_string()).get();
                    return vec![];
                }
                let account_msg = sign.account_msg.unwrap();
                let account = account_msg.account.to_vec();
                let encrypt_code = account_msg.encrypt_code.to_vec();

                //检查私钥是否解锁,解锁的才能签名
                match crate::STATEMAP.get(account.clone().into()) {
                    Some(v) => {
                        if v == 1 {
                            let _pair = Err::AccountErrors(
                                "It has been locked. Please unlock it first".to_string(),
                            )
                            .get();
                            return vec![];
                        }
                    }
                    None => {
                        let _pair = Err::AccountErrors(
                            "the account internal error,contact the staff please".to_string(),
                        )
                        .get();
                        return vec![];
                    }
                };

                let mut sql = proto::common::Sql::default();
                sql.sql = "select * from keystore where account = $1".into();
                sql.params.push(account.into());

                match proto_utils::qb_serialize(&sql) {
                    Ok(v) => {
                        let result = mw_std::sql::sql_execute(v.as_slice(), 1).await;
                        match quick_protobuf::deserialize_from_slice::<proto::keystore::Keystore>(
                            result.as_slice(),
                        ) {
                            Ok(keystore) => {
                                let encrypt_code_decode = cypher::xchacha20::xchacha20_decryption(
                                    encrypt_code.as_slice(),
                                    keystore.nonce.as_ref(),
                                    keystore.encrypt_code.as_ref(),
                                );

                                if encrypt_code_decode != encrypt_code {
                                    let _pair =
                                        Err::AccountErrors("encrypt code error".to_string()).get();
                                    return vec![];
                                };

                                //解密public_key
                                let public_key = cypher::xchacha20::xchacha20_decryption(
                                    encrypt_code.as_slice(),
                                    keystore.nonce.as_ref(),
                                    keystore.public_key.as_ref(),
                                );
                                //解密secret_key
                                let secret_key = cypher::xchacha20::xchacha20_decryption(
                                    encrypt_code.as_slice(),
                                    keystore.nonce.as_ref(),
                                    keystore.secret_key.as_ref(),
                                );

                                return match crate::cypher::ed_25519::sign(
                                    public_key,
                                    secret_key,
                                    sign.message.as_ref(),
                                ) {
                                    Some(v) => v,
                                    None => {
                                        vec![]
                                    }
                                };
                            }
                            Err(err) => {
                                let _pair = Err::ProtoErrors(err).get();
                                return vec![];
                            }
                        }
                    }
                    Err(err) => {
                        let _pair = Err::ProtoErrors(err).get();
                        return vec![];
                    }
                };
            }
            Err(err) => {
                let _pair = Err::ProtoErrors(err).get();
                return vec![];
            }
        };
    }

    #[mw_rt::actor::method]
    pub async fn verify_sign(&mut self, bytes: &[u8]) -> i32 {
        match quick_protobuf::deserialize_from_slice::<proto::keystore::VerifySign>(bytes) {
            Ok(verify_sign) => {
                match verify_sign.VerfySign {
                    proto::keystore::mod_VerifySign::OneOfVerfySign::AccountVerifySign(avs) => {
                        match avs.account_msg {
                            Some(account_msg) => {
                                let account = account_msg.account.to_vec();
                                let mut sql = proto::common::Sql::default();
                                sql.sql = "select * from keystore where account = $1".into();
                                sql.params.push(account.into());

                                match proto_utils::qb_serialize(&sql) {
                                    Ok(v) => {
                                        let result =
                                            mw_std::sql::sql_execute(v.as_slice(), 1).await;
                                        let encrypt_code = account_msg.encrypt_code.to_vec();
                                        match quick_protobuf::deserialize_from_slice::<
                                            proto::keystore::Keystore,
                                        >(
                                            result.as_slice()
                                        ) {
                                            Ok(keystore) => {
                                                let encrypt_code_decode =
                                                    cypher::xchacha20::xchacha20_decryption(
                                                        encrypt_code.as_slice(),
                                                        keystore.nonce.as_ref(),
                                                        keystore.encrypt_code.as_ref(),
                                                    );

                                                if encrypt_code_decode != encrypt_code {
                                                    let pair = Err::AccountErrors(
                                                        "encrypt code error".to_string(),
                                                    )
                                                    .get();
                                                    return pair.0 as i32;
                                                };

                                                //解密public_key
                                                let public_key =
                                                    cypher::xchacha20::xchacha20_decryption(
                                                        encrypt_code.as_slice(),
                                                        keystore.nonce.as_ref(),
                                                        keystore.public_key.as_ref(),
                                                    );

                                                match cypher::ed_25519::verify_sign(
                                                    public_key.as_slice(),
                                                    avs.sign.as_ref(),
                                                    avs.message.as_ref(),
                                                ) {
                                                    Some(b) => {
                                                        if b {
                                                            return 0;
                                                        } else {
                                                            return 1;
                                                        }
                                                    }
                                                    None => {
                                                        let pair = Err::AccountErrors(
                                                            "verify sign error".to_string(),
                                                        )
                                                        .get();
                                                        return pair.0 as i32;
                                                    }
                                                }
                                            }
                                            Err(err) => {
                                                let pair = Err::ProtoErrors(err).get();
                                                return pair.0 as i32;
                                            }
                                        }
                                    }
                                    Err(err) => {
                                        let pair = Err::ProtoErrors(err).get();
                                        return pair.0 as i32;
                                    }
                                }
                            }
                            None => {
                                let pair = Err::Null("account msg is null".to_string()).get();
                                return pair.0 as i32;
                            }
                        }
                    }
                    proto::keystore::mod_VerifySign::OneOfVerfySign::PubVerifySign(pvs) => {}
                    proto::keystore::mod_VerifySign::OneOfVerfySign::None => {}
                }
            }
            Err(err) => {
                let pair = Err::ProtoErrors(err).get();
                return pair.0 as i32;
            }
        };
        0
    }

    #[mw_rt::actor::method]
    pub async fn lock_account(&mut self, btes: &[u8]) -> i32 {
        0
    }

    #[mw_rt::actor::method]
    pub async fn unlock_account(&mut self, btes: &[u8]) -> i32 {
        0
    }
}
