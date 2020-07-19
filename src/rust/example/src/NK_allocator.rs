use core::alloc::{GlobalAlloc, Layout, alloc};
use core::ptr::null_mut;

pub struct NKAllocator;

unsafe impl GlobalAlloc for NKAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 { null_mut()}
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout){}
}

#[global_allocator]
static A: NKAllocator = NKAllocator;
