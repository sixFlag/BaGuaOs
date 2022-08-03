#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("{:?}", _info);
    qemu_exit::exit(qemu_exit::ExitCode::Failed);
    loop {}
}

#[macro_use]
mod console;
mod qemu_exit;
mod memory;

use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {

    serial_println!("Hello, World!");
    serial_println!("{:?}", boot_info.memory_map);
    serial_println!("{:#x}", boot_info.physical_memory_offset);
    panic!("shutdown");
}