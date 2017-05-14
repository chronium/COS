#![feature(asm)]
#![feature(unique)]
#![feature(intrinsics)]
#![feature(type_ascription)]
#![feature(const_fn)]
#![feature(lang_items)]
#![no_std]

extern crate volatile;
extern crate rlibc;
extern crate cpuio;
extern crate spin;

#[macro_use]
mod arch;

#[no_mangle]
pub extern fn kmain () {
    arch::io::clear_screen ();
    println!("Hello World!");
    print!("This is a test!\nDoes it work?");

    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality () {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt () -> ! {loop{}}