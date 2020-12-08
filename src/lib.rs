#![no_std]
#[no_mangle]

pub extern fn print_hello_from_rust() -> u8 {
    //println!("Hello from Rust");
		42
}


#[panic_handler]
fn my_panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}
