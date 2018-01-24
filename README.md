# Screenshots
![Keyboard](https://i.imgur.com/Wa43Xir.png)

# Compilation Manual

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
