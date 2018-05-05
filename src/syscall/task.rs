pub fn execv(file_name: &str, args: &[&str]) -> usize {
    unsafe {
        ::task::execv(file_name, args)
    }
}

pub fn proc_info(info_struct_ptr: *mut u8, proc_number: usize) -> usize {
    #[repr(packed)]
    #[derive(Debug)]
    pub struct ProcInfo {
        pub process_index: u32,
        pub process_name_length: u32,
        pub process_base: u32,
        pub process_total_size: u32,
        pub arguments_count: u32,
        pub process_stack_size: u32,
    }

    if let Some(process) = ::task::get_process_at_index(proc_number) {
        let load_request = process.get_load_information().get_load_request();
        let proc_info = ProcInfo {
            process_index: proc_number as u32,
            process_name_length: process.executable_file.get_process_name().len() as u32,
            process_base: process.get_load_information().process_base as u32,
            process_total_size: load_request.get_total_process_size() as u32,
            arguments_count: load_request.arguments_count as u32,
            process_stack_size: load_request.stack_area_size as u32,
        };
        use core::ptr;
        unsafe {
            ptr::write(info_struct_ptr as *mut ProcInfo, proc_info);
        }

        proc_number
    } else {
        0xffffffff
    }
}

pub fn proc_memory_size() -> usize {
    use task::PROCESS_ALLOCATOR;
    unsafe {
        PROCESS_ALLOCATOR.get_block_size() * PROCESS_ALLOCATOR.get_block_count()
    }
}