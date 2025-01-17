	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 11, 0

	.globl	_io_hlt
_io_hlt:
	hlt	#0
	ret


	.globl	_main
	.p2align	2
_main:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
LBB0_1:
	bl	_io_hlt
	b	LBB0_1

.subsections_via_symbols
