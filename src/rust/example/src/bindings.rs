#![allow(non_camel_case_types)] 
pub type spinlock_t = u32;

extern "C" {
    pub fn nk_vc_printf(fmt: *const ::libc::c_char, ...) -> ::libc::c_int;
}

extern "C" {
    pub fn spin_lock_irq_save(lock: *mut spinlock_t) -> u8;
}

extern "C" {
    pub fn spin_unlock_irq_restore(lock: *mut spinlock_t,flags: u8);
}

