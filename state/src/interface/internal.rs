use alloc::{borrow::Cow, slice};

use crate::err::Err;
use crate::proto;
use alloc::string::String;

#[no_mangle]
pub extern "C" fn add_state(ptr: *const u8, size: usize) -> usize {
    let bytes = unsafe { slice::from_raw_parts(ptr, size) };
    let deserialize_result = quick_protobuf::deserialize_from_slice::<proto::state::State>(bytes);

    if deserialize_result.as_ref().is_err() {
        let e = Err::ProtoErrors(deserialize_result.unwrap_err());
        let pair = e.get();
        return pair.0;
    }

    let state = deserialize_result.unwrap();

    let state_id = crate::utils::hash::gen_state_id(bytes);

    let mut sql = proto::common::Sql::default();

    let sql_str = &alloc::format!(
        r#"
        insert into state (id,state,owner,lock,valid,size,is_valid) 
        values ($1,$2,$3,$4,$5,{},{})
    "#,
        state.size,
        0
    );

    sql.sql = Cow::Borrowed(sql_str);

    sql.params.push(Cow::Owned(state_id));
    sql.params.push(Cow::Owned(bytes.to_vec()));
    sql.params.push(state.owner);
    sql.params.push(state.lock);
    sql.params.push(state.valid);

    let result = crate::utils::proto::qb_serialize(&sql);

    if result.is_err() {
        let e = Err::ProtoErrors(result.unwrap_err());
        let pair = e.get();
        return pair.0;
    }

    let bytes = result.unwrap();

    let runtime = mw_rt::runtime::Runtime::new();
    runtime.spawn(async move {
        let v = mw_std::sql::sql_execute(bytes.as_slice(), 0).await;

        let result = String::from_utf8(v);

        if result.is_err() {
            let e = Err::FromUtf8Error(result.unwrap_err());
            let _pair = e.get();
        };
    });

    0
}
