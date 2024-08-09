#![no_main]
#![no_std]

// 可以直接使用 cargo build --target wasm32-unknown-unknown --release
#[panic_handler]
fn handle_panic(_: &core::panic::PanicInfo) -> ! {
    unreachable!()
}

#[no_mangle]
pub extern "C" fn add_two(a: i32, b: i32) -> i32 {
    a + b
}