#[repr(packed)]
#[derive(Debug, Default)]
pub struct ProcInfo {
    pub process_index: u32,
    pub process_name_length: u32,
    pub process_base: u32,
    pub process_total_size: u32,
    pub arguments_count: u32,
    pub process_stack_size: u32,
}

pub fn execv(path_name: &str, args: &[&str]) -> usize {
    // SYSCALL(PROC_EXECV, path_ptr, path_len, args_ptr, args_len)
    use syscalls::syscall::syscall4;
    use alloc::{ Vec, String };
    let mut arguments: Vec<String> = Vec::new();
    let mut ptr_list: Vec<*const u8> = Vec::with_capacity(args.len());
    for arg in args {
        use alloc::string::ToString;
        let mut string_arg = arg.to_string();
        string_arg.push('\x00');
        arguments.push(string_arg);
        ptr_list.push(arguments[arguments.len()-1].as_ptr() as *const u8);
    }

    unsafe {
        syscall4(0x07, path_name.as_ptr() as usize, path_name.len(), ptr_list.as_ptr() as usize, args.len())
    }
}

pub fn proc_info(proc_index: usize) -> Option<ProcInfo> {
    // SYSCALL(PROC_INFO, proc_info_ptr, proc_index)
    use syscalls::syscall::syscall2;

    let proc_info = ProcInfo::default();

    let result = unsafe { syscall2(0x09, &proc_info as *const ProcInfo as usize, proc_index) };
    if result == 0xffffffff {
        None
    } else {
        Some(proc_info)
    }
}

pub fn get_proccess_area_size() -> usize {
    // SYSCALL(PROC_SIZE)
    use syscalls::syscall::syscall0;
    unsafe {
        syscall0(0x10)
    }
}