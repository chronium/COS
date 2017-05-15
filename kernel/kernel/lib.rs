#![feature(asm)]
#![feature(unique)]
#![feature(intrinsics)]
#![feature(type_ascription)]
#![feature(const_fn)]
#![feature(lang_items)]
#![no_std]

extern crate multiboot2;
extern crate volatile;
extern crate rlibc;
extern crate cpuio;
extern crate spin;

#[macro_use]
mod arch;

#[no_mangle]
pub extern fn kmain(mboot_info_addr: usize) {
    arch::io::clear_screen();

    println!("Hello from the ChronOS Kernel");

    let boot_info = unsafe { multiboot2::load(mboot_info_addr) };
    let memmap_tag = boot_info.memory_map_tag().expect("Memory map tag required");

    println!("Memory areas:");
    for area in memmap_tag.memory_areas() {
        println!("   start: 0x{:X}, length: 0x{:X}", area.base_addr, area.length);
    }

    let elf_sections_tag = boot_info.elf_sections_tag().expect("ELF-sections tag required");

    println!("Kernel sections");
    for section in elf_sections_tag.sections() {
        println!("   addr: 0x{:X}, size: 0x{:X}, flags: 0x{:X}", section.addr, section.size, section.flags);
    }

    let kernel_start = elf_sections_tag.sections().map(|s| s.addr).min().unwrap() + 0xFFFFFFFF80000000u64;
    let kernel_end = elf_sections_tag.sections().map(|s| s.addr + s.size).max().unwrap();

    let mboot_start = mboot_info_addr;
    let mboot_end = mboot_start + (boot_info.total_size as usize);

    println!("kernel_start: 0x{:X}, kernel_end: 0x{:X}", kernel_start, kernel_end);
    println!("multiboot_start: 0x{:X}, multiboot_end: 0x{:X}", mboot_start, mboot_end);

    loop{}
}

#[lang = "eh_personality"] extern fn eh_personality () {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt (fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
    println!("\n\nPANIC in {} at line {}:", file, line);
    println!("   {}", fmt);

    loop { }
}