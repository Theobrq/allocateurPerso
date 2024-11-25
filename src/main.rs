#![no_std]
#![no_main]

extern crate alloc;
mod bump;
mod asm;
use crate::bump::BumpAllocator;
use core::panic::PanicInfo;
use alloc::boxed::Box;

///on créer notre propre heap
pub const HEAP_SIZE: usize = 1024 * 1024;

pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    ///creation d'un locked en initialisant le mutex
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

///ici on utilise une fonctione pour calculer l'alignement
///elle aligne l'adresse memoire addr vers la prochaine adresse qui est multiple de 2
fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

#[global_allocator]
static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());

#[no_mangle]
pub extern "C" fn rust_eh_personality() {
}

#[no_mangle]
pub extern "C" fn memset() {
}



#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    let heap_start = match asm::brk(None) {
        Ok(addr) => addr,
        Err(_) => {
            //erreur de brk
            loop {}
        }
    };

    //calculer la fin de la heap
    let heap_end = heap_start + HEAP_SIZE;

    //demander une extension de la heap via brk
    if let Err(_) = asm::brk(Some(heap_end)) {

        loop {}
    }


    unsafe {
        ALLOCATOR.lock().init(heap_start, HEAP_SIZE);
    }

    //allocation
    let _a = Box::new(6);

    loop {}
}
