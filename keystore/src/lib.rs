#![no_std]
#![feature(default_alloc_error_handler)]
extern crate alloc;

use alloc::string::String;

mod interface;
mod proto;
mod sql;
mod test;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[mw_rt::async_main]
async fn main() {
    // 调用js提供的查表是否存在的方法
    // TODO

    // 如果表存在不做操作，不存在新建
    // TODO

    let v = mw_std::sql::sql_execute(sql::CREATE_KEYSTORE_TABLE, 0).await;
    let str_result = String::from_utf8(v);

    // if str_result.as_ref().err().is_some(){
    //     return;
    // }

    match str_result {
        Ok(str) => {
            mw_std::debug::println(str.as_str());
        }
        Err(e) => {
            mw_std::debug::println(&alloc::format!("{}", e));
        }
    };
}
