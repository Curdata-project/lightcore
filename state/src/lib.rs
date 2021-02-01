#![no_std]
#![feature(default_alloc_error_handler)]
extern crate alloc;
// extern crate mw_rt;

use alloc::boxed::Box;
use alloc::{string::String, string::ToString, vec, vec::Vec};
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
        mw_std::debug::println("init state start");
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
                                    mw_std::notify::notify_number(0, 0);

                                }
                                "fail" => {
                                    let pair =
                                        Err::InitErrors("init state db fail".to_string()).get();
                                    panic!(pair.1.as_str());
                                }
                                _ => {
                                    let pair = Err::InitErrors(
                                        "init state db fail,execute sql result unknown".to_string(),
                                    )
                                    .get();
                                    panic!(pair.1.as_str());
                                }
                            },
                            Err(err) => {
                                let pair = Err::FromUtf8Error(err).get();
                                panic!(pair.1.as_str());
                            }
                        }
                    }
                    Err(err) => {
                        let pair = Err::ProtoErrors(err).get();
                        panic!(pair.1.as_str());
                    }
                }
            } else {
                mw_std::notify::notify_number(0, 0);
            }
        });
        mw_std::debug::println("init state end");
        State {}
    }

    async fn init(&mut self) {}
}

#[mw_rt::actor::expose]
impl State {
    #[mw_rt::actor::method]
    pub async fn add_state(&mut self, bytes: &[u8]) -> i32 {
        mw_std::debug::println("state add start");
        let deserialize_result =
            quick_protobuf::deserialize_from_slice::<proto::state::State>(bytes);

        if deserialize_result.as_ref().is_err() {
            let e = Err::ProtoErrors(deserialize_result.unwrap_err());
            let pair = e.get();
            return pair.0 as i32;
        }

        let state = deserialize_result.unwrap();

        let state_id = hash_utils::gen_hash_32_id(bytes);

        let mut sql = proto::common::Sql::default();

        sql.sql = alloc::format!(
            "insert into state (id,state,owner,lock,valid,size,is_valid) values (?,?,?,?,?,?,?)"
        )
        .into();
        sql.params.push({
            let mut param = proto::common::Param::default();
            param.tp = "bytes".into();
            param.data = proto::common::mod_Param::OneOfdata::buffer(state_id.into());
            param
        });
        sql.params.push({
            let mut param = proto::common::Param::default();
            param.tp = "bytes".into();
            param.data = proto::common::mod_Param::OneOfdata::buffer(bytes.into());
            param
        });
        sql.params.push({
            let mut param = proto::common::Param::default();
            param.tp = "bytes".into();
            param.data = proto::common::mod_Param::OneOfdata::buffer(state.owner.into());
            param
        });
        sql.params.push({
            let mut param = proto::common::Param::default();
            param.tp = "bytes".into();
            param.data = proto::common::mod_Param::OneOfdata::buffer(state.lock.into());
            param
        });
        sql.params.push({
            let mut param = proto::common::Param::default();
            param.tp = "bytes".into();
            param.data = proto::common::mod_Param::OneOfdata::buffer(state.valid.into());
            param
        });
        sql.params.push({
            let mut param = proto::common::Param::default();
            param.tp = "number".into();
            param.data = proto::common::mod_Param::OneOfdata::number(state.size);
            param
        });
        sql.params.push({
            let mut param = proto::common::Param::default();
            param.tp = "number".into();
            param.data = proto::common::mod_Param::OneOfdata::number(0 as u64);
            param
        });
        

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
                "ok" => {
                    mw_std::debug::println("state add end");
                    0
                },
                "fail" => {
                    let pair = Err::SqlExecture("sql execute fail from insert into state").get();
                    mw_std::debug::println(pair.1.as_str());
                    pair.0 as i32
                },
                _ => {
                    let pair = Err::Null("sql execute unknown result from insert into state").get();
                    mw_std::debug::println(pair.1.as_str());
                    pair.1 as i32
                },
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
        sql.params.push({
            let mut param = proto::common::Param::default();
            param.tp = "bytes".into();
            param.data = proto::common::mod_Param::OneOfdata::buffer(bytes.into());
            param
        });

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
        sql.params.push({
            let mut param = proto::common::Param::default();
            param.tp = "bytes".into();
            param.data = proto::common::mod_Param::OneOfdata::buffer(bytes.into());
            param
        });
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
