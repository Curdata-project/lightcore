/// async load
#[no_mangle]
pub extern "C" fn load_contract(index: u32, bytes: &[u8]) {
    let runtime = mw_rt::runtime::Runtime::new();
    runtime.spawn(async move {
        let result = mw_std::contract::loda(bytes).await;
        mw_std::notify::notify_number(index, result);
    });
}

// sync load
#[no_mangle]
pub extern "C" fn do_load_contract(bytes: &[u8]) -> u32 {
    mw_std::contract::do_load(bytes)
}

#[no_mangle]
pub extern "C" fn get_contract(contract_id: u32) -> u32 {
    mw_std::contract::get_by_id(contract_id)
}

/// async
#[no_mangle]
pub extern "C" fn list_contract(index: u32) {
    let runtime = mw_rt::runtime::Runtime::new();
    runtime.spawn(async move {
        let result = mw_std::contract::list().await;
        mw_std::notify::notify_ptr_size(index, result.as_slice());
    });
}
