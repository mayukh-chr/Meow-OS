#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points


// run using
//> cargo build --target x86_64-blog_os.json 
//> qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin 
// this should be simplified by just loading the bootimage with
//> cargo bootimage
// followed by
//> cargo run

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

#[no_mangle] // don't mangle the name of this function


pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}


mod vga_buffer; //module to handle printing


/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}




