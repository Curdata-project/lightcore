#[marrow::main]
async fn main() {

}

#[no_mangle]
pub extern "C" fn _entry() {
    let runtime = runtime::Runtime::new();

    runtime.spawn(main());
}