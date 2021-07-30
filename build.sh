cargo kbuild
cargo run --package boot
qemu-system-x86_64 -drive format=raw,file=target/x86_64-spinix/debug/boot-bios-spinix.img -serial stdio
