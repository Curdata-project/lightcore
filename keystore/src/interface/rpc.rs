//! RPC interfce
use crate::{cypher, proto};
use alloc::slice;
use alloc::string::String;
use alloc::vec::Vec;

/// 分页获取list
#[no_mangle]
pub extern "C" fn list_accounts(index: usize, page: usize, item: usize, _order: usize) {
    let runtime = mw_rt::runtime::Runtime::new();
    runtime.spawn(async move {
        let sql = alloc::format!(
            "select * from ketstore limit {} offset {}",
            item,
            item * (page - 1)
        );

        let v = mw_std::sql::sql_execute(sql.as_str(), 1).await;

        //将str rpc反序列化
        let keystore_list_result =
            quick_protobuf::deserialize_from_slice::<proto::keystore::KeystoreList>(v.as_slice());

        if keystore_list_result.as_ref().is_err() {
            return;
        }

        let keystore_list = keystore_list_result.unwrap();

        //解析
        let mut keystore_display_list = proto::keystore::KeypairDisplayList::default();

        for value in keystore_list.keystore_list {
            let mut keystore_display = proto::keystore::KeypairDisplay::default();

            keystore_display.account = value.account;
            keystore_display.cert = value.cert;
            keystore_display.public_key = value.public_key;
            keystore_display.ty = value.ty;

            keystore_display_list
                .keypair_display_list
                .push(keystore_display);
        }

        //序列化
        let mut out: Vec<u8> = Vec::new();
        let serialize_result =
            quick_protobuf::serialize_into_slice(&keystore_display_list, out.as_mut_slice());

        if serialize_result.as_ref().is_err() {
            return;
        }

        mw_std::notify::notify_ptr_size(index, &out);
    });
}

/// 根据account获取信息
#[no_mangle]
pub extern "C" fn get_account(index: usize, ptr: *mut u8, size: usize) {
    let runtime = mw_rt::runtime::Runtime::new();

    runtime.spawn(async move {
        let s = unsafe { slice::from_raw_parts(ptr, size) };
        let hex_str = hex::encode(s.to_vec());

        let sql = alloc::format!(r#"select * from keystore where account = "{}""#, hex_str);

        let v = mw_std::sql::sql_execute(sql.as_str(), 1).await;

        if v.len() == 0 {
            return;
        }

        let keystore_result =
            quick_protobuf::deserialize_from_slice::<proto::keystore::Keypair>(v.as_slice());

        if keystore_result.as_ref().err().is_some() {
            return;
        }

        let keystore = keystore_result.unwrap();
        let mut keypair_display = proto::keystore::KeypairDisplay::default();
        keypair_display.account = keystore.account;
        keypair_display.cert = keystore.cert;
        keypair_display.public_key = keystore.public_key;
        keypair_display.ty = keystore.ty;

        let mut out: Vec<u8> = Vec::new();
        let serialize_result =
            quick_protobuf::serialize_into_slice(&keypair_display, out.as_mut_slice());

        if serialize_result.as_ref().is_err() {
            return;
        }

        mw_std::notify::notify_ptr_size(index, &out);
    });
}

#[no_mangle]
pub extern "C" fn import_account(index: usize, ptr: *mut u8, size: usize) {
    let runtime = mw_rt::runtime::Runtime::new();

    runtime.spawn(async move {
        let s = unsafe { slice::from_raw_parts(ptr, size) };

        let deserialize_result =
            quick_protobuf::deserialize_from_slice::<proto::keystore::Keypair>(s);

        if deserialize_result.as_ref().is_err() {
            return;
        }

        let keypair = deserialize_result.unwrap();

        let hex_account = hex::encode(keypair.account);
        let hex_secret_key = hex::encode(keypair.secret_key);
        let hex_public_key = hex::encode(keypair.public_key);
        let hex_seed = hex::encode(keypair.seed);
        let hex_encrypt_code = hex::encode(keypair.encrypt_code);
        let hex_nonce = hex::encode(keypair.nonce);
        let public_encrypt_type = "SM2";
        let secret_encrypt_type = "ED25519";
        let timestamp = mw_std::time::get_timestamp();
        let cert = "";

        let sql = alloc::format!(
            r#"
        inser into keystore values(
            "{}","{}","{}","{}","{}","{}","{}","{}",{},"{}"
        )
        "#,
            hex_account,
            hex_seed,
            hex_encrypt_code,
            public_encrypt_type,
            secret_encrypt_type,
            hex_public_key,
            hex_secret_key,
            cert,
            timestamp,
            hex_nonce,
        );

        let mut v = mw_std::sql::sql_execute(sql.as_str(), 0).await;

        let str = unsafe { String::from_raw_parts(v.as_mut_ptr(), v.len(), v.len()) };

        match str.as_str() {
            "ok" => {
                mw_std::notify::notify_number(index, 0);
            }
            "fail" => {
                mw_std::notify::notify_number(index, 1);
            }
            _ => {}
        };
    });
}

#[no_mangle]
pub extern "C" fn export_accounts(index: usize, ptr: *mut u8, size: usize) {
    let runtime = mw_rt::runtime::Runtime::new();
    runtime.spawn(async move {
        let s = unsafe { slice::from_raw_parts(ptr, size) };
        let hex_account = hex::encode(s);

        let sql = alloc::format!(
            r#"select * from keystore where account = "{}""#,
            hex_account
        );
        let v = mw_std::sql::sql_execute(sql.as_str(), 1).await;

        let deserialize_result =
            quick_protobuf::deserialize_from_slice::<proto::keystore::Keystore>(v.as_slice());
        if deserialize_result.as_ref().is_err() {
            return;
        }

        let keystore = deserialize_result.unwrap();

        let mut keypair = proto::keystore::Keypair::default();

        keypair.account = keystore.account;
        keypair.cert = keystore.cert;
        keypair.public_key = keystore.public_key;
        keypair.ty = keystore.secret_encrypt_type;
        keypair.seed = keystore.seed;
        keypair.secret_key = keystore.secret_key;

        let mut out: Vec<u8> = Vec::new();
        let serialize_result = quick_protobuf::serialize_into_slice(&keypair, out.as_mut_slice());
        if serialize_result.as_ref().is_err() {
            return;
        };
        mw_std::notify::notify_ptr_size(index, &out);
    });
}

//TODO impl new_account func
#[no_mangle]
pub extern "C" fn new_account(index: usize, ptr: *mut u8, size: usize) {
    let runtime = mw_rt::runtime::Runtime::new();
    runtime.spawn(async move {
        let secret = crate::cypher::ed_25519::gen_secret().await;
        // let encrypt_code = unsafe { slice::from_raw_parts(ptr, size) };

        let s = unsafe { slice::from_raw_parts(ptr, size) };

        let deserialize_result =
            quick_protobuf::deserialize_from_slice::<proto::keystore::AccountMsg>(s);

        if deserialize_result.as_ref().is_err() {
            return;
        }

        let account_msg = deserialize_result.unwrap();

        let encrypt_code = account_msg.encrypt_code.as_ref();

        let timestamp = mw_std::time::get_timestamp();
        let nonce = mw_std::rand::gen_rand32().await;

        let nonce = nonce.as_slice();

        let secret_key = secret.secret_key.unwrap();

        let public_key = secret.public_key.unwrap();

        let seed = secret.seed.unwrap();

        let secret_key_en =
            cypher::xchacha20::xchacha20_encryption(encrypt_code, nonce, secret_key.to_vec());

        let public_key_en =
            cypher::xchacha20::xchacha20_encryption(encrypt_code, nonce, public_key.to_vec());

        let seed_en = cypher::xchacha20::xchacha20_encryption(encrypt_code, nonce, seed.to_vec());

        let encrypt_code_en =
            cypher::xchacha20::xchacha20_encryption(encrypt_code, nonce, encrypt_code.to_vec());

        let hex_account = hex::encode(account_msg.account.as_ref());
        let hex_seed = hex::encode(seed_en.as_slice());
        let hex_encrypt_code = hex::encode(encrypt_code_en.as_slice());
        let public_encrypt_type = "SM2";
        let secret_encrypt_type = "ED25519";
        let hex_public_key = hex::encode(public_key_en.as_slice());
        let hex_secret_key = hex::encode(secret_key_en.as_slice());
        let cert = "";
        let hex_nonce = hex::encode(nonce);

        let sql = alloc::format!(
            r#"
        inser into keystore values(
            "{}","{}","{}","{}","{}","{}","{}","{}",{},"{}"
        )
        "#,
            hex_account,
            hex_seed,
            hex_encrypt_code,
            public_encrypt_type,
            secret_encrypt_type,
            hex_public_key,
            hex_secret_key,
            cert,
            timestamp,
            hex_nonce,
        );
        //加锁
        crate::STATEMAP.insert(account_msg.account.as_ref().to_vec(), 1);

        let mut v = mw_std::sql::sql_execute(sql.as_str(), 0).await;

        let str = unsafe { String::from_raw_parts(v.as_mut_ptr(), v.len(), v.len()) };

        match str.as_str() {
            "ok" => {
                mw_std::notify::notify_number(index, 0);
            }
            "fail" => {
                mw_std::notify::notify_number(index, 1);
            }
            _ => {}
        };
    });
}

#[no_mangle]
pub extern "C" fn sign_message(index: usize, ptr: *mut u8, size: usize) {
    // 1判断是否在库
    // 2判断是否是解锁状态

    let runtime = mw_rt::runtime::Runtime::new();

    runtime.spawn(async move {
        let s = unsafe { slice::from_raw_parts(ptr, size) };
        let deserialize_result =
            quick_protobuf::deserialize_from_slice::<proto::keystore::SignMsg>(s);

        if deserialize_result.as_ref().is_err() {
            return;
        }

        let sign_msg = deserialize_result.unwrap();

        let account_msg_op = sign_msg.account_msg.as_ref();

        if account_msg_op.as_ref().is_none() {
            return;
        }

        let account_msg = account_msg_op.unwrap();

        let account = account_msg.account.as_ref();

        let encrypt_code = account_msg.encrypt_code.as_ref();

        let hex_account = hex::encode(account);

        let sql = alloc::format!(
            r#"select * from keystore where account = "{}""#,
            hex_account
        );
        let v = mw_std::sql::sql_execute(&sql, 1).await;

        if v.len() == 0 {
            return;
        }

        let keystore_result =
            quick_protobuf::deserialize_from_slice::<proto::keystore::Keystore>(v.as_slice());

        if keystore_result.as_ref().err().is_some() {
            return;
        }

        let keystore = keystore_result.unwrap();

        match crate::STATEMAP.get(account.to_vec()) {
            Some(v) => {
                if v == 1 {
                    return;
                }
            }
            None => {
                return;
            }
        };

        let encrypt_code_de = cypher::xchacha20::xchacha20_decryption(
            encrypt_code,
            keystore.nonce.as_ref(),
            keystore.encrypt_code.to_vec(),
        );

        // 判等
        // 1 转string
        // 2 遍历对比

        let hex1 = hex::encode(encrypt_code_de.as_slice());
        let hex2 = hex::encode(encrypt_code);

        if hex1 != hex2 {
            return;
        }

        //解密public_key
        let public_key = cypher::xchacha20::xchacha20_decryption(
            encrypt_code,
            keystore.nonce.as_ref(),
            keystore.public_key.to_vec(),
        );
        //解密secret_key
        let secret_key = cypher::xchacha20::xchacha20_decryption(
            encrypt_code,
            keystore.nonce.as_ref(),
            keystore.secret_key.to_vec(),
        );

        let sign_op =
            crate::cypher::ed_25519::sign(public_key, secret_key, sign_msg.message.as_ref());

        match sign_op {
            Some(v) => mw_std::notify::notify_ptr_size(index, v.as_slice()),
            None => mw_std::notify::notify_ptr_size(index, Vec::new().as_slice()),
        }
    });
}

#[no_mangle]
pub extern "C" fn verify_sign(index: usize, ptr: *mut u8, size: usize) {
    let s = unsafe { slice::from_raw_parts(ptr, size) };
    let deserialize_result =
        quick_protobuf::deserialize_from_slice::<proto::keystore::VerifySign>(s);

    if deserialize_result.as_ref().is_err() {
        return;
    }

    let verify_sign = deserialize_result.unwrap();
    match verify_sign.VerfySign {
        crate::proto::keystore::mod_VerifySign::OneOfVerfySign::AccountVerifySign(avs) => {
            let runtime = mw_rt::runtime::Runtime::new();
            let sign = avs.sign.clone();
            if avs.sign_msg.as_ref().is_none() {
                return;
            }

            let sign_msg = avs.sign_msg.unwrap();

            if sign_msg.account_msg.as_ref().is_none() {
                return;
            }

            let account_msg = sign_msg.account_msg.unwrap();

            let msg = sign_msg.message.clone();

            runtime.spawn(async move {
                let hex_account = hex::encode(account_msg.account.as_ref());

                let encrypt_code = account_msg.encrypt_code.as_ref();

                let sql = alloc::format!(
                    r#"select * from keystore where account = "{}""#,
                    hex_account
                );
                let v = mw_std::sql::sql_execute(sql.as_str(), 1).await;

                if v.len() == 0 {
                    return;
                }

                let keystore_result = quick_protobuf::deserialize_from_slice::<
                    proto::keystore::Keystore,
                >(v.as_slice());

                if keystore_result.as_ref().err().is_some() {
                    return;
                }

                let keystore = keystore_result.unwrap();

                let encrypt_code_de = cypher::xchacha20::xchacha20_decryption(
                    encrypt_code,
                    keystore.nonce.as_ref(),
                    keystore.encrypt_code.to_vec(),
                );

                // 判等
                // 1 转string
                // 2 遍历对比

                let hex1 = hex::encode(encrypt_code_de.as_slice());
                let hex2 = hex::encode(encrypt_code);

                if hex1 != hex2 {
                    return;
                }

                // 解密public
                let public_key = cypher::xchacha20::xchacha20_decryption(
                    encrypt_code,
                    keystore.nonce.as_ref(),
                    keystore.public_key.to_vec(),
                );

                let verify_op = cypher::ed_25519::verify_sign(
                    public_key.as_slice(),
                    sign.as_ref(),
                    msg.as_ref(),
                );

                match verify_op {
                    Some(flag) => {
                        if flag {
                            mw_std::notify::notify_number(index, 0);
                        } else {
                            mw_std::notify::notify_number(index, 1);
                        }
                    }
                    None => {
                        mw_std::notify::notify_number(index, 1);
                    }
                };
            });
        }
        crate::proto::keystore::mod_VerifySign::OneOfVerfySign::PubVerifySign(pvs) => {
            let verify_op = cypher::ed_25519::verify_sign(
                pvs.public_key.as_ref(),
                pvs.sign.as_ref(),
                pvs.message.as_ref(),
            );
            match verify_op {
                Some(flag) => {
                    if flag {
                        mw_std::notify::notify_number(index, 0);
                    } else {
                        mw_std::notify::notify_number(index, 1);
                    }
                }
                None => {
                    mw_std::notify::notify_number(index, 1);
                }
            };
        }
        crate::proto::keystore::mod_VerifySign::OneOfVerfySign::None => {
            return;
        }
    }
}

#[no_mangle]
pub extern "C" fn lock_account(index: usize, ptr: *mut u8, size: usize) {
    // let hex_str = hex::encode(s.to_vec());
    let s = unsafe { slice::from_raw_parts(ptr, size) };
    let deserialize_result =
        quick_protobuf::deserialize_from_slice::<proto::keystore::AccountMsg>(s);

    if deserialize_result.as_ref().is_err() {
        return;
    }

    let account_msg = deserialize_result.unwrap();

    //检查是否已经是加锁的状态
    match crate::STATEMAP.get(account_msg.account.as_ref().to_vec()) {
        Some(v) => {
            if v == 1 {
                return;
            }
        }
        None => {
            return;
        }
    };

    let runtime = mw_rt::runtime::Runtime::new();
    runtime.spawn(async move {
        let hex_account = hex::encode(account_msg.account.as_ref());

        let encrypt_code = account_msg.encrypt_code.as_ref();

        let sql = alloc::format!(
            r#"select * from keystore where account = "{}""#,
            hex_account
        );
        let v = mw_std::sql::sql_execute(sql.as_str(), 1).await;

        if v.len() == 0 {
            return;
        }

        let keystore_result =
            quick_protobuf::deserialize_from_slice::<proto::keystore::Keystore>(v.as_slice());

        if keystore_result.as_ref().err().is_some() {
            return;
        }

        let keystore = keystore_result.unwrap();

        let encrypt_code_de = cypher::xchacha20::xchacha20_decryption(
            encrypt_code,
            keystore.nonce.as_ref(),
            keystore.encrypt_code.to_vec(),
        );

        // 判等
        // 1 转string
        // 2 遍历对比

        let hex1 = hex::encode(encrypt_code_de.as_slice());
        let hex2 = hex::encode(encrypt_code);

        if hex1 == hex2 {
            //解锁
            crate::STATEMAP.insert(account_msg.account.as_ref().to_vec(), 1);
            mw_std::notify::notify_number(index, 0);
        } else {
            mw_std::notify::notify_number(index, 1);
        }
    });
}

#[no_mangle]
pub extern "C" fn unlock_account(index: usize, ptr: *mut u8, size: usize) {
    let s = unsafe { slice::from_raw_parts(ptr, size) };
    let deserialize_result =
        quick_protobuf::deserialize_from_slice::<proto::keystore::AccountMsg>(s);

    if deserialize_result.as_ref().is_err() {
        return;
    }
    let account_msg = deserialize_result.unwrap();
    //检查是否已经是解锁的状态
    match crate::STATEMAP.get(account_msg.account.as_ref().to_vec()) {
        Some(v) => {
            if v == 0 {
                return;
            }
        }
        None => {
            return;
        }
    };
    let runtime = mw_rt::runtime::Runtime::new();
    runtime.spawn(async move {
        // let hex_str = hex::encode(s.to_vec());

        let hex_account = hex::encode(account_msg.account.as_ref());

        let encrypt_code = account_msg.encrypt_code.as_ref();

        let sql = alloc::format!(
            r#"select * from keystore where account = "{}""#,
            hex_account
        );
        let v = mw_std::sql::sql_execute(sql.as_str(), 1).await;

        if v.len() == 0 {
            return;
        }

        let keystore_result =
            quick_protobuf::deserialize_from_slice::<proto::keystore::Keystore>(v.as_slice());

        if keystore_result.as_ref().err().is_some() {
            return;
        }

        let keystore = keystore_result.unwrap();

        let encrypt_code_de = cypher::xchacha20::xchacha20_decryption(
            encrypt_code,
            keystore.nonce.as_ref(),
            keystore.encrypt_code.to_vec(),
        );

        // 判等
        // 1 转string
        // 2 遍历对比

        let hex1 = hex::encode(encrypt_code_de.as_slice());
        let hex2 = hex::encode(encrypt_code);

        if hex1 == hex2 {
            //解锁
            crate::STATEMAP.insert(account_msg.account.as_ref().to_vec(), 0);
            mw_std::notify::notify_number(index, 0);
        } else {
            mw_std::notify::notify_number(index, 1);
        }
    });
}
