use crate::{align_up, Locked};
use core::alloc::{GlobalAlloc, Layout};
use core::ptr;
///documentation 
/// - https://os.phil-opp.com/heap-allocation/
/// -
///On  commence par créer la structure de l'allocateur, avec son debut, sa fin, le suivant et le nombre d'allocation
pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: usize,
    allocations: usize,
}



impl BumpAllocator {
    ///on commence par initialiser notre heap a 0
    pub const fn new() -> Self {
        BumpAllocator {
            heap_start: 0,
            heap_end: 0,
            next: 0,
            allocations: 0,
        }
    }
    ///cette methode est en unsafe car on est pas sur de la plage de memoire donnée
    pub unsafe fn init(&mut self, heap_start: usize, heap_size: usize) {
        //debut de la heap
        self.heap_start = heap_start;
        //le debut + la taille de la heap donné, qui nous donne donc la fin de la heap
        self.heap_end = heap_start + heap_size;
        //on pointe vers le début de la heap qui est inutilisé
        self.next = heap_start;
    }
}

///on défini le trait globalalloc en unsafe par ce qu'on modifie de la memoire
unsafe impl GlobalAlloc for Locked<BumpAllocator> {
    //on va définir deux method en unsafe, alloc et dealloc, pour allouer et deallouer de la memoire, encore une fois on utilise unsafe pour garantir
    //la securité de la memoire
    //on utilise layout pour avoir la memoire et la taille qu'il faut, ca garanti que les allocations restent a l'intereieur de leur region de memoire dans la heap
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        //on prend une reference mutable pour modifier l'état interne
        let mut bump = self.lock();
        //align_up
        let alloc_start = align_up(bump.next, layout.align());
        //checked_add va renvoyer none si un overflow se produit
        let alloc_end = match alloc_start.checked_add(layout.size()) {
            Some(end) => end,
            None => return ptr::null_mut(),
        };


        //on vérifie si alloc_end dépasse la limite de la heap
        if alloc_end > bump.heap_end {
            //c'est qu'il y a pas assez de mémoire, on renvoie un pointeur null
            ptr::null_mut()
        } else {
            //bump.next est mis à jour pour pointer vers la fin de l'espace mémoire alloué
            //ca fais en sorte que la prochaine allocation va commencer juste après l'espace (si on alloue 32 octet, bump.next est mis a jour pour pointer au premier octet libre après les 32 alloués
            bump.next = alloc_end;
            //on increment de 1 pour voire le nombre d'allocation
            bump.allocations += 1;
            //return du pointeur
            alloc_start as *mut u8
        }


    }
    //on défini dealloc
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        //ion reprend une reference mutable pour modifier l'état interne
        let mut bump = self.lock();
        //on enleve 1 a notre incrementation car on deallou
        bump.allocations -= 1;
        //si jamais il y a plus d'allocation, on reset next a heap_start
        if bump.allocations == 0 {
            bump.next = bump.heap_start;
        }
    }
}
