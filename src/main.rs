#![no_std]
#![no_main]

extern crate alloc;
mod bump;
use crate::bump::BumpAllocator;
use core::panic::PanicInfo;


//on créer notre propre heap
pub const HEAP_SIZE: usize = 100 * 1024;
pub const HEAP_START: usize = 0x0010;

pub struct Locked<A> {
    inner: spin::Mutex<A>,
}

impl<A> Locked<A> {
    pub const fn new(inner: A) -> Self {
        Locked {
            inner: spin::Mutex::new(inner),
        }
    }

    pub fn lock(&self) -> spin::MutexGuard<A> {
        self.inner.lock()
    }
}

//ici on utilise une fonctione pour calculer l'alignement
//elle aligne l'adresse memoire addr vers la prochaine adresse qui est multiple de 2
fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

#[global_allocator]
static ALLOCATOR: Locked<BumpAllocator> = Locked::new(BumpAllocator::new());



#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}


#[no_mangle]
pub extern "C" fn _start() -> ! {
    unsafe {
        ALLOCATOR.lock().init(HEAP_START, HEAP_SIZE);
    }
    
    loop {}
}
