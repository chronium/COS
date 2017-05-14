#![feature(asm)]
#![feature(intrinsics)]
#![feature(lang_items)]
#![no_std]

extern crate rlibc;

#[no_mangle]
pub extern fn kmain () {
    // ATTENTION: we have a very small stack and no guard page

    let hello = b"Hello from Rust!";
    let color_byte = 0x07; // gray foreground, black background

    let mut hello_colored = [color_byte; 32];
    for (i, char_byte) in hello.into_iter().enumerate() {
        hello_colored[i*2] = *char_byte;
    }

    // write `Hello from Rust!` to the somehwat-center of the VGA text buffer
    let buffer_ptr = (0xb8000 + 1980) as *mut _;
    unsafe { *buffer_ptr = hello_colored };

    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality () {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt () -> ! {loop{}}