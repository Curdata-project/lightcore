use alloc::{borrow::Cow, rc::Rc, string::String, vec, vec::Vec};
use core::ffi::c_void;
use core::pin::Pin;
use core::task::{Context, Poll, Waker};
use core::{cell::RefCell, future::Future};

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

fn rpc_contract_run_contract_fun<F>(handle: i32, bytes: &[u8], mut f: F)
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

fn rpc_contract_load_contract_fun<F>(bytes: &[u8], mut f: F)
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
