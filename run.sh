#!/bin/sh

FILESYSTEM_HEAD="build/head.bin"
FILESYSTEM="build/filesystem.bin"
OS_FILE="build/os.bin"

filesize_in_sectors() {
    size_in_bytes=0
    if [[ "$OSTYPE" == "linux-gnu" ]]; then
        size_in_bytes=$(stat -L -c %s $1)
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        size_in_bytes=$(stat -f %z $1)
    fi
    return $(( size_in_bytes/512 ))
}

attach_kernel() {
    filesize_in_sectors $FILESYSTEM_HEAD
    RESEREVED_SECTORS=$?
    cat $FILESYSTEM_HEAD > $OS_FILE
    dd if=$FILESYSTEM of=$OS_FILE count=1 bs=90 conv=notrunc
    dd if=$FILESYSTEM skip=$RESEREVED_SECTORS bs=512 >> $OS_FILE
}

create_filesystem() {
    # Unmounting
    sudo umount /mnt || true
    # Creating empty file
    dd if=/dev/zero of=$FILESYSTEM bs=1m count=34

    # Getting the size of FILESYSTEM_HEAD
    filesize_in_sectors $FILESYSTEM_HEAD
    RESEREVED_SECTORS=$?
    
    # Creating an empty Fat32 filesystem
    mkfs.fat -F 32 -R $RESEREVED_SECTORS $FILESYSTEM

    mkdir -p build/isofiles

    # Adding files to the newly created filesystem
    mkdir build/isofiles/testdir
    mkdir build/isofiles/testasdasd
    mkdir build/isofiles/testasdasd2
    mkdir build/isofiles/testasdasd3
    mkdir build/isofiles/testasdas4
    echo 'This is a sample file for testing' > build/isofiles/testdir/testfile.txt

    # Mounting the file and copying files to it
    if [[ "$OSTYPE" == "linux-gnu" ]]; then
        sudo mount -o loop FILESYSTEM /mnt
        sudo cp -r build/isofiles/. /mnt
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        MOUNT_POINT=$(hdiutil attach -imagekey diskimage-class=CRawDiskImage -nomount $FILESYSTEM)
        echo "Mounting $FILESYSTEM to $MOUNT_POINT"
	    sudo mount -t msdos $MOUNT_POINT /mnt
	    sudo cp -r build/isofiles/. /mnt
	    hdiutil detach $MOUNT_POINT
    fi
    sudo umount /mnt || true
    rm -r build/isofiles
}

run() {
    # Compiling bootloader and kernel into a single file
    make head
    # Creating a filesystem Fat32 file with reserved sectors the size of the bootloader+kernel
    create_filesystem
    # Attaching the kernel to the filesystem file
    attach_kernel
    # Running
    qemu-system-i386 -drive file=$OS_FILE,format=raw
}

run