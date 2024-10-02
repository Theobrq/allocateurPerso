#![no_std]
#![no_main]

///Definition du trait pour implémenter global alloc
/// Alloc : alloue une taille de mémoire et d'allignement défini
/// Dealloc : free la memoire alloué juste avant
/// `Ressources` :
/// - https://doc.rust-lang.org/std/alloc/struct.Layout.html
/// - https://doc.rust-lang.org/stable/std/alloc/trait.GlobalAlloc.html
/// - https://doc.rust-lang.org/std/primitive.usize.html
pub unsafe trait GlobAlloc{
    unsafe fn alloc(&self, size: usize, align: usize) -> *mut u8;

    unsafe fn dealloc(&self, ptr: *mut u8, size: usize, align: usize);
}


///Suivre le nombre d'allocation
/// `Ressources` :
/// - https://bd103.github.io/blog/2023-06-27-global-allocators
/// -
//compteur
pub struct Counter(u64);

impl Counter{
    //initialiser a 0
    pub const fn new() -> Self{
        Counter(0)
    }
    //retourner le nbr actuel
    pub fn count(&self) -> u64{
        self.0
    }
}


///Definition de mon propre allocateur
/// `Ressources` :
/// - https://bd103.github.io/blog/2023-06-27-global-allocators
/// -
pub struct customAlloc;

unsafe impl GlobAlloc for customAlloc{
    unsafe fn alloc(&self, size: usize, align: usize) -> *mut u8 {

    }
    unsafe fn dealloc(&self, ptr: *mut u8, size: usize, align: usize) {

    }
}


/// Enregistrement de mon allocateur
#[global_allocator]
static A: customAlloc= customAlloc;
