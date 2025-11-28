#![no_std]
#![no_main]

extern crate alloc;
extern crate panic_halt;
use alloc::string::ToString;
use core::arch::global_asm;
use core::ffi::c_void;
use core::ptr::null_mut;

mod allocator;
mod instance;
mod ldrapi;
mod nocrt;
mod ntpeb;
mod utils;

use allocator::NT_HEAPALLOCATOR;
use instance::INSTANCE_MAGIC;
use instance::Instance;
use instance::init_native_funcs;
use ntpeb::find_peb;

global_asm!(
    r#"
.globl _start

.section .text

_start:
    push  rsi
    mov   rsi, rsp
    and   rsp, 0xFFFFFFFFFFFFFFF0
    sub   rsp, 0x20
    call  initialize
    mov   rsp, rsi
    pop   rsi
    ret
"#
);

unsafe extern "C" {
    unsafe fn _start();
}

#[unsafe(no_mangle)]
#[inline(never)]
pub extern "C" fn initialize() {
    unsafe {
        let mut instance = Instance::new();

        // Append instance address to PEB.ProcessHeaps
        let instance_ptr: *mut c_void = &mut instance as *mut _ as *mut c_void;

        let peb = find_peb();
        let process_heaps = (*peb).process_heaps as *mut *mut c_void;
        let number_of_heaps = (*peb).number_of_heaps as usize;

        // Increase the NumberOfHeaps
        (*peb).number_of_heaps += 1;

        // Append the instance_ptr
        *process_heaps.add(number_of_heaps) = instance_ptr;

        // Initialize native functions
        init_native_funcs();

        // Proceed to main function
        niam();
    }
}

fn niam() {
    let inst = get_instance().unwrap();
    let write_file = inst.write_file.unwrap();
    let nt_terminate_process = inst.ntdll.nt_terminate_process.unwrap();

    NT_HEAPALLOCATOR.initialize();

    let mut bytes_written: u32 = 0;
    let test = "Rustic64!".to_string();

    // Call WriteFile with a predefined message and handle (STD_OUTPUT_HANDLE = -11).
    unsafe {
        write_file(
            (-11i32) as u32 as *mut c_void, // STD_OUTPUT_HANDLE
            test.as_ptr() as *const c_void,
            test.len() as u32,
            &mut bytes_written,
            null_mut(),
        );
    }

    // Call TerminateProcess with process handle -1 (current process).
    unsafe {
        nt_terminate_process(-1isize as *mut c_void, 0);
    }
}

/// Attempts to locate the global `Instance` by scanning process heaps and
/// returns a mutable reference to it if found.
fn get_instance() -> Option<&'static mut Instance> {
    let peb = find_peb(); // Locate the PEB (Process Environment Block)
    let process_heaps = unsafe { (*peb).process_heaps };
    let number_of_heaps = unsafe { (*peb).number_of_heaps as usize };

    for i in 0..number_of_heaps {
        let heap = unsafe { *process_heaps.add(i) };
        if !heap.is_null() {
            let instance = unsafe { &mut *(heap as *mut Instance) };
            if instance.magic == INSTANCE_MAGIC {
                return Some(instance); // Return the instance if the magic value matches
            }
        }
    }
    None // Return None if the instance is not found
}
