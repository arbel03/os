# Screenshots
![screenshot](https://i.imgur.com/kmqk6md.png)

# Compilation Manual

A linux environment is needed for compiling, dosfstools is needed to generate a disk img.

### Install Rust
Install rustup from https://www.rustup.rs  

### Configure Rust
Set the default keychain to nightly: `rustup override add nightly`

### Install Xargo
Install Xargo, a wrapper for cargo that eases cross compilation.  
`cargo install xargo`  
Add the rust source code component for cross compiling (needed by xargo).  
`rustup component add rust-src`  

### Run
Run using `./run.sh`  

### TwoUnderscorez's OS

An operating system similar to mine, but written in C, utilizes paging and implements ext2, [click here to check it out!](https://github.com/TwoUnderscorez/DuckOS)
