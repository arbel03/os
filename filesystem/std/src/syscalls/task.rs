pub fn execv(path_name: &str, args: &[&str]) -> usize {
    use syscalls::syscall::syscall4;
    use alloc::{ Vec, String };
    let mut arguments: Vec<String> = Vec::new();
    let mut ptr_list: Vec<*const u8> = Vec::with_capacity(args.len());
    for arg in args {
        use alloc::string::ToString;
        let mut string_arg = arg.to_string();
        string_arg.push('\x00');
        arguments.push(string_arg);
        ptr_list.push(arg.as_ptr() as *const u8);
    }

    unsafe {
        syscall4(0x07, path_name.as_ptr() as usize, path_name.len(), ptr_list.as_ptr() as usize, args.len())
    }
}
