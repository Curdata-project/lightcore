#![no_std]
#![feature(default_alloc_error_handler)]
extern crate alloc;
// extern crate mw_rt;

use alloc::boxed::Box;
use alloc::{string::String, vec, vec::Vec};
use common::{err::Err, hash_utils, proto_utils};
use core::cell::RefCell;
use mw_rt::actor::Actor;

mod call_package;
mod proto;
mod sql;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[mw_rt::actor::actor]
pub struct State {}

#[async_trait::async_trait]
impl Actor for State {
    fn new() -> Self {
        let runtime = mw_rt::runtime::Runtime::new();
        runtime.spawn(async move {
            let result = mw_std::sql::sql_table_exist("state".as_bytes()).await;

            // exist=0
            if result != 0 {
                let mut sql = proto::common::Sql::default();
                sql.sql = sql::CREATE_STATE_TABLE.into();
                match proto_utils::qb_serialize(&sql) {
                    Ok(v) => {
                        let result = mw_std::sql::sql_execute(v.as_slice(), 0).await;
                        match String::from_utf8(result) {
                            Ok(str) => match str.as_str() {
                                "ok" => {
                                    mw_std::debug::println("init state db success");
                                }
                                "fail" => {
                                    panic!("init state db fail");
                                }
                                _ => {
                                    mw_std::debug::println(&alloc::format!("sql return:{}", str));
                                    panic!("init state db fail");
                                }
                            },
                            Err(err) => {
                                let pair = Err::FromUtf8Error(err).get();
                                mw_std::debug::println(pair.1.as_str());
                            }
                        }
                    }
                    Err(err) => {
                        let pair = Err::ProtoErrors(err).get();
                        mw_std::debug::println(pair.1.as_str());
                    }
                }
            }
        });
        State {}
    }

    async fn init(&mut self) {}
}

#[mw_rt::actor::expose]
impl State {
    #[mw_rt::actor::method]
    pub async fn add_state(&mut self, bytes: &[u8]) -> i32 {
        let deserialize_result =
            quick_protobuf::deserialize_from_slice::<proto::state::State>(bytes);

        if deserialize_result.as_ref().is_err() {
            let e = Err::ProtoErrors(deserialize_result.unwrap_err());
            let pair = e.get();
            return pair.0 as i32;
        }

        let state = deserialize_result.unwrap();

        //先验证
        let valid = state.valid.to_vec();
        let handle = call_package::contract::load_contract(valid.as_slice()).await;
        let r = call_package::contract::run_contract(handle, bytes).await;

        if r != 0 {
            let e = Err::VaildFail(r);
            let pair = e.get();
            return pair.0 as i32;
        }

        let state_id = hash_utils::gen_hash_32_id(bytes);

        let mut sql = proto::common::Sql::default();

        sql.sql = alloc::format!(
            "insert into state (id,state,owner,lock,valid,size,is_valid) values (?,?,?,?,?,{},{})",
            state.size,
            0
        )
        .into();

        sql.params.push(state_id.into());
        sql.params.push(bytes.into());
        sql.params.push(state.owner);
        sql.params.push(state.lock);
        sql.params.push(state.valid);

        let result = proto_utils::qb_serialize(&sql);

        if result.is_err() {
            let e = Err::ProtoErrors(result.unwrap_err());
            let pair = e.get();
            return pair.0 as i32;
        }

        let bytes = result.unwrap();
        let v = mw_std::sql::sql_execute(bytes.as_slice(), 0).await;

        let result = String::from_utf8(v);
        return match result {
            Ok(value) => match value.as_str() {
                "ok" => 0,
                "fail" => 1,
                _ => 1,
            },
            Err(err) => {
                let e = Err::FromUtf8Error(err);
                let pair = e.get();
                pair.0 as i32
            }
        };
    }

    #[mw_rt::actor::method]
    pub async fn delete_state(&mut self, bytes: &[u8]) -> i32 {
        let mut sql = proto::common::Sql::default();
        sql.sql = alloc::format!("delete from state where id = ?").into();
        sql.params.push(bytes.into());

        let result = common::proto_utils::qb_serialize(&sql);
        if result.is_err() {
            let e = Err::ProtoErrors(result.unwrap_err());
            let pair = e.get();
            return pair.0 as i32;
        }

        let bytes = result.unwrap();
        let v = mw_std::sql::sql_execute(bytes.as_slice(), 0).await;
        let result = String::from_utf8(v);
        return match result {
            Ok(value) => match value.as_str() {
                "ok" => 0,
                "fail" => 1,
                _ => 1,
            },
            Err(err) => {
                let e = Err::FromUtf8Error(err);
                let pair = e.get();
                pair.0 as i32
            }
        };
    }

    //external interface
    #[mw_rt::actor::method]
    pub async fn list_state(&mut self, page: usize, item: usize, _order: usize) -> Vec<u8> {
        let mut sql = proto::common::Sql::default();
        sql.sql = alloc::format!(
            "select * from state limit {} offset {}",
            item,
            item * (page - 1)
        )
        .into();

        let result = common::proto_utils::qb_serialize(&sql);
        if result.is_err() {
            let e = Err::ProtoErrors(result.unwrap_err());
            let _pair = e.get();
            return vec![];
        }
        let bytes = result.unwrap();
        let v = mw_std::sql::sql_execute(bytes.as_slice(), 1).await;
        v
    }

    #[mw_rt::actor::method]
    pub async fn get_state(&mut self, bytes: &[u8]) -> Vec<u8> {
        let mut sql = proto::common::Sql::default();
        sql.sql = alloc::format!("select * from state where id = ?").into();
        sql.params.push(bytes.into());
        let result = common::proto_utils::qb_serialize(&sql);
        if result.is_err() {
            let e = Err::ProtoErrors(result.unwrap_err());
            let _pair = e.get();
            return vec![];
        }
        let bytes = result.unwrap();
        let v = mw_std::sql::sql_execute(bytes.as_slice(), 1).await;
        v
    }

    #[mw_rt::actor::method]
    pub async fn valid_signed_state(&mut self, bytes: &[u8]) -> i32 {
        let result = quick_protobuf::deserialize_from_slice::<proto::state::State>(bytes);
        if result.is_err() {
            let e = Err::ProtoErrors(result.unwrap_err());
            let pair = e.get();
            return pair.0 as i32;
        }
        let state = result.unwrap();
        let valid = state.valid.to_vec();
        let handle = call_package::contract::load_contract(valid.as_slice()).await;
        let r = call_package::contract::run_contract(handle, bytes).await;
        r
    }
}
