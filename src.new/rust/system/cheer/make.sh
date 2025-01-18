cargo rustc --release -- --emit asm
nasm src/asmhead.asm -o target/release/build/asmhead.o
clang target/release/deps/cheer-*.s target/release/build/asmhead.o -r src/haribote.ld -o haribote.sys
dd if=/dev/zero of=haribote.img bs=512 count=2880
nasm src/ipl.asm -o target/release/build/ipl.bin
dd if=target/release/build/ipl.bin of=haribote.img bs=512 count=1 conv=notrunc
dd if=haribote.sys of=haribote.img seek=33 bs=512 conv=notrunc
rm target/release/build/ipl.bin haribote.sys target/release/build/asmhead.o