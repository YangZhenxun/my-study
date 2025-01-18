#![no_main]
#![no_std]

use core::panic::PanicInfo;
use core::arch::global_asm;

extern "Rust"{
    fn io_hlt();
}

global_asm!(include_str!("func.S"));

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop{}
}

#[no_mangle]
fn main(){
    loop{
        unsafe{io_hlt();}
    }
}