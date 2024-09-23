use core::{
    alloc::{GlobalAlloc, Layout},
    arch::asm,
    ffi::{c_ulong, c_void},
    ptr::null_mut,
};

use crate::get_instance;

pub const HEAP_GROWABLE: c_ulong = 0x00000002;
pub const HEAP_ZERO_MEMORY: c_ulong = 0x00000008;

/// Global allocator implementation using NT Heap API.
#[global_allocator]
#[link_section = ".text"]
pub static NT_HEAPALLOCATOR: NtHeapAlloc = NtHeapAlloc::new();

/// Struct representing a custom heap allocator using the NT Heap API.
pub struct NtHeapAlloc;

unsafe impl Send for NtHeapAlloc {}
unsafe impl Sync for NtHeapAlloc {}

impl NtHeapAlloc {
    /// Creates a new `NtHeapAlloc`.
    pub const fn new() -> NtHeapAlloc {
        NtHeapAlloc
    }

    /// Retrieves the raw handle to the heap managed by this allocator.
    /// This function fetches the heap handle from the global instance.
    #[inline]
    fn handle(&self) -> *mut c_void {
        unsafe { get_instance().unwrap().heap_handle() }
    }

    /// Initializes the heap by calling `RtlCreateHeap` and storing the resulting handle.
    /// This function uses the global instance to set the heap handle.
    #[inline]
    pub fn initialize(&self) {
        let raw_heap_handle = unsafe {
            (get_instance().unwrap().ntdll.rtl_create_heap)(
                HEAP_GROWABLE,
                null_mut(),
                0,
                0,
                null_mut(),
                null_mut(),
            )
        };
        unsafe {
            get_instance()
                .unwrap()
                .set_heap_handle(raw_heap_handle as _)
        };
    }
}

/// Implementation of the `GlobalAlloc` trait for `NtHeapAlloc`,
/// using the NT Heap API for memory management.
unsafe impl GlobalAlloc for NtHeapAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        (get_instance().unwrap().ntdll.rtl_allocate_heap)(self.handle(), 0, layout.size())
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        (get_instance().unwrap().ntdll.rtl_allocate_heap)(
            self.handle(),
            HEAP_ZERO_MEMORY,
            layout.size(),
        )
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        (get_instance().unwrap().ntdll.rtl_free_heap)(self.handle(), 0, ptr);
    }

    unsafe fn realloc(&self, ptr: *mut u8, _layout: Layout, new_size: usize) -> *mut u8 {
        (get_instance().unwrap().ntdll.rtl_re_allocate_heap)(self.handle(), 0, ptr, new_size)
    }
}

#[no_mangle]
unsafe fn rust_oom() -> ! {
    asm!("ud2", options(noreturn));
}
