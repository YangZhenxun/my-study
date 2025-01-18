#![no_main]
#![no_std]

use kernel::init::initcall_t;
mod kernel;

fn kernel_init(){ todo!() }

fn early_boot_irqs_disabled(){ todo!() }

enum system_states {}

pub fn do_one_initcall(func: initcall_t){
    todo!()
}


#[no_mangle]
fn main(){

}