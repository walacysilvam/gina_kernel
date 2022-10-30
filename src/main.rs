//
//      Gina OS 
//  versao 0.1 - teste

#![no_std]  //retira qualquer lib.
#![no_main] //retira qualquer ponto de entrada.

// usado para criar a panic-handler
use core::panic::PanicInfo;

// funcao panic handler para retornar erros,
// nao existe panic handler nem qualquer outra funcao aqui
// pre definidas. Compilacao "pura".
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// funcao de entrada substituindo a inicalizacao C padrao.
// a funcao de entrada DEVE se chamar _start

static HELLO: &[u8] = b"Gina's Alive!";
#[no_mangle]
pub extern "C" fn _start() -> ! {
    //convertendo o buffer em um ponteiro bruto 
    let vga_buffer = 0x8000 as *mut u8;
    
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize *2 + 1) = 0xb;
        }
    }

    loop {}
}

