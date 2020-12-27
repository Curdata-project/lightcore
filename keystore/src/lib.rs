#![no_std]
#![feature(default_alloc_error_handler)]
extern crate alloc;

use core::cell::RefCell;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;

mod cypher;
mod interface;
mod proto;
mod sql;
mod test;

#[macro_use]
extern crate lazy_static;
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

#[mw_rt::async_main]
async fn main() {
    // 调用js提供的查表是否存在的方法
    let flag = mw_std::sql::sql_table_exist("keystore");

    match flag {
        0 => {}
        1 => {
            let v = mw_std::sql::sql_execute(sql::CREATE_KEYSTORE_TABLE, 0).await;
            let str_result = String::from_utf8(v);
            if str_result.as_ref().is_err() {
                return;
            }

            let str = str_result.unwrap();
            match str.as_str() {
                "ok" => {}
                "fail" => {
                    return;
                }
                _ => {}
            }
        }
        _ => {}
    }

    let v = mw_std::sql::sql_execute(sql::CREATE_KEYSTORE_TABLE, 0).await;
    let str_result = String::from_utf8(v);

    // if str_result.as_ref().err().is_some(){
    //     return;
    // }

    match str_result {
        Ok(str) => {
            mw_std::debug::println(str.as_str());

            STATEMAP.init();
        }
        Err(e) => {
            mw_std::debug::println(&alloc::format!("{}", e));
        }
    };
}
