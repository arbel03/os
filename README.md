# Screenshots
![screenshot](https://i.imgur.com/er0m4vr.png)
![screenshot](https://i.imgur.com/xFYjdVT.png)

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
Run using `make`  
