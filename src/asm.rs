use core::arch::asm;


///appelle la syscall brk pour gérer l'extension de la heap
///# Paramètres
///- new_end: Si Some(addr), on demande à brk de redéfinir la fin de la heap à addr Sinon none pour récupérer l'adresse actuelle
///# Retour
///Renvoie l'adresse de fin de la heap après l'appel ou une erreur en cas de problème
pub fn brk(new_end: Option<usize>) -> Result<usize, isize> {
    //syscall brk
    let syscall_number: usize = 12; 

    let addr: usize = new_end.unwrap_or(0);
    let mut result: isize;

    //appel du syscall brk avec assembleur
    unsafe {
        asm!(
            "syscall",
            in("rax") syscall_number,
            in("rdi") addr,
            lateout("rax") result,
            options(nostack, preserves_flags)
        );
    }

    if result < 0 {
        Err(result)
    } else {
        Ok(result as usize)
    }
}