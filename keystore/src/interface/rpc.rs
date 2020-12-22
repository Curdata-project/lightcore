//! RPC interfce
use alloc::string::String;
use alloc::vec::Vec;

use crate::module;
/// 分页获取list
//TODO 那此处的参数应该要有个callback，最后我在把这个callback和数据都传回去
#[no_mangle]
pub extern "C" fn list_accounts(page: usize, item: usize, _order: usize) -> u8 {
    let sql = alloc::format!(
        "select * from ketstore limit {} offset {}",
        item,
        item * (page - 1)
    );
    let runtime = mw_rt::runtime::Runtime::new();
    runtime.spawn(async move {
        let v = mw_std::sql::sql_execute(sql.as_str(), 1).await;
        let str_result = String::from_utf8(v);

        if str_result.as_ref().err().is_some() {
            return;
        }

        let str = str_result.unwrap();

        //将str rpc反序列化
        let keystore_list_result = quick_protobuf::deserialize_from_slice::<
            module::keystore::KeystoreList,
        >(str.as_bytes());

        if keystore_list_result.as_ref().err().is_some() {
            return;
        }

        let keystore_list = keystore_list_result.unwrap();

        //解析
        let mut keystore_display_list = module::keystore::KeypairDisplayList::default();

        for value in keystore_list.keystore_list {
            let mut keystore_display = module::keystore::KeypairDisplay::default();

            keystore_display.account = value.account;
            keystore_display.cert = value.cert;
            keystore_display.public_key = value.public_key;
            keystore_display.type_pb = value.type_pb;

            keystore_display_list
                .keypair_display_list
                .push(keystore_display);
        }

        //序列化
        let mut keystore_display_list_bytes: Vec<u8> = Vec::new();
        let serialize_result = quick_protobuf::serialize_into_slice(
            &keystore_display_list,
            keystore_display_list_bytes.as_mut_slice(),
        );

        if serialize_result.as_ref().err().is_some() {
            return;
        }

        //调用js的callback方法通知回去
    });
    0
}
