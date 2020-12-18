#![no_std]
#![feature(default_alloc_error_handler)]
extern crate alloc;

mod module;
mod test;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[mw_rt::async_main]
async fn main() {

}
