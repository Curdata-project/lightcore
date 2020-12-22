//! RPC interfce
use alloc::slice;
use alloc::vec::Vec;

use crate::proto;

extern "C" {
    fn _callback_index_ptr_size(index: usize, ptr: *const u8, size: usize);
    // fn _callback_index_args()
}

/// 分页获取list
//TODO 那此处的参数应该要有个callback，最后我在把这个callback和数据都传回去
#[no_mangle]
pub extern "C" fn list_accounts(page: usize, item: usize, _order: usize, index: usize) {
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

        if keystore_list_result.as_ref().err().is_some() {
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
        unsafe {
            _callback_index_ptr_size(
                index,
                keystore_display_list_bytes.as_ptr(),
                keystore_display_list_bytes.len(),
            );
        };
    });
}

/// 根据account获取信息
#[no_mangle]
pub extern "C" fn get_account(ptr: *mut u8, size: usize, index: usize) {
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
        keypair_display.type_pb = keystore.type_pb;

        let mut out: Vec<u8> = Vec::new();
        let serialize_result =
            quick_protobuf::serialize_into_slice(&keypair_display, out.as_mut_slice());

        if serialize_result.as_ref().err().is_some() {
            return;
        }

        unsafe {
            _callback_index_ptr_size(index, out.as_ptr(), out.len());
        }
    });
}

#[no_mangle]
pub extern "C" fn new_account(_ptr: *mut u8, _size: usize, _index: usize) {

}
