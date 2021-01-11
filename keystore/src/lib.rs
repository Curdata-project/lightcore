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
                            }
                        }
                        Err(err) => {
                            let pair = Err::ProtoErrors(err).get();
                            mw_std::debug::println(&pair.1);
                        }
                    }
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
    pub async fn list_accounts(&mut self, ind: usize, page: usize, item: usize, order: usize) -> Vec<u8> {
        let mut sql = proto::common::Sql::default();
        sql.sql = Cow::Owned(alloc::format!("select * from keystore limit {} offset {}",item,item * (page - 1)));
        let v = vec![0u8;2];
        sql.params.push(v.into());
        match proto_utils::qb_serialize(&sql) {
            Ok(v) => {

            }
            Err(err) => {}
        }
        vec![]
    }

    #[mw_rt::actor::method]
    pub async fn get_account(&mut self, id: &[u8]) -> Vec<u8> {
        vec![]
    }

    #[mw_rt::actor::method]
    pub async fn import_account(&mut self, bytes: &[u8]) -> i32 {
        0
    }

    #[mw_rt::actor::method]
    pub async fn export_accounts(&mut self, id: &[u8]) -> Vec<u8> {
        vec![]
    }

    #[mw_rt::actor::method]
    pub async fn new_account(&mut self, bytes: &[u8]) -> i32 {
        0
    }

    #[mw_rt::actor::method]
    pub async fn sign_message(&mut self, bytes: &[u8]) -> Vec<u8> {
        vec![]
    }

    #[mw_rt::actor::method]
    pub async fn verify_sign(&mut self, bytes: &[u8]) -> i32 {
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
