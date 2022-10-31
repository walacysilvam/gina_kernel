/*
              Gina Kernel 
    versao 0.1 - teste/experimental
*/

#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("==================================================");
    println!("=        kernel running on version => ({})      =", 0.1);
    println!("==================================================");
    println!("=> Gina kernel iniciado :: Aguardando console...");

    loop {}
}

// Retorna o panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
