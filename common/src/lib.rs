#![no_std]
#![feature(default_alloc_error_handler)]
extern crate alloc;
// extern crate mw_rt;

mod proto;
mod err;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;