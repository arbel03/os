use filesystem::FILESYSTEM;
use filesystem::File;

pub unsafe fn open(file_name: &str) -> usize {
    match FILESYSTEM.as_mut().unwrap().open_file(file_name) {
        Ok(file_descriptor) => file_descriptor,
        Err(open_error) => open_error as usize,
    }
}

pub unsafe fn seek(fd: usize, new_current: usize) -> usize {
    FILESYSTEM.as_mut().unwrap().seek(fd, new_current);
    1
}

pub unsafe fn stat(directory_path: &str, stat_ptr: *mut u8, child_node: usize) -> usize {
    use core::ptr;
    use filesystem::File;
    let filesystem = FILESYSTEM.as_mut().unwrap();

    #[repr(packed)]
    pub struct Stat {
        pub directory_name_length: u32,
        pub directory_size: u32,
        pub is_folder: bool,
        pub child_nodes_count: u32,
    }

    let parent_directory = if directory_path == "." {
        filesystem.get_root_directory()
    } else if let Some(dir) = filesystem.get_directory(directory_path) {
        dir
    } else {
        return 0xffffffff;
    };

    let child_dirs = filesystem.get_child_directories(&parent_directory);
    let directory = if child_node == 0 {
        parent_directory
    } else {
        child_dirs[child_node-1].clone()
    };
    
    let mut stat = Stat {
        directory_name_length: directory.get_name().len() as u32,
        directory_size: directory.get_size() as u32,
        is_folder: directory.get_fat_dir().is_folder(),
        child_nodes_count: 0,
    };

    if directory.get_fat_dir().is_folder() {
        if directory.get_fat_dir().get_cluster() == 0 {
            stat.child_nodes_count = child_dirs.len() as u32;
        } else {
            stat.child_nodes_count = FILESYSTEM.as_mut().unwrap().get_child_directories(&directory).len() as u32;
        }
    }
    ptr::write(stat_ptr as *mut Stat, stat);

    0
}

// Reading contents of file to buffer
pub unsafe fn read(fd: usize, read_buffer: &mut [u8]) -> usize {
    FILESYSTEM.as_mut().unwrap().read_file(fd, read_buffer)
}

pub unsafe fn read_dir_name(parent_dir_name: &str, name_buffer: &mut [u8], child_node: usize) -> usize {
    let filesystem = FILESYSTEM.as_mut().unwrap();
    let parent = if parent_dir_name == "." {
        filesystem.get_root_directory()
    } else if let Some(directory) = filesystem.get_directory(parent_dir_name) {
        directory
    } else {
        return 0xffffffff;
    };
    let dirs = FILESYSTEM.as_mut().unwrap().get_child_directories(&parent);
    let name = dirs[child_node].get_name();
    let name_bytes = name.as_bytes();
    &name_buffer.clone_from_slice(name_bytes);
    0
}