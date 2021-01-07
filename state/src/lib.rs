#![no_std]
#![feature(default_alloc_error_handler)]
extern crate alloc;
// extern crate mw_rt;

use alloc::boxed::Box;
use alloc::{borrow::Cow, rc::Rc, string::String, vec, vec::Vec};
use common::{err::Err, proto_utils};
use core::ffi::c_void;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};
use core::{cell::RefCell, future::Future};
use mw_rt::actor::Actor;

mod proto;
mod utils;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[mw_rt::actor::actor]
pub struct State {}

#[async_trait::async_trait]
impl Actor for State {
    fn new() -> Self {
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
        let handle = load_contract(valid.as_slice()).await;
        let r = run_contract(handle, bytes).await;

        if r != 0 {
            let e = Err::VaildFail(r);
            let pair = e.get();
            return pair.0 as i32;
        }

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
        let sql_str = alloc::format!(
            r#"
            delete from state where id = $1
        "#
        );

        let mut sql = proto::common::Sql::default();
        sql.sql = Cow::Owned(sql_str);
        sql.params.push(Cow::Owned(bytes.to_vec()));

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
        let sql_str = alloc::format!(
            r#"
            select * from state limit {} offset {}
        "#,
            item,
            item * (page - 1)
        );

        let mut sql = proto::common::Sql::default();
        sql.sql = Cow::Owned(sql_str);

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
        let sql_str = alloc::format!(
            r#"
            select * from state where id = $1
        "#
        );

        let mut sql = proto::common::Sql::default();
        sql.sql = Cow::Owned(sql_str);
        sql.params.push(Cow::Owned(bytes.to_vec()));
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
        let handle = load_contract(valid.as_slice()).await;
        let r = run_contract(handle, bytes).await;
        r
    }
}

#[derive(Debug, Clone)]
pub struct BytesResult {
    inner: Rc<RefCell<BytesInner>>,
}

#[derive(Debug, Clone, Default)]
struct BytesInner {
    ptr: Option<*const u8>,
    size: Option<usize>,
    task: Option<Waker>,
}

#[derive(Debug, Clone)]
pub struct NumberResult {
    inner: Rc<RefCell<NumberInner>>,
}

#[derive(Debug, Clone, Default)]
struct NumberInner {
    result: Option<i32>,
    task: Option<Waker>,
}

impl NumberResult {
    pub fn default() -> Self {
        NumberResult {
            inner: Rc::new(RefCell::new(Default::default())),
        }
    }
}

impl Future for NumberResult {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut inner = self.inner.borrow_mut();

        if inner.result.is_some() {
            let v = inner.result.unwrap();
            return Poll::Ready(v);
        }

        inner.task = Some(cx.waker().clone());
        Poll::Pending
    }
}

pub fn load_contract(bytes: &[u8]) -> NumberResult {
    let result = NumberResult::default();
    let mut inner = result.inner.borrow_mut();

    rpc_contract_load_contract_fun(bytes, |result| {
        inner.result = Some(result);
        let task_op = &inner.task.as_ref();
        if task_op.is_some() {
            task_op.unwrap().wake_by_ref()
        };
    });
    result.clone()
}

pub fn run_contract(handle: i32, bytes: &[u8]) -> NumberResult {
    let result = NumberResult::default();
    let mut inner = result.inner.borrow_mut();

    rpc_contract_run_contract_fun(handle, bytes, |result| {
        inner.result = Some(result);
        let task_op = &inner.task.as_ref();
        if task_op.is_some() {
            task_op.unwrap().wake_by_ref()
        };
    });
    result.clone()
}

pub fn rpc_contract_run_contract_fun<F>(handle: i32, bytes: &[u8], mut f: F)
where
    F: FnMut(i32),
{
    #[link(wasm_import_module = "contract")]
    extern "C" {
        fn rpc_contract_run_contract(
            id: i32,
            ptr: *const u8,
            size: usize,
            cb: unsafe extern "C" fn(*mut c_void, i32),
            user_data: *mut c_void,
        );
    }

    unsafe extern "C" fn hook_number<F>(user_data: *mut c_void, result: i32)
    where
        F: FnMut(i32),
    {
        //这里将闭包的数据指针强转为函数指针，并传入参数
        (*(user_data as *mut F))(result)
    }
    let user_data = &mut f as *mut _ as *mut c_void;

    unsafe {
        rpc_contract_run_contract(
            handle,
            bytes.as_ptr(),
            bytes.len(),
            hook_number::<F>,
            user_data,
        );
    }
}

pub fn rpc_contract_load_contract_fun<F>(bytes: &[u8], mut f: F)
where
    F: FnMut(i32),
{
    #[link(wasm_import_module = "contract")]
    extern "C" {
        fn rpc_contract_load_contract(
            ptr: *const u8,
            size: usize,
            cb: unsafe extern "C" fn(*mut c_void, i32),
            user_data: *mut c_void,
        );
    }

    unsafe extern "C" fn hook_number<F>(user_data: *mut c_void, result: i32)
    where
        F: FnMut(i32),
    {
        //这里将闭包的数据指针强转为函数指针，并传入参数
        (*(user_data as *mut F))(result)
    }

    let user_data = &mut f as *mut _ as *mut c_void;

    unsafe {
        rpc_contract_load_contract(bytes.as_ptr(), bytes.len(), hook_number::<F>, user_data);
    }
}

#[no_mangle]
pub extern "C" fn call_rpc_contract_load_contract(
    result: i32,
    cb: unsafe extern "C" fn(*mut c_void, i32),
    user_data: *mut c_void,
) {
    unsafe { cb(user_data, result) }
}

#[no_mangle]
pub extern "C" fn call_rpc_contract_run_contract(
    ptr: *const u8,
    size: usize,
    cb: unsafe extern "C" fn(*mut c_void, *const u8, usize),
    user_data: *mut c_void,
) {
    unsafe { cb(user_data, ptr, size) }
}
