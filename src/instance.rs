use core::{ffi::c_void, ptr::null_mut};

use crate::{
    get_instance,
    ldrapi::{ldr_function, ldr_module},
};

pub type RtlCreateHeap = unsafe extern "system" fn(
    Flags: u32,
    HeapBase: *mut u8,
    ReserveSize: usize,
    CommitSize: usize,
    Lock: *mut u8,
    Parameters: *mut u8,
) -> *mut c_void;

pub type RtlAllocateHeap =
    unsafe extern "system" fn(hHeap: *mut c_void, dwFlags: u32, dwBytes: usize) -> *mut u8;

pub type RtlFreeHeap =
    unsafe extern "system" fn(hHeap: *mut c_void, dwFlags: u32, lpMem: *mut u8) -> i32;

pub type RtlReAllocateHeap = unsafe extern "system" fn(
    hHeap: *mut c_void,
    dwFlags: u32,
    lpMem: *mut u8,
    dwBytes: usize,
) -> *mut u8;

pub type RtlDestroyHeap = unsafe extern "system" fn(hHeap: *mut c_void) -> *mut c_void;

pub type NtTerminateProcess =
    unsafe extern "system" fn(ProcessHandle: *mut c_void, ExitStatus: i32) -> i32;

pub struct Ntdll {
    pub module_base: *mut u8,
    pub rtl_create_heap: RtlCreateHeap,
    pub rtl_allocate_heap: RtlAllocateHeap,
    pub rtl_free_heap: RtlFreeHeap,
    pub rtl_re_allocate_heap: RtlReAllocateHeap,
    pub rtl_destroy_heap: RtlDestroyHeap,
    pub nt_terminate_process: NtTerminateProcess,
}

impl Ntdll {
    pub fn new() -> Self {
        Ntdll {
            module_base: null_mut(),
            rtl_create_heap: unsafe { core::mem::transmute(null_mut::<c_void>()) },
            rtl_allocate_heap: unsafe { core::mem::transmute(null_mut::<c_void>()) },
            rtl_free_heap: unsafe { core::mem::transmute(null_mut::<c_void>()) },
            rtl_re_allocate_heap: unsafe { core::mem::transmute(null_mut::<c_void>()) },
            rtl_destroy_heap: unsafe { core::mem::transmute(null_mut::<c_void>()) },
            nt_terminate_process: unsafe { core::mem::transmute(null_mut::<c_void>()) },
        }
    }
}

pub type WriteFile = unsafe extern "system" fn(
    hFile: *mut c_void,
    lpBuffer: *const c_void,
    nNumberOfBytesToWrite: u32,
    lpNumberOfBytesWritten: *mut u32,
    lpOverlapped: *mut c_void,
) -> i32;

#[repr(C)]
// Struct representing an instance that holds function pointers and a base address.
pub struct Instance {
    pub magic: u32,
    pub heap_handle: *mut c_void,
    pub ntdll: Ntdll,
    pub kernel32_base: *mut u8,
    pub write_file: WriteFile,
}

// Constant to identify a valid instance by a unique "magic" number.
pub const INSTANCE_MAGIC: u32 = 0x17171717;

impl Instance {
    pub fn new() -> Self {
        Instance {
            magic: INSTANCE_MAGIC,
            heap_handle: null_mut(),
            ntdll: Ntdll::new(),
            kernel32_base: null_mut(),
            write_file: unsafe { core::mem::transmute(null_mut::<c_void>()) },
        }
    }

    pub fn heap_handle(&self) -> *mut c_void {
        self.heap_handle
    }

    pub fn set_heap_handle(&mut self, handle: *mut c_void) {
        self.heap_handle = handle;
    }
}

pub fn init_native_funcs() {
    const NTDLL_DBJ2: u32 = 0x1edab0ed;
    const RTL_CREATE_HEAP_H: usize = 0xe1af6849;
    const RTL_ALLOCATE_HEAP_H: usize = 0x3be94c5a;
    const RTL_FREE_HEAP_H: usize = 0x73a9e4d7;
    const RTL_DESTROY_HEAP_H: usize = 0xceb5349f;
    const RTL_REALLOCATE_HEAP_H: usize = 0xaf740371;
    const NT_TERMINATE_PROCESS_H: usize = 0x4ed9dd4f;

    const KERNEL32_DBJ2: u32 = 0x6ddb9555;
    const WRITE_FILE_DBJ2: usize = 0xf1d207d0;

    unsafe {
        let instance = get_instance().unwrap();

        // Load the base address of ntdll.dll.
        instance.ntdll.module_base = ldr_module(NTDLL_DBJ2);

        // Resolve RtlCreateHeap
        let rtl_create_heap_addr = ldr_function(instance.ntdll.module_base, RTL_CREATE_HEAP_H);
        instance.ntdll.rtl_create_heap = core::mem::transmute(rtl_create_heap_addr);

        // Resolve RtlAllocateHeap
        let rtl_allocate_heap_addr = ldr_function(instance.ntdll.module_base, RTL_ALLOCATE_HEAP_H);
        instance.ntdll.rtl_allocate_heap = core::mem::transmute(rtl_allocate_heap_addr);

        // Resolve RtlFreeHeap
        let rtl_free_heap_addr = ldr_function(instance.ntdll.module_base, RTL_FREE_HEAP_H);
        instance.ntdll.rtl_free_heap = core::mem::transmute(rtl_free_heap_addr);

        // Resolve RtlReAllocateHeap
        let rtl_reallocate_heap_addr =
            ldr_function(instance.ntdll.module_base, RTL_REALLOCATE_HEAP_H);
        instance.ntdll.rtl_re_allocate_heap = core::mem::transmute(rtl_reallocate_heap_addr);

        // Resolve RtlDestroyHeap
        let rtl_destroy_heap_addr = ldr_function(instance.ntdll.module_base, RTL_DESTROY_HEAP_H);
        instance.ntdll.rtl_destroy_heap = core::mem::transmute(rtl_destroy_heap_addr);

        // Resolve NtTerminateProcess
        let nt_terminate_process_addr =
            ldr_function(instance.ntdll.module_base, NT_TERMINATE_PROCESS_H);
        instance.ntdll.nt_terminate_process = core::mem::transmute(nt_terminate_process_addr);

        // Load the base address of kernel32.dll.
        instance.kernel32_base = ldr_module(KERNEL32_DBJ2);

        // Resolve WriteFile
        let k_write_file_addr = ldr_function(instance.kernel32_base, WRITE_FILE_DBJ2);
        instance.write_file = core::mem::transmute(k_write_file_addr);
    }
}
