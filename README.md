# Screenshots
![Exception Handling](https://i.imgur.com/IDt038G.png)

# Compiling Instructions

### Install Rust
Install rustup from https://www.rustup.rs
Or by using the following command: 
`curl https://sh.rustup.rs -sSf | sh`

### Configure Rust
Set the default keychain to nightly.
`rustup override add nightly`

### Install Xargo
Cargo comes with rustup, but it is not enough for cross compiling, we will use xargo, a wrapper for cargo that eases cross compilation.
`cargo install xargo`
Add the rust source code component for cross compiling (needed by xargo).
`rustup component add rust-src`

### Run
Run using `make`
