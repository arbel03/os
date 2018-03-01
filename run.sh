#!/bin/sh

FILESYSTEM_HEAD="build/head.bin"
FILESYSTEM="build/filesystem.bin"
OS_FILE="build/os.img"

filesize_in_sectors() {
    SIZE_IN_BYTES=0
    if [[ "$OSTYPE" == "linux-gnu" ]]; then
        SIZE_IN_BYTES=$(wc -c < $1)
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        SIZE_IN_BYTES=$(stat -f %z $1)
    fi
    echo $((SIZE_IN_BYTES / 512))
}

attach_kernel() {
    RESEREVED_SECTORS=$(filesize_in_sectors $FILESYSTEM_HEAD)
    cat $FILESYSTEM_HEAD > $OS_FILE
    dd if=$FILESYSTEM of=$OS_FILE count=1 bs=90 conv=notrunc
    dd if=$FILESYSTEM skip=$RESEREVED_SECTORS bs=512 >> $OS_FILE
}

create_filesystem() {
    # Unmounting
    sudo umount /mnt || true

    # Creating empty file
    dd if=/dev/zero of=$FILESYSTEM bs=$BLOCK_SIZE count=34
    
    # Getting the size of FILESYSTEM_HEAD
    RESEREVED_SECTORS=$(filesize_in_sectors $FILESYSTEM_HEAD)

    export PATH=/sbin:$PATH
    # Creating an empty Fat32 filesystem
    mkfs.fat -F 32 -R $RESEREVED_SECTORS $FILESYSTEM

    mkdir -p build/isofiles

    cp -r filesystem/. build/isofiles

    # Mounting the file and copying files to it
    if [[ "$OSTYPE" == "linux-gnu" ]]; then
        sudo mount -o loop $FILESYSTEM /mnt
        sudo cp -r build/isofiles/. /mnt
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        MOUNT_POINT=$(hdiutil attach -imagekey diskimage-class=CRawDiskImage -nomount $FILESYSTEM)
        echo "Mounting $FILESYSTEM to $MOUNT_POINT"
	    sudo mount -t msdos $MOUNT_POINT /mnt
	    sudo cp -r build/isofiles/. /mnt
	    hdiutil detach $MOUNT_POINT
    fi
    sudo umount /mnt || true
}

run() {
    BLOCK_SIZE=0
    LD=0
    if [[ "$OSTYPE" == "linux-gnu" ]]; then
        BLOCK_SIZE="1M"
        LD="ld"
    elif [[ "$OSTYPE" == "darwin"* ]]; then
        BLOCK_SIZE="1m"
        LD="i386-elf-ld"
    fi
    # Compiling bootloader and kernel into a single file
    make head LD=$LD
    success=$?
    # If there is no error in compiling the kernel
    if [ $success -ne 2 ]; then
        # Creating a filesystem Fat32 file with reserved sectors the size of the bootloader+kernel
        create_filesystem
        # Attaching the kernel to the filesystem file
        attach_kernel
        # Running
        qemu-system-i386 -drive file=$OS_FILE,format=raw
    fi
}

run