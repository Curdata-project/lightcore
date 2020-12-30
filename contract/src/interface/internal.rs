//! the internal interface

#[no_mangle]
pub extern "C" fn _run_contract(index: usize, args: &[u8]) {
    let runtime = mw_rt::runtime::Runtime::new();

    runtime.spawn(async move {
        let v = mw_std::contract::run(args).await;
        mw_std::notify::notify_ptr_size(index, v.as_slice());
    });
}
