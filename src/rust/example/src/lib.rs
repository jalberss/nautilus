#![feature(compiler_builtins)]
#![feature(allocator_api)]
//#![feature(alloc)]
#![feature(alloc_error_handler)]
// no stdlib
#![no_std]
//#![no_main]
// Give us this feature to override?
#![feature(start)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)] 

// cargo cult
#![feature(lang_items)]
#![no_builtins]

// https://docs.rs/containerof/0.2.2/src/containerof/lib.rs.html#37-41
#[macro_export]
macro_rules! containerof_field_offset {
    ($container:ty : $field:ident) => (unsafe {
        &(*(0usize as *const $container)).$field as *const _ as usize
    })
}

// avoid buildins - we want it to use our library
extern crate libc;
extern crate alloc;
// extern crate compiler_builtins;
pub mod bindings;
pub mod dev_bindings;
pub mod spinlock_bindings;
pub mod threads;
pub const MIN_ALIGN: usize = 16;
use alloc::boxed::Box;

// nomangle + pub extern "C" means standard C linkage and visibility
#[no_mangle]
pub extern "C" fn nk_rust_example(a: i32, b: i32) -> i32
{
    use alloc::vec::Vec;
    let mut v: Vec<i32> = Vec::new();
    v.push(a);
    v.push(b);
    call_nk_thread_create();
    // loop {
    //     unsafe {
    //         thread_bindings::nk_yield();
    //     }
    // };
    let a = v.pop().unwrap();
    let b = v.pop().unwrap();
    return a+b;
}

pub struct threadAdd(i32,i32);

#[lang = "eh_personality"]
#[no_mangle]
pub extern fn rust_eh_personality() {
}

// The following cruft is here to handle Rust->OS dependencies
// currently only one:  Rust needs to know how to panic
use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
pub fn nk_rust_panic(_info: &PanicInfo) -> !
{
   // should call nk_panic here...
   loop { }
}


///////////////////////////////////////////////////////////////
#[link(name = "dev")] 
extern {
    static mut state_lock: libc::uint32_t;
}

extern {
    static mut dev_list: dev_bindings::list_head;
}

#[no_mangle]
pub extern "C" fn nk_rust_nk_dev_dump_devices() {
    use ::libc::{uint8_t, c_char};
    use dev_bindings::*;
    use bindings::*;
    unsafe {
        nk_vc_printf("Pre NK Call\n\0".as_ptr() as *const i8);
        nk_dev_dump_devices();
        nk_vc_printf("Post NK Call\n\0".as_ptr() as *const i8);
    };

    // let state_lock: -> asdfasdfasdf
    // ignore lock for now (eventually get gcc to give us
    // an instantiation for spin_lock_irq_save and not
    // just inline everything.
    
    unsafe {
        nk_vc_printf("Pre NK Rust Call\n\0".as_ptr() as *const i8);
        let mut cur: *mut list_head = dev_list.next;
        let z: usize = containerof_field_offset!(nk_dev : dev_list_node);
        while cur != &mut dev_list as *mut list_head{
            let d_: *mut c_char = (cur as *mut c_char).sub(z);
            let d = d_ as *mut nk_dev;
            nk_dev_printf(d);
            cur = (*cur).next;
        }
	// spin_unlock_irq_restore(&mut state_lock,_state_lock_flags);
        nk_vc_printf("Post NK Rust Call\n\0".as_ptr() as *const i8);        
    }

    unsafe {nk_vc_printf("\nPre With Iterators\n\0".as_ptr() as *const i8);}
    let dev_iter = dev_iterator::new();
    for x in dev_iter {
        nk_dev_printf(x);
    }
    unsafe{
        nk_vc_printf("\nPost With Iterators\n\0".as_ptr() as *const i8);
        nk_vc_printf("\nPre With IntoIterator\n\0".as_ptr() as *const i8);
        let nk_dev: *mut nk_dev = ((dev_list.next) as *mut c_char).sub(containerof_field_offset!(nk_dev : dev_list_node)) as *mut nk_dev;
        for x in (*nk_dev).into_iter() {
            nk_dev_printf(x);
        }
        nk_vc_printf("\nPost With IntoIterator\n\0".as_ptr() as *const i8);
    }

    unsafe {nk_vc_printf("\nPre With map\n\0".as_ptr() as *const i8);}
    let dev_iter = dev_iterator::new();
    dev_iter.into_iter().for_each(|x| nk_dev_printf(x));
    unsafe {nk_vc_printf("\nPost With map\n\0".as_ptr() as *const i8);}

}

fn nk_dev_printf(d: *mut dev_bindings::nk_dev){
    use libc::c_char;
    use bindings::*;
     unsafe {
         let t =  nk_dev_dispatch((*d).type_).as_ptr() as *const i8;
         nk_vc_printf("%s: %s flags=0x%lx interface=%p state=%p\n\0".as_ptr() as *const i8, &(*d).name[0] as *const c_char, t, (*d).flags, (*d).interface, (*d).state);
    }
}

#[allow(non_upper_case_globals)]
fn nk_dev_dispatch(name: dev_bindings::nk_dev_type_t) -> & 'static str {
    use dev_bindings::*;

    match name {
        nk_dev_type_t_NK_DEV_GENERIC => "generic\0", 
        nk_dev_type_t_NK_DEV_INTR => "interrupt\0", 
        nk_dev_type_t_NK_DEV_TIMER => "timer\0", 
        nk_dev_type_t_NK_DEV_BUS => "bus\0", 
        nk_dev_type_t_NK_DEV_CHAR => "char\0",
        nk_dev_type_t_NK_DEV_BLK => "block\0",
        nk_dev_type_t_NK_DEV_NET => "net\0",
        _ => "unknown\0"
    }
}

fn call_nk_thread_create(){
    use threads::thread_bindings::*;
    use libc::c_void;
    let fp: nk_thread_fun_t = Some(call_test);
    let mut tid1: Box<u64> = Box::new(0);
    let mut tid2: Box<u64> = Box::new(0);
    let tid_p1: *mut nk_thread_id_t = Box::into_raw(tid1) as *mut nk_thread_id_t;
    let tid_p2: *mut nk_thread_id_t = Box::into_raw(tid2) as *mut nk_thread_id_t;
        
    unsafe {
        rust_puts("Before create\0");
        let mut x = nk_thread_start(fp, null_mut(), null_mut(), 0, 0, tid_p1, -1);
        nk_vc_printf("Ret value is: %d\n\0".as_ptr() as *mut i8, x);
        let mut res: *mut *mut c_void = null_mut();
        nk_join(*tid_p1, res);
        rust_puts("Post create\0");
    }

    // unsafe {
    //     rust_puts("Before create\0");
    //     let add = Box::new(threadAdd(9,9));
    //     let fun: nk_thread_fun_t = Some(nk_other);
    //     let b = Box::new(0);
    //     let b_raw = Box::into_raw(b);
    //     let mut unnecessary_cast = b_raw as *mut c_void;
    //     let mut out: *mut *mut c_void = (&mut unnecessary_cast) as *mut *mut c_void;

    //     let mut input: *mut c_void = Box::into_raw(add) as *mut c_void;
    //     let mut x = nk_thread_create(fun, input, out, 0,0, tid_p2, -1);
    //     let mut res = null_mut();

    //     nk_vc_printf("%d is valid?\n\0".as_ptr() as *mut i8, *tid_p2);

        
    //     let mut y = nk_join(*tid_p2,&mut res as *mut *mut c_void);
    //     nk_vc_printf("Is this 18?: %d\n\0".as_ptr() as *mut i8,*(res as * mut ::libc::uint64_t));
    // }
}

unsafe extern fn nk_other(input: *mut libc::c_void, out: *mut *mut libc::c_void ){
    let b: Box<threadAdd> = Box::from_raw(input as *mut threadAdd);
    *out = Box::into_raw(Box::new(b.0 + b.1)) as *mut libc::c_void;
    threads::thread_bindings::nk_thread_exit(*out);
}

unsafe extern fn call_test(_: *mut libc::c_void, _: *mut *mut libc::c_void){
    use threads::thread_bindings;
    thread_bindings::set_vc_for_rust();
    // loop {
    //     //thread_bindings::nk_yield();
    // };
    bindings::nk_vc_printf("Here! is this spawned?\0".as_ptr() as *const i8);
    thread_bindings::nk_thread_exit(null_mut());
}

unsafe extern fn rust_puts(s: &str){
    unsafe {
        bindings::nk_vc_printf("%s\n\0".as_ptr() as *const i8,s);
    }
}


// a Global Allocator cannot be used in submodules, boo

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;
pub mod mm_bindings;


pub struct NKAllocator;
unsafe impl GlobalAlloc for NKAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {

        // MIN_ALIGN is used instead of 4096 
        if layout.align() <= MIN_ALIGN && layout.align() <= layout.size() {
            mm_bindings::kmem_malloc(layout.size()) as *mut u8
        } else {
            aligned_malloc(&layout)
        }

    }
    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout){
        use libc::{c_void};
        
        mm_bindings::kmem_free(ptr as *mut c_void)
    }
}

#[inline]
unsafe fn aligned_malloc(_layout: &Layout) -> *mut u8 {
    use core::ptr;
    // Check to see if malloc gives back non aligned pointer!
    // libc::memalign(layout.align(), layout.size()) as *mut u8;
    ptr::null_mut()
}

#[global_allocator]
static ALLOC: NKAllocator = NKAllocator;


#[alloc_error_handler]
fn on_oom(_layout: Layout) -> ! {

    loop {}
}
// do we want to use kmem_malloc
//

// TODO make a println! macro
