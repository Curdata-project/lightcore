#![no_std]
#![feature(default_alloc_error_handler)]
extern crate alloc;
use alloc::borrow::Cow;
use alloc::boxed::Box;
use common::{err::Err, proto_utils};
use core::cell::RefCell;

use alloc::collections::BTreeMap;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use mw_rt::actor::Actor;

mod proto;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[macro_use]
lazy_static! {
    pub static ref LOADHANDLEMAP: LoadHandleMap = LoadHandleMap::new();
}

unsafe impl Sync for LoadHandleMap {}

pub struct LoadHandleMap {
    map: RefCell<BTreeMap<i32, mw_std::loader::Instance>>,
}

impl LoadHandleMap {
    fn new() -> Self {
        let map = RefCell::new(BTreeMap::new());

        Self { map }
    }

    pub fn insert(&self, key: i32, instance: mw_std::loader::Instance) {
        self.map.borrow_mut().insert(key, instance);
    }

    pub fn init(&self) {
        self.map.borrow_mut().clear();
    }

    pub fn get(&self, key: i32) -> Option<mw_std::loader::Instance> {
        match self.map.borrow_mut().get(&key) {
            Some(v) => Some(v.clone()),
            None => None,
        }
    }

    pub fn list(&self) -> Vec<i32> {
        let map = self.map.borrow_mut();
        let mut v: Vec<i32> = Vec::new();
        for entry in map.iter() {
            v.push(*entry.0);
        }

        v
    }
}

#[mw_rt::actor::actor]
pub struct Contract {}

#[async_trait::async_trait]
impl Actor for Contract {
    fn new() -> Self {
        LOADHANDLEMAP.init();
        mw_std::notify::notify_number(0, 0);
        Contract {}
    }

    async fn init(&mut self) {}
}

#[mw_rt::actor::expose]
impl Contract {
    #[mw_rt::actor::method]
    pub async fn load_contract(&mut self, bytes: &[u8]) -> i32 {
        // mw_std::debug::println(&alloc::format!("{:?}",bytes));
        let instance = mw_std::loader::loader(bytes).await;
        mw_std::debug::println(&alloc::format!("instance:{:?}",instance));
        if instance.handle.is_none() {
            return -1;
        }
        let result = instance.handle.unwrap();
        LOADHANDLEMAP.insert(instance.handle.unwrap(), instance);
        let i = LOADHANDLEMAP.get(result);
        mw_std::debug::println(&alloc::format!("{:?}",i));
        result
    }

    #[mw_rt::actor::method]
    pub async fn get_contract(&mut self, id: i32) -> Vec<u8> {
        let instance_op: Option<mw_std::loader::Instance> = LOADHANDLEMAP.get(id);
        let mut contract_id = proto::contract::ContractRef::default();
        if instance_op.is_none() {
            let pair = Err::Null("load handler is null".to_string()).get();

            return vec![];
        }

        let instance = instance_op.unwrap();
        let handle = instance.handle.unwrap().to_be_bytes(); //小端

        contract_id.contract_id = handle.to_vec().into();

        let result = proto_utils::qb_serialize(&contract_id);
        if result.is_err() {
            let e = Err::ProtoErrors(result.unwrap_err());
            let pair = e.get();

            let result = proto_utils::qb_serialize(&contract_id);

            return match result {
                Ok(value) => value,
                Err(err) => {
                    let e = Err::ProtoErrors(err);
                    let _pair = e.get();
                    return vec![];
                }
            };
        }

        result.unwrap()
    }

    #[mw_rt::actor::method]
    pub async fn list_contract(&mut self) -> Vec<u8> {
        let v: Vec<i32> = LOADHANDLEMAP.list();

        let mut contract_list = proto::contract::ContractList::default();
        let list: Vec<Cow<[u8]>> = v
            .iter()
            .map(|i| {
                let bytes = i.to_be_bytes().to_vec();
                bytes.into()
            })
            .collect();

        contract_list.contract_list = list;
        let result = proto_utils::qb_serialize(&contract_list);
        return match result {
            Ok(value) => value,
            Err(err) => {
                let e = Err::ProtoErrors(err);
                let _pair = e.get();
                return vec![];
            }
        };
    }
    
    #[mw_rt::actor::method]
    pub async fn run_contract(&mut self, id: i32, bytes: &[u8]) -> i32 {
        let instance_op: Option<mw_std::loader::Instance> = LOADHANDLEMAP.get(id);
        if instance_op.is_none() {
            let e = Err::Null("load handler is null".to_string());
            let pair = e.get();
            return pair.0 as i32;
        }

        let instance = instance_op.unwrap();

        let result = instance.run(bytes);
        result
    }
}
