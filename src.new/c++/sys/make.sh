#start shell
mkdir bin
nasm asmhead.asm -o bin/asmhead.bin
nasm ipl.asm -o bin/ipl.bin
nasm asmhead.asm -o bin/asmhead.o
clang main.c func.S bin/asmhead.o -r haribote.ld -o haribote.sys -nodefaultlibs -nostartfiles
dd if=/dev/zero of=haribote.img bs=512 count=2880
dd if=bin/ipl.bin of=haribote.img bs=512 count=1 conv=notrunc
dd if=haribote.sys of=haribote.img seek=33 bs=512 conv=notrunc
rm bin/asmhead.bin bin/ipl.bin bin/main.bin main.o func.o bin/asmhead.o all.o haribote.sys
rmdir bin
#end