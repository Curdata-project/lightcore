#![no_std]
#![feature(default_alloc_error_handler)]
extern crate alloc;

use alloc::{boxed::Box, collections::BTreeMap};
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
pub struct Transactione {}

#[async_trait::async_trait]
impl Actor for Transactione {
    fn new() -> Self {
        let table_name = "transaction".as_bytes();
        let runtime = mw_rt::runtime::Runtime::new();
        runtime.spawn(async move {
            let result = mw_std::sql::sql_table_exist(table_name).await;

            if result != 0 {
                let mut sql = proto::common::Sql::default();
                sql.sql = sql::CREATE_TRANSACTION_TABLE.into();
                match proto_utils::qb_serialize(&sql) {
                    Ok(v) => {
                        let result = mw_std::sql::sql_execute(v.as_slice(), 0).await;
                        match String::from_utf8(result) {
                            Ok(str) => match str.as_str() {
                                "ok" => {
                                    mw_std::debug::println("init transaction db success");
                                    mw_std::notify::notify_number(0, 0);
                                }
                                "fail" => {
                                    let pair =
                                        Err::InitErrors("init transaction db fail".to_string())
                                            .get();
                                    panic!(pair.1.as_str());
                                }
                                _ => {
                                    let pair = Err::InitErrors(
                                        "init transaction db fail,execute sql result unknown"
                                            .to_string(),
                                    )
                                    .get();
                                    panic!(pair.1.as_str());
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
            } else {
                mw_std::notify::notify_number(0, 0);
            }
        });

        Transactione {}
    }

    async fn init(&mut self) {}
}

#[mw_rt::actor::expose]
impl Transactione {
    #[mw_rt::actor::method]
    pub async fn list_txs(&mut self) -> Vec<u8> {
        let mut sql = proto::common::Sql::default();
        sql.sql = "select * from transaction".into();
        let result = proto_utils::qb_serialize(&sql);
        if result.is_err() {
            let e = Err::ProtoErrors(result.unwrap_err());
            let pair = e.get();
            mw_std::debug::println(&alloc::format!("{}", pair.1));
            return vec![];
        }
        let sql = result.unwrap();
        let result = mw_std::sql::sql_execute(sql.as_slice(), 1).await;
        result
    }

    #[mw_rt::actor::method]
    pub async fn get_tx(&mut self, id: &[u8]) -> Vec<u8> {
        let mut sql = proto::common::Sql::default();
        sql.sql = "select * from where transaction where id = ?".into();
        sql.params.push(id.into());
        let result = proto_utils::qb_serialize(&sql);
        if result.is_err() {
            let e = Err::ProtoErrors(result.unwrap_err());
            let pair = e.get();
            mw_std::debug::println(&alloc::format!("{}", pair.1));
            return vec![];
        }
        let sql = result.unwrap();
        let result = mw_std::sql::sql_execute(sql.as_slice(), 1).await;
        result
    }

    #[mw_rt::actor::method]
    pub async fn send_tx(&mut self, bare_tx: &[u8]) -> i32 {
        let result =
            quick_protobuf::deserialize_from_slice::<proto::transaction::BareTransaction>(bare_tx);
        if result.is_err() {
            let pair = Err::ProtoErrors(result.unwrap_err()).get();
            return pair.0 as i32;
        }

        let bare_transaction = result.unwrap();
        let mut state_id_list = bare_transaction.inputs;
        let args = bare_transaction.arguments;
        let mut count = 0;
        let mut tx = proto::transaction::Transaction::default();
        let mut input_v: Vec<Vec<u8>> = Vec::new();

        for item in state_id_list.iter_mut() {
            // 查库
            let mut sql = proto::common::Sql::default();
            sql.sql = sql::QUERY_STATE_BY_ID.into();
            sql.params.push(item.clone());
            let result = proto_utils::qb_serialize(&sql);
            if result.is_err() {
                let pair = Err::ProtoErrors(result.unwrap_err()).get();
                return pair.0 as i32;
            };
            let serialize = result.unwrap();
            let v = mw_std::sql::sql_execute(serialize.as_slice(), 1).await;
            if v.len() <= 0 {
                let pair = Err::Null("query is null".to_string()).get();
                return pair.0 as i32;
            };
            // deserialize

            let result =
                quick_protobuf::deserialize_from_slice::<proto::state::State>(v.as_slice());
            if result.is_err() {
                let pair = Err::ProtoErrors(result.unwrap_err()).get();
                return pair.0 as i32;
            };

            let state = result.unwrap();
            let op = args.get(count);
            count = count + 1;
            if op.is_none() {
                let pair = Err::Null("get arg null".to_string()).get();
                return pair.0 as i32;
            }
            let arg = op.unwrap().to_vec();

            let mut signed_state = proto::state::SignedState::default();
            //TODO sig state
            // signed_state.signature =
            // signed_state.witness =
            signed_state.state = Some(state);
            signed_state.id = item.clone();
            let mut tx_input = proto::transaction::TranscationInput::default();
            tx_input.state = Some(signed_state);
            tx_input.arguments = arg.into();
            let result = proto_utils::qb_serialize(&tx_input);
            if result.is_err() {
                let pair = Err::ProtoErrors(result.unwrap_err()).get();
                return pair.0 as i32;
            }
            input_v.push(result.unwrap());
        }

        let input_v: Vec<proto::transaction::TranscationInput> = input_v
            .iter_mut()
            .map(|v| {
                let result = quick_protobuf::deserialize_from_slice::<
                    proto::transaction::TranscationInput,
                >(v.as_slice());
                if result.is_err() {
                    return proto::transaction::TranscationInput::default();
                };
                result.unwrap()
            })
            .collect();

        tx.inputs = input_v;
        tx.outputs = bare_transaction.outputs;
        let result = proto_utils::qb_serialize(&tx);
        if result.is_err() {
            let pair = Err::ProtoErrors(result.unwrap_err()).get();
            return pair.0 as i32;
        }
        let id = hash_utils::gen_hash_32_id(result.unwrap().as_slice());
        tx.id = id.into();
        let result = proto_utils::qb_serialize(&tx);
        if result.is_err() {
            let pair = Err::ProtoErrors(result.unwrap_err()).get();
            return pair.0 as i32;
        }
        self.send_raw_tx(result.unwrap().as_slice()).await
    }

    #[mw_rt::actor::method]
    pub async fn send_raw_tx(&mut self, tx: &[u8]) -> i32 {
        let result = quick_protobuf::deserialize_from_slice::<proto::transaction::Transaction>(tx);
        if result.is_err() {
            let pair = Err::ProtoErrors(result.unwrap_err()).get();
            return pair.0 as i32;
        }
        let transaction = result.unwrap();
        let mut inputs = transaction.inputs;
        let mut outputs = transaction.outputs;

        let mut input_param = proto::common::BytesList::default();
        let mut output_param = proto::common::BytesList::default();

        let mut inpus_size = 0;
        let mut outputs_size = 0;
        // 验证inputs是否可用,从链上获取可用state列表然后对比
        //TODO 去server获取
        let available_map: BTreeMap<Vec<u8>, bool> = BTreeMap::new();

        //获取inputs的 总长度和查看是否都可用
        for input in inputs.iter() {
            if input.state.is_none() {
                let pair = Err::Null("input is null".to_string()).get();
                return pair.0 as i32;
            }

            match input.state.clone() {
                Some(signed_state) => {
                    match signed_state.state.clone() {
                        Some(state) => {
                            inpus_size = inpus_size + state.size;
                            //判断map中是否有这个可用state
                            match available_map.get(&signed_state.id.to_vec()) {
                                Some(_b) => {}
                                None => {
                                    let pair = Err::Null("the state not".to_string()).get();
                                    return pair.0 as i32;
                                }
                            };
                        }
                        None => {
                            let pair = Err::Null("state is null".to_string()).get();
                            return pair.0 as i32;
                        }
                    };
                }
                None => {
                    let pair = Err::Null("signed state is null".to_string()).get();
                    return pair.0 as i32;
                }
            }
        }

        for output in outputs.iter() {
            outputs_size = outputs_size + output.size;
        }

        if outputs_size != inpus_size {
            return -2;
        }

        //消库 state
        for input in inputs.iter_mut() {
            if input.state.is_none() {
                return -1;
            }
            match input.state.clone() {
                Some(signed_state) => {
                    match signed_state.state.clone() {
                        Some(state) => {
                            match proto_utils::qb_serialize(input) {
                                Ok(v) => {
                                    input_param.params.push(v.into());
                                }
                                Err(err) => {
                                    let pair = Err::ProtoErrors(err).get();
                                    return pair.0 as i32;
                                }
                            }

                            let lock_contract = state.lock.to_vec();
                            // load contract
                            let handle =
                                call_package::contract::load_contract(lock_contract.as_slice())
                                    .await;
                            if handle < 0 {
                                let pair = Err::Null("load contract result fail".to_string()).get();
                                return pair.0 as i32;
                            };
                            // run contract
                            let result = call_package::contract::run_contract(
                                handle,
                                input.arguments.to_vec().as_slice(),
                            )
                            .await;
                            if result != 0 {
                                let pair = Err::UnlockFail(result).get();
                                return pair.0 as i32;
                            };
                        }
                        None => {
                            let pair = Err::Null("state is null".to_string()).get();
                            return pair.0 as i32;
                        }
                    };
                }
                None => {
                    let pair = Err::Null("signed state is null".to_string()).get();
                    return pair.0 as i32;
                }
            }
        }

        //写库 state
        for output in outputs.iter_mut() {
            match proto_utils::qb_serialize(output) {
                Ok(v) => {
                    output_param.params.push(v.clone().into());
                    let result = call_package::state::add_state(v.as_slice()).await;
                    if result != 0 {
                        let pair = Err::CallState("add state fail".to_string()).get();
                        return pair.0 as i32;
                    }
                }
                Err(err) => {
                    let pair = Err::ProtoErrors(err).get();
                    return pair.0 as i32;
                }
            }
        }
        // 写库transaction
        let mut sql = proto::common::Sql::default();
        sql.sql = alloc::format!(
            "insert into transaction(id,inputs,outputs,timestamp) values(?,?,?,{})",
            transaction.timestamp
        )
        .into();
        sql.params.push(transaction.id.into());
        match proto_utils::qb_serialize(&input_param) {
            Ok(v) => {
                sql.params.push(v.into());
            }
            Err(err) => {
                let pair = Err::ProtoErrors(err).get();
                return pair.0 as i32;
            }
        }

        match proto_utils::qb_serialize(&output_param) {
            Ok(v) => {
                sql.params.push(v.into());
            }
            Err(err) => {
                let pair = Err::ProtoErrors(err).get();
                return pair.0 as i32;
            }
        }

        match proto_utils::qb_serialize(&sql) {
            Ok(v) => {
                let result = mw_std::sql::sql_execute(v.as_slice(), 0).await;
                match String::from_utf8(result) {
                    Ok(str) => match str.as_str() {
                        "ok" => {}
                        "fail" => {
                            let pair =
                                Err::SqlExecture("update state set is_vaild to 1 fail".to_string())
                                    .get();
                            return pair.0 as i32;
                        }
                        _ => {
                            let pair = Err::SqlExecture(
                                "update state set is_vaild to 1 unknown error code".to_string(),
                            )
                            .get();
                            return pair.0 as i32;
                        }
                    },
                    Err(err) => {
                        let pair = Err::FromUtf8Error(err).get();
                        return pair.0 as i32;
                    }
                }
            }
            Err(err) => {
                let pair = Err::ProtoErrors(err).get();
                return pair.0 as i32;
            }
        }
        //主链登记一下
        0
    }
}
