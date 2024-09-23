use core::arch::asm;
use core::ffi::{c_ulong, c_void};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ListEntry {
    pub flink: *mut ListEntry,
    pub blink: *mut ListEntry,
}

#[repr(C)]
pub struct UnicodeString {
    pub length: u16,
    pub maximum_length: u16,
    pub buffer: *mut u16,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct SectionPointer {
    pub section_pointer: *mut c_void,
    pub check_sum: c_ulong,
}

#[repr(C)]
pub union HashLinksOrSectionPointer {
    pub hash_links: ListEntry,
    pub section_pointer: SectionPointer,
}

#[repr(C)]
pub union TimeDateStampOrLoadedImports {
    pub time_date_stamp: c_ulong,
    pub loaded_imports: *mut c_void,
}

#[repr(C)]
pub struct LoaderDataTableEntry {
    pub in_load_order_links: ListEntry,
    pub in_memory_order_links: ListEntry,
    pub in_initialization_order_links: ListEntry,
    pub dll_base: *mut c_void,
    pub entry_point: *mut c_void,
    pub size_of_image: c_ulong,
    pub full_dll_name: UnicodeString,
    pub base_dll_name: UnicodeString,
    pub flags: c_ulong,
    pub load_count: i16,
    pub tls_index: i16,
    pub hash_links_or_section_pointer: HashLinksOrSectionPointer,
    pub time_date_stamp_or_loaded_imports: TimeDateStampOrLoadedImports,
    pub entry_point_activation_context: *mut c_void,
    pub patch_information: *mut c_void,
    pub forwarder_links: ListEntry,
    pub service_tag_links: ListEntry,
    pub static_links: ListEntry,
}

#[repr(C)]
pub struct PebLoaderData {
    pub length: c_ulong,
    pub initialized: c_ulong,
    pub ss_handle: *mut c_void,
    pub in_load_order_module_list: ListEntry,
    pub in_memory_order_module_list: ListEntry,
    pub in_initialization_order_module_list: ListEntry,
}

#[repr(C)]
pub struct PEB {
    pub inherited_address_space: bool,
    pub read_image_file_exec_options: bool,
    pub being_debugged: bool,
    pub spare: bool,
    pub mutant: *mut c_void,
    pub image_base: *mut c_void,
    pub loader_data: *const PebLoaderData,
    pub process_parameters: *const RtlUserProcessParameters,
    pub sub_system_data: *mut c_void,
    pub process_heap: *mut c_void,
    pub fast_peb_lock: *mut c_void,
    pub fast_peb_lock_routine: *mut c_void,
    pub fast_peb_unlock_routine: *mut c_void,
    pub environment_update_count: c_ulong,
    pub kernel_callback_table: *const *mut c_void,
    pub event_log_section: *mut c_void,
    pub event_log: *mut c_void,
    pub free_list: *mut c_void,
    pub tls_expansion_counter: c_ulong,
    pub tls_bitmap: *mut c_void,
    pub tls_bitmap_bits: [c_ulong; 2],
    pub read_only_shared_memory_base: *mut c_void,
    pub read_only_shared_memory_heap: *mut c_void,
    pub read_only_static_server_data: *const *mut c_void,
    pub ansi_code_page_data: *mut c_void,
    pub oem_code_page_data: *mut c_void,
    pub unicode_case_table_data: *mut c_void,
    pub number_of_processors: c_ulong,
    pub nt_global_flag: c_ulong,
    pub spare_2: [u8; 4],
    pub critical_section_timeout: i64,
    pub heap_segment_reserve: c_ulong,
    pub heap_segment_commit: c_ulong,
    pub heap_de_commit_total_free_threshold: c_ulong,
    pub heap_de_commit_free_block_threshold: c_ulong,
    pub number_of_heaps: c_ulong,
    pub maximum_number_of_heaps: c_ulong,
    pub process_heaps: *const *const *mut c_void,
    pub gdi_shared_handle_table: *mut c_void,
    pub process_starter_helper: *mut c_void,
    pub gdi_dc_attribute_list: *mut c_void,
    pub loader_lock: *mut c_void,
    pub os_major_version: c_ulong,
    pub os_minor_version: c_ulong,
    pub os_build_number: c_ulong,
    pub os_platform_id: c_ulong,
    pub image_sub_system: c_ulong,
    pub image_sub_system_major_version: c_ulong,
    pub image_sub_system_minor_version: c_ulong,
    pub gdi_handle_buffer: [c_ulong; 22],
    pub post_process_init_routine: c_ulong,
    pub tls_expansion_bitmap: c_ulong,
    pub tls_expansion_bitmap_bits: [u8; 80],
    pub session_id: c_ulong,
}

#[repr(C)]
pub struct RtlUserProcessParameters {
    pub maximum_length: u32,
    pub length: u32,
    pub flags: u32,
    pub debug_flags: u32,
    pub console_handle: *mut c_void,
    pub console_flags: u32,
    pub standard_input: *mut c_void,
    pub standard_output: *mut c_void,
    pub standard_error: *mut c_void,
    pub current_directory_path: UnicodeString,
    pub current_directory_handle: *mut c_void,
    pub dll_path: UnicodeString,
    pub image_path_name: UnicodeString,
    pub command_line: UnicodeString,
    pub environment: *mut c_void,
    pub starting_x: u32,
    pub starting_y: u32,
    pub count_x: u32,
    pub count_y: u32,
    pub count_chars_x: u32,
    pub count_chars_y: u32,
    pub fill_attribute: u32,
    pub window_flags: u32,
    pub show_window_flags: u32,
    pub window_title: UnicodeString,
    pub desktop_info: UnicodeString,
    pub shell_info: UnicodeString,
    pub runtime_data: UnicodeString,
    pub current_directories: [UnicodeString; 32],
    pub environment_size: u32,
    pub environment_version: u32,
    pub package_dependency_data: *mut c_void,
    pub process_group_id: u32,
    pub loader_threads: u32,
}

pub const IMAGE_DOS_SIGNATURE: u16 = 0x5A4D; // "MZ"
pub const IMAGE_NT_SIGNATURE: u32 = 0x00004550; // "PE\0\0"

#[repr(C)]
pub struct ImageDosHeader {
    pub e_magic: u16,
    pub e_cblp: u16,
    pub e_cp: u16,
    pub e_crlc: u16,
    pub e_cparhdr: u16,
    pub e_minalloc: u16,
    pub e_maxalloc: u16,
    pub e_ss: u16,
    pub e_sp: u16,
    pub e_csum: u16,
    pub e_ip: u16,
    pub e_cs: u16,
    pub e_lfarlc: u16,
    pub e_ovno: u16,
    pub e_res: [u16; 4],
    pub e_oemid: u16,
    pub e_oeminfo: u16,
    pub e_res2: [u16; 10],
    pub e_lfanew: i32,
}

#[repr(C)]
pub struct ImageFileHeader {
    pub machine: u16,
    pub number_of_sections: u16,
    pub time_date_stamp: u32,
    pub pointer_to_symbol_table: u32,
    pub number_of_symbols: u32,
    pub size_of_optional_header: u16,
    pub characteristics: u16,
}

#[repr(C)]
pub struct ImageDataDirectory {
    pub virtual_address: u32,
    pub size: u32,
}

#[repr(C)]
pub struct ImageExportDirectory {
    pub characteristics: u32,
    pub time_date_stamp: u32,
    pub major_version: u16,
    pub minor_version: u16,
    pub name: u32,
    pub base: u32,
    pub number_of_functions: u32,
    pub number_of_names: u32,
    pub address_of_functions: u32,
    pub address_of_names: u32,
    pub address_of_name_ordinals: u32,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub struct ImageNtHeaders {
    pub signature: u32,
    pub file_header: ImageFileHeader,
    pub optional_header: ImageOptionalHeader64,
}

#[cfg(target_arch = "x86_64")]
#[repr(C)]
pub struct ImageOptionalHeader64 {
    pub magic: u16,
    pub major_linker_version: u8,
    pub minor_linker_version: u8,
    pub size_of_code: u32,
    pub size_of_initialized_data: u32,
    pub size_of_uninitialized_data: u32,
    pub address_of_entry_point: u32,
    pub base_of_code: u32,
    pub image_base: u64,
    pub section_alignment: u32,
    pub file_alignment: u32,
    pub major_operating_system_version: u16,
    pub minor_operating_system_version: u16,
    pub major_image_version: u16,
    pub minor_image_version: u16,
    pub major_subsystem_version: u16,
    pub minor_subsystem_version: u16,
    pub win32_version_value: u32,
    pub size_of_image: u32,
    pub size_of_headers: u32,
    pub check_sum: u32,
    pub subsystem: u16,
    pub dll_characteristics: u16,
    pub size_of_stack_reserve: u64,
    pub size_of_stack_commit: u64,
    pub size_of_heap_reserve: u64,
    pub size_of_heap_commit: u64,
    pub loader_flags: u32,
    pub number_of_rva_and_sizes: u32,
    pub data_directory: [ImageDataDirectory; 16],
}

#[cfg(target_arch = "x86_64")]
pub fn find_peb() -> *mut PEB {
    let peb_ptr: *mut PEB;
    unsafe {
        asm!(
        "mov {}, gs:[0x60]",
        out(reg) peb_ptr
        );
    }
    peb_ptr
}
