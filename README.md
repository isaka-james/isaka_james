# Isaka_James OS

A hobby OS written in pure Rust.  
Built from scratch for learning, not production.

---

## features

- vga init  
- login prompt

---

## todo

- memory management  
- file system  
- shell  
- multitasking  
- drivers

---

## build & run

```sh
git clone https://github.com/isaka-jame/isaka_james.git
cd isaka_james

cargo install bootimage
cargo bootimage

qemu-system-x86_64 -drive format=raw,file=target/x86_64-isaka-james/debug/bootimage-isaka_james.bin
