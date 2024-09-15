#![no_main]
#![no_std]
mod lang_items;
mod vga_buf;

#[no_mangle]
fn main(){
    println!("hello world{}", "!");
}