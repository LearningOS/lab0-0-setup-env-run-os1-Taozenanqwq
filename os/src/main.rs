#![no_std]
#![no_main]
#![feature(panic_info_message)]

use log::*;

#[macro_use]
mod console;
mod lang_items;
mod logging;
mod sbi;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));

fn clear_bss() {
  extern "C" {
      fn sbss();
      fn ebss();
  }
  (sbss as usize..ebss as usize).for_each(|a| {
      unsafe { (a as *mut u8).write_volatile(0) }
  });
}

#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }
    clear_bss();
    logging::init();
    error!(".boot_stack [{:#x}, {:#x})", boot_stack as usize, boot_stack_top as usize);
    warn!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);
    info!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    debug!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    trace!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    panic!("Shutdown machine!");
}


//cargo build --release 
//丢弃源数据 rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os -O binary target/riscv64gc-unknown-none-elf/release/os.bin
/* 启动QUME加载内核镜像 qemu-system-riscv64 \
    -machine virt \
    -nographic \
    -bios ../bootloader/rustsbi-qemu.bin \
    -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \
    -s -S 
    */
/* GDB连接QUME riscv64-unknown-elf-gdb \
    -ex 'file target/riscv64gc-unknown-none-elf/release/os' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'
    */