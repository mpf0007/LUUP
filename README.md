<p align=center>
  <img src="./.statics/luup_logo.png"  alt="LUUP Logo" /></a>
</p>

Description
------------
`Operation System Written in Rust Language`

Instructions
------------

1. Install [Rust Nightly](http://www.rust-lang.org/install.html)

2. Install [QEMU](https://www.qemu.org/download)

3. For running `bootimage` and building the bootloader, you need to have the `llvm-tools-preview` rustup component installed. You can do so by executing `rustup component add llvm-tools-preview`

4. Clone the [LUUP](https://github.com/mpf0007/LUUP) source code .  

5. Run `cargo bootimage ` to build bootable disk image.
