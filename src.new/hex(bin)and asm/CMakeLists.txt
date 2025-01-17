cmake_minimum_required(VERSION 3.12)

project(Sys)
set(CMAKE_ASM_NASM_SOURCE_FILE_EXTENSIONS asm)
set(CMAKE_ASM_NASM_COMPILE_OBJECT "<CMAKE_ASM_NASM_COMPILER> <INCLUDES> <FLAGS> -o <OBJECT> <SOURCE>")
add_executable(build testa.asm ipl.asm)
enable_language(ASM_NASM)