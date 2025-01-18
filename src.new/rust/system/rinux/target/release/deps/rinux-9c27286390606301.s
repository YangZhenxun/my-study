	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 11, 0
	.p2align	2
__ZN4core3fmt5Write10write_char17ha9974152ddeeecd2E:
	sub	sp, sp, #32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	stur	wzr, [x29, #-4]
	cmp	w1, #128
	b.hs	LBB0_2
	sturb	w1, [x29, #-4]
	mov	w2, #1
	sub	x1, x29, #4
	bl	_write_string
	mov	w0, #0
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB0_2:
	cmp	w1, #2048
	b.hs	LBB0_4
	lsr	w8, w1, #6
	orr	w8, w8, #0xc0
	sturb	w8, [x29, #-4]
	mov	w8, #128
	bfxil	w8, w1, #0, #6
	sturb	w8, [x29, #-3]
	mov	w2, #2
	sub	x1, x29, #4
	bl	_write_string
	mov	w0, #0
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB0_4:
	cmp	w1, #16, lsl #12
	b.hs	LBB0_6
	lsr	w8, w1, #12
	orr	w8, w8, #0xe0
	sturb	w8, [x29, #-4]
	mov	w8, #128
	mov	w9, #128
	bfxil	w9, w1, #6, #6
	sturb	w9, [x29, #-3]
	bfxil	w8, w1, #0, #6
	sturb	w8, [x29, #-2]
	mov	w2, #3
	sub	x1, x29, #4
	bl	_write_string
	mov	w0, #0
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret
LBB0_6:
	mov	w8, #240
	bfxil	w8, w1, #18, #3
	sturb	w8, [x29, #-4]
	mov	w8, #128
	mov	w9, #128
	bfxil	w9, w1, #12, #6
	sturb	w9, [x29, #-3]
	mov	w9, #128
	bfxil	w9, w1, #6, #6
	sturb	w9, [x29, #-2]
	bfxil	w8, w1, #0, #6
	sturb	w8, [x29, #-1]
	mov	w2, #4
	sub	x1, x29, #4
	bl	_write_string
	mov	w0, #0
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	ret

	.p2align	2
__ZN4core3fmt5Write9write_fmt17he6c24b252d635fccE:
	mov	x2, x1
Lloh0:
	adrp	x1, l___unnamed_1@PAGE
Lloh1:
	add	x1, x1, l___unnamed_1@PAGEOFF
	b	__ZN4core3fmt5write17he20999f7a51cdf3eE
	.loh AdrpAdd	Lloh0, Lloh1

	.p2align	2
__ZN4core3ops8function6FnOnce9call_once17h0ef8c50f7a941497E:
	strb	wzr, [x8]
	mov	w9, #32768
	movk	w9, #11, lsl #16
	stp	x9, xzr, [x8, #8]
	mov	w9, #15
	strb	w9, [x8, #24]
	ret

	.p2align	2
__ZN4spin4once17Once$LT$T$C$R$GT$18try_call_once_slow17h42d3a3258b9e333cE:
	.cfi_startproc
	sub	sp, sp, #64
	stp	x20, x19, [sp, #32]
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	mov	w10, #0
Lloh2:
	adrp	x8, __ZN5rinux7vga_buf6WRITER17hbf996fbf89dd8635E@PAGE+32
Lloh3:
	add	x8, x8, __ZN5rinux7vga_buf6WRITER17hbf996fbf89dd8635E@PAGEOFF+32
	mov	w9, #1
	b	LBB3_2
LBB3_1:
	mov	w10, #0
LBB3_2:
	casab	w10, w9, [x8]
	cmp	w10, #0
	b.eq	LBB3_10
	ands	w10, w10, #0xff
	b.le	LBB3_1
	cmp	w10, #1
	b.eq	LBB3_6
	b	LBB3_12
LBB3_5:
	isb
LBB3_6:
	ldaprb	w10, [x8]
	cmp	w10, #1
	b.eq	LBB3_5
	cbz	w10, LBB3_1
	cmp	w10, #2
	b.ne	LBB3_14
LBB3_9:
Lloh4:
	adrp	x0, __ZN5rinux7vga_buf6WRITER17hbf996fbf89dd8635E@PAGE
Lloh5:
	add	x0, x0, __ZN5rinux7vga_buf6WRITER17hbf996fbf89dd8635E@PAGEOFF
	ldp	x29, x30, [sp, #48]
	ldp	x20, x19, [sp, #32]
	add	sp, sp, #64
	ret
LBB3_10:
	ldr	x8, [x0]
	ldr	x9, [x8, #40]
	str	xzr, [x8, #40]
	cbz	x9, LBB3_15
Lloh6:
	adrp	x19, __ZN5rinux7vga_buf6WRITER17hbf996fbf89dd8635E@PAGE
Lloh7:
	add	x19, x19, __ZN5rinux7vga_buf6WRITER17hbf996fbf89dd8635E@PAGEOFF
	mov	x8, sp
	blr	x9
	ldp	q0, q1, [sp]
	stp	q0, q1, [x19]
	mov	w8, #2
	stlurb	w8, [x19, #32]
Lloh8:
	adrp	x0, __ZN5rinux7vga_buf6WRITER17hbf996fbf89dd8635E@PAGE
Lloh9:
	add	x0, x0, __ZN5rinux7vga_buf6WRITER17hbf996fbf89dd8635E@PAGEOFF
	ldp	x29, x30, [sp, #48]
	ldp	x20, x19, [sp, #32]
	add	sp, sp, #64
	ret
LBB3_12:
	cmp	w10, #2
	b.eq	LBB3_9
Lloh10:
	adrp	x0, l___unnamed_2@PAGE
Lloh11:
	add	x0, x0, l___unnamed_2@PAGEOFF
Lloh12:
	adrp	x2, l___unnamed_3@PAGE
Lloh13:
	add	x2, x2, l___unnamed_3@PAGEOFF
	mov	w1, #13
	bl	__ZN4core9panicking5panic17h231f56423bbc3dfcE
LBB3_14:
Lloh14:
	adrp	x0, l___unnamed_4@PAGE
Lloh15:
	add	x0, x0, l___unnamed_4@PAGEOFF
Lloh16:
	adrp	x2, l___unnamed_5@PAGE
Lloh17:
	add	x2, x2, l___unnamed_5@PAGEOFF
	mov	w1, #38
	bl	__ZN4core9panicking5panic17h231f56423bbc3dfcE
LBB3_15:
Lloh18:
	adrp	x0, l___unnamed_6@PAGE
Lloh19:
	add	x0, x0, l___unnamed_6@PAGEOFF
Lloh20:
	adrp	x2, l___unnamed_7@PAGE
Lloh21:
	add	x2, x2, l___unnamed_7@PAGEOFF
	mov	w1, #42
	bl	__ZN4core9panicking5panic17h231f56423bbc3dfcE
	.loh AdrpAdd	Lloh2, Lloh3
	.loh AdrpAdd	Lloh4, Lloh5
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpAdd	Lloh6, Lloh7
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh16, Lloh17
	.loh AdrpAdd	Lloh14, Lloh15
	.loh AdrpAdd	Lloh20, Lloh21
	.loh AdrpAdd	Lloh18, Lloh19
	.cfi_endproc

	.p2align	2
__ZN53_$LT$core..fmt..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17hd99318960518385dE:
	mov	x0, x1
Lloh22:
	adrp	x1, l___unnamed_8@PAGE
Lloh23:
	add	x1, x1, l___unnamed_8@PAGEOFF
	mov	w2, #5
	b	__ZN4core3fmt9Formatter9write_str17h981b7013b08ce90fE
	.loh AdrpAdd	Lloh22, Lloh23

	.private_extern	_rust_begin_unwind
	.globl	_rust_begin_unwind
	.p2align	2
_rust_begin_unwind:
LBB5_1:
	b	LBB5_1

	.globl	_write_byte
	.p2align	2
_write_byte:
	and	w8, w1, #0xff
	cmp	w8, #10
	b.ne	LBB6_8
	ldr	x8, [x0]
	mov	w9, #1
	mov	x10, x8
LBB6_2:
	mov	x11, #0
LBB6_3:
	add	x12, x10, x11
	ldrb	w13, [x12, #161]
	ldrb	w14, [x12, #160]
	strb	w14, [x12]
	strb	w13, [x12, #1]
	add	x11, x11, #2
	cmp	x11, #160
	b.ne	LBB6_3
	add	x9, x9, #1
	add	x10, x10, #160
	cmp	x9, #25
	b.ne	LBB6_2
	mov	x9, #0
	ldrb	w10, [x0, #16]
	mov	w11, #32
LBB6_6:
	add	x12, x8, x9
	strb	w11, [x12, #3840]
	strb	w10, [x12, #3841]
	add	x9, x9, #2
	cmp	x9, #160
	b.ne	LBB6_6
	str	xzr, [x0, #8]
	ret
LBB6_8:
	ldr	x10, [x0, #8]
	cmp	x10, #79
	b.ls	LBB6_16
	ldr	x8, [x0]
	mov	w9, #1
	mov	x10, x8
LBB6_10:
	mov	x11, #0
LBB6_11:
	add	x12, x10, x11
	ldrb	w13, [x12, #161]
	ldrb	w14, [x12, #160]
	strb	w14, [x12]
	strb	w13, [x12, #1]
	add	x11, x11, #2
	cmp	x11, #160
	b.ne	LBB6_11
	add	x9, x9, #1
	add	x10, x10, #160
	cmp	x9, #25
	b.ne	LBB6_10
	mov	x10, #0
	ldrb	w9, [x0, #16]
	mov	w11, #32
LBB6_14:
	add	x12, x8, x10
	strb	w11, [x12, #3840]
	strb	w9, [x12, #3841]
	add	x10, x10, #2
	cmp	x10, #160
	b.ne	LBB6_14
	mov	x10, #0
	add	x8, x8, x10, lsl #1
	strb	w1, [x8, #3840]
	strb	w9, [x8, #3841]
	add	x8, x10, #1
	str	x8, [x0, #8]
	ret
LBB6_16:
	ldrb	w9, [x0, #16]
	ldr	x8, [x0]
	add	x8, x8, x10, lsl #1
	strb	w1, [x8, #3840]
	strb	w9, [x8, #3841]
	add	x8, x10, #1
	str	x8, [x0, #8]
	ret

	.globl	_new_line
	.p2align	2
_new_line:
	ldr	x8, [x0]
	mov	w9, #1
	mov	x10, x8
LBB7_1:
	mov	x11, #0
LBB7_2:
	add	x12, x10, x11
	ldrb	w13, [x12, #161]
	ldrb	w14, [x12, #160]
	strb	w14, [x12]
	strb	w13, [x12, #1]
	add	x11, x11, #2
	cmp	x11, #160
	b.ne	LBB7_2
	add	x9, x9, #1
	add	x10, x10, #160
	cmp	x9, #25
	b.ne	LBB7_1
	mov	x9, #0
	ldrb	w10, [x0, #16]
	mov	w11, #32
LBB7_5:
	add	x12, x8, x9
	strb	w11, [x12, #3840]
	strb	w10, [x12, #3841]
	add	x9, x9, #2
	cmp	x9, #160
	b.ne	LBB7_5
	str	xzr, [x0, #8]
	ret

	.globl	_write_string
	.p2align	2
_write_string:
	cbz	x2, LBB8_30
	add	x8, x1, x2
	ldrb	w9, [x0, #16]
	mov	w10, #32
	ldp	x11, x13, [x0]
	b	LBB8_4
LBB8_2:
	mov	w12, #254
LBB8_3:
	add	x14, x11, x13, lsl #1
	strb	w12, [x14, #3840]
	strb	w9, [x14, #3841]
	add	x13, x13, #1
	str	x13, [x0, #8]
	cmp	x1, x8
	b.eq	LBB8_30
LBB8_4:
	ldrb	w12, [x1], #1
	cmp	w12, #31
	b.ls	LBB8_14
	cmp	w12, #126
	b.hi	LBB8_22
	cmp	x13, #80
	b.lo	LBB8_3
	mov	x13, x11
	mov	w14, #1
LBB8_8:
	mov	x15, #0
LBB8_9:
	add	x16, x13, x15
	ldrb	w17, [x16, #161]
	ldrb	w2, [x16, #160]
	strb	w2, [x16]
	strb	w17, [x16, #1]
	add	x15, x15, #2
	cmp	x15, #160
	b.ne	LBB8_9
	add	x14, x14, #1
	add	x13, x13, #160
	cmp	x14, #25
	b.ne	LBB8_8
	mov	x13, #0
LBB8_12:
	add	x14, x11, x13
	strb	w10, [x14, #3840]
	strb	w9, [x14, #3841]
	add	x13, x13, #2
	cmp	x13, #160
	b.ne	LBB8_12
	mov	x13, #0
	b	LBB8_3
LBB8_14:
	cmp	w12, #10
	b.ne	LBB8_22
	mov	x12, x11
	mov	w13, #1
LBB8_16:
	mov	x14, #0
LBB8_17:
	add	x15, x12, x14
	ldrb	w16, [x15, #161]
	ldrb	w17, [x15, #160]
	strb	w17, [x15]
	strb	w16, [x15, #1]
	add	x14, x14, #2
	cmp	x14, #160
	b.ne	LBB8_17
	add	x13, x13, #1
	add	x12, x12, #160
	cmp	x13, #25
	b.ne	LBB8_16
	mov	x12, #0
LBB8_20:
	add	x13, x11, x12
	strb	w10, [x13, #3840]
	strb	w9, [x13, #3841]
	add	x12, x12, #2
	cmp	x12, #160
	b.ne	LBB8_20
	mov	x13, #0
	str	x13, [x0, #8]
	cmp	x1, x8
	b.ne	LBB8_4
	b	LBB8_30
LBB8_22:
	cmp	x13, #80
	b.lo	LBB8_2
	mov	x12, x11
	mov	w13, #1
LBB8_24:
	mov	x14, #0
LBB8_25:
	add	x15, x12, x14
	ldrb	w16, [x15, #161]
	ldrb	w17, [x15, #160]
	strb	w17, [x15]
	strb	w16, [x15, #1]
	add	x14, x14, #2
	cmp	x14, #160
	b.ne	LBB8_25
	add	x13, x13, #1
	add	x12, x12, #160
	cmp	x13, #25
	b.ne	LBB8_24
	mov	x12, #0
LBB8_28:
	add	x13, x11, x12
	strb	w10, [x13, #3840]
	strb	w9, [x13, #3841]
	add	x12, x12, #2
	cmp	x12, #160
	b.ne	LBB8_28
	mov	x13, #0
	b	LBB8_2
LBB8_30:
	ret

	.globl	_write_str
	.p2align	2
_write_str:
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	bl	_write_string
	mov	w0, #0
	ldp	x29, x30, [sp], #16
	ret

	.globl	__print
	.p2align	2
__print:
	.cfi_startproc
	sub	sp, sp, #48
	stp	x20, x19, [sp, #16]
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	mov	x20, x0
Lloh24:
	adrp	x19, __ZN5rinux7vga_buf6WRITER17hbf996fbf89dd8635E@PAGE
Lloh25:
	add	x19, x19, __ZN5rinux7vga_buf6WRITER17hbf996fbf89dd8635E@PAGEOFF
	str	x19, [sp]
	ldapurb	w8, [x19, #32]
	cmp	w8, #2
	b.ne	LBB10_7
	mov	w9, #0
	mov	w8, #1
	casab	w9, w8, [x19]
	cmp	w9, #0
	b.ne	LBB10_3
	b	LBB10_5
LBB10_2:
	isb
LBB10_3:
	ldrb	w9, [x19]
	cbnz	w9, LBB10_2
	mov	w9, #0
	casab	w9, w8, [x19]
	cmp	w9, #0
	b.ne	LBB10_3
LBB10_5:
Lloh26:
	adrp	x1, l___unnamed_1@PAGE
Lloh27:
	add	x1, x1, l___unnamed_1@PAGEOFF
	add	x0, x19, #8
	mov	x2, x20
	bl	__ZN4core3fmt5write17he20999f7a51cdf3eE
	cbnz	w0, LBB10_8
	stlrb	wzr, [x19]
	ldp	x29, x30, [sp, #32]
	ldp	x20, x19, [sp, #16]
	add	sp, sp, #48
	ret
LBB10_7:
	mov	x0, sp
	bl	__ZN4spin4once17Once$LT$T$C$R$GT$18try_call_once_slow17h42d3a3258b9e333cE
	mov	x19, x0
	mov	w9, #0
	mov	w8, #1
	casab	w9, w8, [x0]
	cmp	w9, #0
	b.ne	LBB10_3
	b	LBB10_5
LBB10_8:
Lloh28:
	adrp	x0, l___unnamed_9@PAGE
Lloh29:
	add	x0, x0, l___unnamed_9@PAGEOFF
Lloh30:
	adrp	x3, l___unnamed_10@PAGE
Lloh31:
	add	x3, x3, l___unnamed_10@PAGEOFF
Lloh32:
	adrp	x4, l___unnamed_11@PAGE
Lloh33:
	add	x4, x4, l___unnamed_11@PAGEOFF
	add	x2, sp, #15
	mov	w1, #43
	bl	__ZN4core6result13unwrap_failed17h5e4590a7ae843be2E
	.loh AdrpAdd	Lloh24, Lloh25
	.loh AdrpAdd	Lloh26, Lloh27
	.loh AdrpAdd	Lloh32, Lloh33
	.loh AdrpAdd	Lloh30, Lloh31
	.loh AdrpAdd	Lloh28, Lloh29
	.cfi_endproc

	.globl	_main
	.p2align	2
_main:
	sub	sp, sp, #80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
Lloh34:
	adrp	x8, l___unnamed_12@PAGE
Lloh35:
	add	x8, x8, l___unnamed_12@PAGEOFF
	mov	w9, #1
	stp	x8, x9, [sp, #8]
	sub	x8, x29, #8
	stp	xzr, xzr, [sp, #32]
	str	x8, [sp, #24]
	add	x0, sp, #8
	bl	__print
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	ret
	.loh AdrpAdd	Lloh34, Lloh35

	.section	__TEXT,__const
l___unnamed_9:
	.ascii	"called `Result::unwrap()` on an `Err` value"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_10:
	.asciz	"\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\000\001\000\000\000\000\000\000"
	.quad	__ZN53_$LT$core..fmt..Error$u20$as$u20$core..fmt..Debug$GT$3fmt17hd99318960518385dE

	.section	__TEXT,__const
l___unnamed_6:
	.ascii	"Lazy instance has previously been poisoned"

l___unnamed_13:
	.ascii	"/Users/yangzhenxun/.cargo/registry/src/index.crates.io-6f17d22bba15001f/spin-0.9.8/src/lazy.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_7:
	.quad	l___unnamed_13
	.asciz	"^\000\000\000\000\000\000\000d\000\000\000\025\000\000"

	.section	__TEXT,__const
l___unnamed_2:
	.ascii	"Once panicked"

l___unnamed_14:
	.ascii	"/Users/yangzhenxun/.cargo/registry/src/index.crates.io-6f17d22bba15001f/spin-0.9.8/src/once.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_3:
	.quad	l___unnamed_14
	.asciz	"^\000\000\000\000\000\000\000\346\000\000\000*\000\000"

	.section	__TEXT,__const
l___unnamed_4:
	.ascii	"Once previously poisoned by a panicked"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_5:
	.quad	l___unnamed_14
	.asciz	"^\000\000\000\000\000\000\000N\001\000\000%\000\000"

	.section	__TEXT,__const
l___unnamed_8:
	.ascii	"Error"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_1:
	.asciz	"\000\000\000\000\000\000\000\000\030\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	_write_str
	.quad	__ZN4core3fmt5Write10write_char17ha9974152ddeeecd2E
	.quad	__ZN4core3fmt5Write9write_fmt17he6c24b252d635fccE

	.section	__TEXT,__const
l___unnamed_15:
	.ascii	"src/vga_buf.rs"

	.section	__DATA,__data
	.p2align	3, 0x0
__ZN5rinux7vga_buf6WRITER17hbf996fbf89dd8635E:
	.space	32
	.space	1
	.space	7
	.quad	__ZN4core3ops8function6FnOnce9call_once17h0ef8c50f7a941497E

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_11:
	.quad	l___unnamed_15
	.asciz	"\016\000\000\000\000\000\000\000\205\000\000\000#\000\000"

	.section	__TEXT,__const
l___unnamed_16:
	.ascii	"hello world!\n"

	.section	__DATA,__const
	.p2align	3, 0x0
l___unnamed_12:
	.quad	l___unnamed_16
	.asciz	"\r\000\000\000\000\000\000"

.subsections_via_symbols
