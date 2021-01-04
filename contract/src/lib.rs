#![no_std]
#![feature(default_alloc_error_handler)]
extern crate alloc;
use core::cell::RefCell;
use alloc::boxed::Box;
use alloc::borrow::Cow;
use alloc::borrow::ToOwned;

use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec::Vec;
use mw_rt::actor::Actor;

mod proto;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// #[macro_use]
// extern crate lazy_static;
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

    pub fn list(&self) -> Vec<i32>{
        let map = self.map.borrow_mut();
        let mut v:Vec<i32> = Vec::new();
        for entry in map.iter(){
            v.push(*entry.0);
        }

        v
    }
}

#[mw_rt::actor::actor]
pub struct Contract{}

#[async_trait::async_trait]
impl Actor for Contract{
    fn new() -> Self {
        Contract{}
    }

    async fn init(&mut self) {}
}


#[mw_rt::actor::expose]
impl Contract{

    #[mw_rt::actor::method]
    pub async fn load_contract(&mut self,bytes:&[u8]){
        let instance = mw_std::loader::loader(bytes).await;
        if instance.handle.is_none(){
            return;
        }
        LOADHANDLEMAP.insert(instance.handle.unwrap(),instance);
    }

    pub async fn get_contract(&mut self, id:i32)->&[u8]{
        let instance_op:Option<mw_std::loader::Instance> = LOADHANDLEMAP.get(id);
        if instance_op.is_none(){
            return &[0u8;0];
        }

        let instance = instance_op.unwrap();
        let handle = instance.handle.unwrap().to_be_bytes();//小端

        let mut contract_id = proto::contract::ContractRef::default();
        contract_id.contract_id = Cow::Borrowed(&handle);
        let mut out:Vec<u8> = Vec::new();

        let serialize_result = quick_protobuf::serialize_into_slice(&contract_id, out.as_mut_slice());
        if serialize_result.as_ref().is_err(){
            return &[0u8;0];
        }
        let s =  out.as_slice();
        &[0u8;0]
    }

    pub async fn list_contract(&mut self)->&[u8]{
        let v:Vec<i32> = LOADHANDLEMAP.list();

        let mut contract_list = proto::contract::ContractList::default();
        let list :Vec<Cow<[u8]>> =v.iter().map(|i|{
            let bytes = i.to_be_bytes().to_vec();
            Cow::Owned(bytes)
        }).collect();

        contract_list.contract_list = list;
        &[0u8;0]
    }


}

// #[mw_rt::async_
