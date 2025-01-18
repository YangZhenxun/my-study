	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 11, 0

	.globl	_io_hlt
_io_hlt:
	hlt	#0
	ret


	.globl	_io_hlt
	.p2align	2
_io_hlt:
Lfunc_begin0:
	.cfi_sections .debug_frame
	.cfi_startproc
	.file	1 "/Users/yangzhenxun/Documents/GitHub/my-study/src.new/rust/system/cheer" "src/main.rs"
	.loc	1 8 21 prologue_end
	ret
Ltmp0:
Lfunc_end0:
	.cfi_endproc

	.private_extern	_rust_begin_unwind
	.globl	_rust_begin_unwind
	.p2align	2
_rust_begin_unwind:
Lfunc_begin1:
	.loc	1 13 0
	.cfi_startproc
	sub	sp, sp, #16
	.cfi_def_cfa_offset 16
	str	x0, [sp, #8]
Ltmp2:
	.loc	1 14 5 prologue_end
	b	LBB1_1
LBB1_1:
	b	LBB1_1
Ltmp3:
Lfunc_end1:
	.cfi_endproc

	.globl	_main
	.p2align	2
_main:
Lfunc_begin2:
	.loc	1 18 0
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
Ltmp4:
	.loc	1 19 5 prologue_end
	b	LBB2_1
LBB2_1:
	.loc	1 20 9
	bl	_io_hlt
	b	LBB2_1
Ltmp5:
Lfunc_end2:
	.cfi_endproc

	.section	__DWARF,__debug_abbrev,regular,debug
Lsection_abbrev:
	.byte	1
	.byte	17
	.byte	1
	.byte	37
	.byte	14
	.byte	19
	.byte	5
	.byte	3
	.byte	14
	.byte	16
	.byte	23
	.byte	27
	.byte	14
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	0
	.byte	0
	.byte	2
	.byte	57
	.byte	1
	.byte	3
	.byte	14
	.byte	0
	.byte	0
	.byte	3
	.byte	4
	.byte	1
	.byte	73
	.byte	19
	.byte	109
	.byte	25
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	4
	.byte	40
	.byte	0
	.byte	3
	.byte	14
	.byte	28
	.byte	15
	.byte	0
	.byte	0
	.byte	5
	.byte	19
	.byte	1
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.byte	50
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	6
	.byte	13
	.byte	0
	.byte	3
	.byte	14
	.byte	73
	.byte	19
	.ascii	"\210\001"
	.byte	15
	.byte	56
	.byte	11
	.byte	50
	.byte	11
	.byte	0
	.byte	0
	.byte	7
	.byte	51
	.byte	1
	.byte	21
	.byte	19
	.byte	0
	.byte	0
	.byte	8
	.byte	13
	.byte	0
	.byte	73
	.byte	19
	.ascii	"\210\001"
	.byte	15
	.byte	56
	.byte	11
	.byte	52
	.byte	25
	.byte	0
	.byte	0
	.byte	9
	.byte	25
	.byte	1
	.byte	22
	.byte	11
	.byte	0
	.byte	0
	.byte	10
	.byte	13
	.byte	0
	.byte	3
	.byte	14
	.byte	73
	.byte	19
	.ascii	"\210\001"
	.byte	15
	.byte	56
	.byte	11
	.byte	0
	.byte	0
	.byte	11
	.byte	19
	.byte	0
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.byte	50
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	12
	.byte	25
	.byte	1
	.byte	0
	.byte	0
	.byte	13
	.byte	19
	.byte	0
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	14
	.byte	47
	.byte	0
	.byte	73
	.byte	19
	.byte	3
	.byte	14
	.byte	0
	.byte	0
	.byte	15
	.byte	36
	.byte	0
	.byte	3
	.byte	14
	.byte	62
	.byte	11
	.byte	11
	.byte	11
	.byte	0
	.byte	0
	.byte	16
	.byte	46
	.byte	0
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.ascii	"\347\177"
	.byte	25
	.byte	64
	.byte	24
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	63
	.byte	25
	.byte	0
	.byte	0
	.byte	17
	.byte	46
	.byte	1
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.ascii	"\347\177"
	.byte	25
	.byte	64
	.byte	24
	.byte	110
	.byte	14
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.ascii	"\207\001"
	.byte	25
	.byte	0
	.byte	0
	.byte	18
	.byte	5
	.byte	0
	.byte	2
	.byte	24
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	19
	.byte	46
	.byte	0
	.byte	17
	.byte	1
	.byte	18
	.byte	6
	.byte	64
	.byte	24
	.byte	3
	.byte	14
	.byte	58
	.byte	11
	.byte	59
	.byte	11
	.byte	63
	.byte	25
	.byte	0
	.byte	0
	.byte	20
	.byte	15
	.byte	0
	.byte	73
	.byte	19
	.byte	3
	.byte	14
	.byte	51
	.byte	6
	.byte	0
	.byte	0
	.byte	21
	.byte	19
	.byte	1
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.ascii	"\210\001"
	.byte	15
	.byte	0
	.byte	0
	.byte	22
	.byte	15
	.byte	0
	.byte	73
	.byte	19
	.byte	51
	.byte	6
	.byte	0
	.byte	0
	.byte	23
	.byte	1
	.byte	1
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	24
	.byte	33
	.byte	0
	.byte	73
	.byte	19
	.byte	34
	.byte	13
	.byte	55
	.byte	11
	.byte	0
	.byte	0
	.byte	25
	.byte	36
	.byte	0
	.byte	3
	.byte	14
	.byte	11
	.byte	11
	.byte	62
	.byte	11
	.byte	0
	.byte	0
	.byte	26
	.byte	21
	.byte	1
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	27
	.byte	5
	.byte	0
	.byte	73
	.byte	19
	.byte	0
	.byte	0
	.byte	0
	.section	__DWARF,__debug_info,regular,debug
Lsection_info:
Lcu_begin0:
.set Lset0, Ldebug_info_end0-Ldebug_info_start0
	.long	Lset0
Ldebug_info_start0:
	.short	4
.set Lset1, Lsection_abbrev-Lsection_abbrev
	.long	Lset1
	.byte	8
	.byte	1
	.long	0
	.short	28
	.long	68
.set Lset2, Lline_table_start0-Lsection_line
	.long	Lset2
	.long	108
	.quad	Lfunc_begin0
.set Lset3, Lfunc_end2-Lfunc_begin0
	.long	Lset3
	.byte	2
	.long	179
	.byte	2
	.long	184
	.byte	2
	.long	188
	.byte	3
	.long	1126

	.long	191
	.byte	1
	.byte	1
	.byte	4
	.long	204
	.byte	0
	.byte	4
	.long	209
	.byte	1
	.byte	4
	.long	215
	.byte	2
	.byte	4
	.long	222
	.byte	3
	.byte	0
	.byte	5
	.long	670
	.byte	56
	.byte	1
	.byte	8
	.byte	6
	.long	682
	.long	1306
	.byte	8
	.byte	32
	.byte	1
	.byte	6
	.long	691
	.long	1457
	.byte	4
	.byte	40
	.byte	1
	.byte	6
	.long	701
	.long	57
	.byte	1
	.byte	48
	.byte	1
	.byte	6
	.long	707
	.long	1464
	.byte	4
	.byte	44
	.byte	1
	.byte	6
	.long	717
	.long	174
	.byte	8
	.byte	0
	.byte	1
	.byte	6
	.long	754
	.long	174
	.byte	8
	.byte	16
	.byte	1
	.byte	0
	.byte	5
	.long	727
	.byte	16
	.byte	1
	.byte	8
	.byte	7
	.long	187
	.byte	8
	.long	1320
	.byte	8
	.byte	0

	.byte	9
	.byte	0
	.byte	10
	.long	733
	.long	237
	.byte	8
	.byte	0
	.byte	0
	.byte	9
	.byte	1
	.byte	10
	.long	740
	.long	258
	.byte	8
	.byte	0
	.byte	0
	.byte	9
	.byte	2
	.byte	10
	.long	746
	.long	279
	.byte	8
	.byte	0
	.byte	0
	.byte	0
	.byte	5
	.long	733
	.byte	16
	.byte	1
	.byte	8
	.byte	6
	.long	736
	.long	1306
	.byte	8
	.byte	8
	.byte	1
	.byte	0
	.byte	5
	.long	740
	.byte	16
	.byte	1
	.byte	8
	.byte	6
	.long	736
	.long	1306
	.byte	8
	.byte	8
	.byte	1
	.byte	0
	.byte	11
	.long	746
	.byte	16
	.byte	1
	.byte	8
	.byte	0
	.byte	5
	.long	799
	.byte	16
	.byte	1
	.byte	8
	.byte	6
	.long	808
	.long	309
	.byte	8
	.byte	0
	.byte	3
	.byte	0
	.byte	5
	.long	811
	.byte	16
	.byte	3
	.byte	8
	.byte	7
	.long	322
	.byte	8
	.long	1320
	.byte	8
	.byte	8

	.byte	12
	.byte	10
	.long	670
	.long	357
	.byte	8
	.byte	0
	.byte	0
	.byte	9
	.byte	0
	.byte	10
	.long	727
	.long	390
	.byte	8
	.byte	0
	.byte	0
	.byte	0
	.byte	5
	.long	670
	.byte	16
	.byte	3
	.byte	8
	.byte	6
	.long	824
	.long	1510
	.byte	8
	.byte	0
	.byte	3
	.byte	6
	.long	883
	.long	1523
	.byte	8
	.byte	8
	.byte	3
	.byte	0
	.byte	5
	.long	727
	.byte	16
	.byte	3
	.byte	8
	.byte	6
	.long	736
	.long	1306
	.byte	8
	.byte	0
	.byte	3
	.byte	0
	.byte	0
	.byte	2
	.long	865
	.byte	13
	.long	876
	.byte	0
	.byte	1
	.byte	0
	.byte	0
	.byte	5
	.long	556
	.byte	48
	.byte	1
	.byte	8
	.byte	6
	.long	566
	.long	1340
	.byte	8
	.byte	0
	.byte	3
	.byte	6
	.long	184
	.long	795
	.byte	8
	.byte	32
	.byte	3
	.byte	6
	.long	767
	.long	1471
	.byte	8
	.byte	16
	.byte	3
	.byte	0
	.byte	11
	.long	1047
	.byte	0
	.byte	1
	.byte	1
	.byte	5
	.long	1085
	.byte	64
	.byte	1
	.byte	8
	.byte	6
	.long	707
	.long	1464
	.byte	4
	.byte	52
	.byte	3
	.byte	6
	.long	691
	.long	1457
	.byte	4
	.byte	48
	.byte	3
	.byte	6
	.long	701
	.long	57
	.byte	1
	.byte	56
	.byte	3
	.byte	6
	.long	754
	.long	892
	.byte	8
	.byte	0
	.byte	3
	.byte	6
	.long	717
	.long	892
	.byte	8
	.byte	16
	.byte	3
	.byte	6
	.long	1109
	.long	1572
	.byte	8
	.byte	32
	.byte	3
	.byte	0
	.byte	0
	.byte	2
	.long	243
	.byte	2
	.long	313
	.byte	5
	.long	324
	.byte	40
	.byte	1
	.byte	8
	.byte	6
	.long	334
	.long	1234
	.byte	8
	.byte	0
	.byte	3
	.byte	6
	.long	480
	.long	698
	.byte	8
	.byte	16
	.byte	3
	.byte	6
	.long	1172
	.long	1644
	.byte	8
	.byte	24
	.byte	3
	.byte	6
	.long	1237
	.long	1657
	.byte	1
	.byte	32
	.byte	3
	.byte	6
	.long	1253
	.long	1657
	.byte	1
	.byte	33
	.byte	3
	.byte	0
	.byte	0
	.byte	2
	.long	1172
	.byte	5
	.long	1214
	.byte	24
	.byte	1
	.byte	8
	.byte	6
	.long	1223
	.long	1379
	.byte	8
	.byte	0
	.byte	3
	.byte	6
	.long	1228
	.long	1464
	.byte	4
	.byte	16
	.byte	3
	.byte	6
	.long	1233
	.long	1464
	.byte	4
	.byte	20
	.byte	3
	.byte	0
	.byte	0
	.byte	0
	.byte	2
	.long	488
	.byte	5
	.long	495
	.byte	8
	.byte	1
	.byte	8
	.byte	7
	.long	711
	.byte	8
	.long	1320
	.byte	8
	.byte	0

	.byte	9
	.byte	0
	.byte	10
	.long	529
	.long	746
	.byte	8
	.byte	0
	.byte	0
	.byte	12
	.byte	10
	.long	762
	.long	764
	.byte	8
	.byte	0
	.byte	0
	.byte	0
	.byte	5
	.long	529
	.byte	8
	.byte	1
	.byte	8
	.byte	14
	.long	1327
	.long	760
	.byte	0
	.byte	5
	.long	762
	.byte	8
	.byte	1
	.byte	8
	.byte	14
	.long	1327
	.long	760
	.byte	6
	.long	736
	.long	1327
	.byte	8
	.byte	0
	.byte	1
	.byte	0
	.byte	0
	.byte	5
	.long	602
	.byte	16
	.byte	1
	.byte	8
	.byte	7
	.long	808
	.byte	8
	.long	1320
	.byte	8
	.byte	0

	.byte	9
	.byte	0
	.byte	10
	.long	529
	.long	843
	.byte	8
	.byte	0
	.byte	0
	.byte	12
	.byte	10
	.long	762
	.long	861
	.byte	8
	.byte	0
	.byte	0
	.byte	0
	.byte	5
	.long	529
	.byte	16
	.byte	1
	.byte	8
	.byte	14
	.long	1418
	.long	760
	.byte	0
	.byte	5
	.long	762
	.byte	16
	.byte	1
	.byte	8
	.byte	14
	.long	1418
	.long	760
	.byte	6
	.long	736
	.long	1418
	.byte	8
	.byte	0
	.byte	1
	.byte	0
	.byte	0
	.byte	5
	.long	1095
	.byte	16
	.byte	1
	.byte	8
	.byte	7
	.long	905
	.byte	8
	.long	1320
	.byte	8
	.byte	0

	.byte	9
	.byte	0
	.byte	10
	.long	529
	.long	941
	.byte	8
	.byte	0
	.byte	0
	.byte	9
	.byte	1
	.byte	10
	.long	762
	.long	959
	.byte	8
	.byte	0
	.byte	0
	.byte	0
	.byte	5
	.long	529
	.byte	16
	.byte	1
	.byte	8
	.byte	14
	.long	1306
	.long	760
	.byte	0
	.byte	5
	.long	762
	.byte	16
	.byte	1
	.byte	8
	.byte	14
	.long	1306
	.long	760
	.byte	6
	.long	736
	.long	1306
	.byte	8
	.byte	8
	.byte	1
	.byte	0
	.byte	0
	.byte	0
	.byte	2
	.long	1005
	.byte	5
	.long	1012
	.byte	1
	.byte	1
	.byte	1
	.byte	7
	.long	1009
	.byte	8
	.long	1126
	.byte	1
	.byte	0

	.byte	9
	.byte	0
	.byte	10
	.long	1041
	.long	1045
	.byte	1
	.byte	0
	.byte	0
	.byte	9
	.byte	1
	.byte	10
	.long	1055
	.long	1084
	.byte	1
	.byte	0
	.byte	0
	.byte	0
	.byte	5
	.long	1041
	.byte	1
	.byte	1
	.byte	1
	.byte	14
	.long	1552
	.long	760
	.byte	14
	.long	471
	.long	1053
	.byte	6
	.long	736
	.long	1552
	.byte	1
	.byte	1
	.byte	1
	.byte	0
	.byte	5
	.long	1055
	.byte	1
	.byte	1
	.byte	1
	.byte	14
	.long	1552
	.long	760
	.byte	14
	.long	471
	.long	1053
	.byte	6
	.long	736
	.long	471
	.byte	1
	.byte	1
	.byte	1
	.byte	0
	.byte	0
	.byte	0
	.byte	0
	.byte	15
	.long	201
	.byte	7
	.byte	1
	.byte	2
	.long	230
	.byte	16
	.quad	Lfunc_begin0
.set Lset4, Lfunc_end0-Lfunc_begin0
	.long	Lset4

	.byte	1
	.byte	111
	.long	236
	.byte	1
	.byte	8

	.byte	17
	.quad	Lfunc_begin1
.set Lset5, Lfunc_end1-Lfunc_begin1
	.long	Lset5

	.byte	1
	.byte	111
	.long	249
	.long	243
	.byte	1
	.byte	13

	.byte	18
	.byte	2
	.byte	145
	.byte	8
	.long	272
	.byte	1
	.byte	13
	.long	1221
	.byte	0
	.byte	19
	.quad	Lfunc_begin2
.set Lset6, Lfunc_end2-Lfunc_begin2
	.long	Lset6
	.byte	1
	.byte	109
	.long	267
	.byte	1
	.byte	18

	.byte	0
	.byte	20
	.long	571
	.long	277
	.long	0
	.byte	21
	.long	342
	.byte	16
	.byte	8
	.byte	10
	.long	385
	.long	1264
	.byte	8
	.byte	0
	.byte	10
	.long	435
	.long	1280
	.byte	8
	.byte	8
	.byte	0
	.byte	22
	.long	1273
	.long	0
	.byte	13
	.long	393
	.byte	0
	.byte	1
	.byte	20
	.long	1293
	.long	442
	.long	0
	.byte	23
	.long	1306
	.byte	24
	.long	1313
	.byte	0
	.byte	4
	.byte	0
	.byte	15
	.long	454
	.byte	7
	.byte	8
	.byte	25
	.long	460
	.byte	8
	.byte	7
	.byte	15
	.long	525
	.byte	7
	.byte	8
	.byte	20
	.long	426
	.long	534
	.long	0
	.byte	21
	.long	573
	.byte	16
	.byte	8
	.byte	10
	.long	581
	.long	1370
	.byte	8
	.byte	0
	.byte	10
	.long	595
	.long	1306
	.byte	8
	.byte	8
	.byte	0
	.byte	22
	.long	1379
	.long	0
	.byte	21
	.long	590
	.byte	16
	.byte	8
	.byte	10
	.long	581
	.long	1409
	.byte	8
	.byte	0
	.byte	10
	.long	595
	.long	1306
	.byte	8
	.byte	8
	.byte	0
	.byte	22
	.long	1126
	.long	0
	.byte	21
	.long	640
	.byte	16
	.byte	8
	.byte	10
	.long	581
	.long	1448
	.byte	8
	.byte	0
	.byte	10
	.long	595
	.long	1306
	.byte	8
	.byte	8
	.byte	0
	.byte	22
	.long	93
	.long	0
	.byte	15
	.long	696
	.byte	16
	.byte	4
	.byte	15
	.long	713
	.byte	7
	.byte	4
	.byte	21
	.long	772
	.byte	16
	.byte	8
	.byte	10
	.long	581
	.long	1501
	.byte	8
	.byte	0
	.byte	10
	.long	595
	.long	1306
	.byte	8
	.byte	8
	.byte	0
	.byte	22
	.long	288
	.long	0
	.byte	20
	.long	417
	.long	830
	.long	0
	.byte	20
	.long	1536
	.long	893
	.long	0
	.byte	26
	.long	996
	.byte	27
	.long	1510
	.byte	27
	.long	1559
	.byte	0
	.byte	15
	.long	1044
	.byte	7
	.byte	0
	.byte	20
	.long	479
	.long	1059
	.long	0
	.byte	21
	.long	1113
	.byte	16
	.byte	8
	.byte	10
	.long	385
	.long	1602
	.byte	8
	.byte	0
	.byte	10
	.long	435
	.long	1618
	.byte	8
	.byte	8
	.byte	0
	.byte	22
	.long	1611
	.long	0
	.byte	13
	.long	1139
	.byte	0
	.byte	1
	.byte	20
	.long	1631
	.long	1160
	.long	0
	.byte	23
	.long	1306
	.byte	24
	.long	1313
	.byte	0
	.byte	6
	.byte	0
	.byte	20
	.long	646
	.long	1181
	.long	0
	.byte	15
	.long	1248
	.byte	2
	.byte	1
	.byte	0
Ldebug_info_end0:
	.section	__TEXT,__text,regular,pure_instructions
Lsec_end0:
	.section	__DWARF,__debug_aranges,regular,debug
	.long	44
	.short	2
.set Lset7, Lcu_begin0-Lsection_info
	.long	Lset7
	.byte	8
	.byte	0
	.space	4,255
	.quad	Lfunc_begin0
.set Lset8, Lsec_end0-Lfunc_begin0
	.quad	Lset8
	.quad	0
	.quad	0
	.section	__DWARF,__debug_str,regular,debug
Linfo_string:
	.asciz	"clang LLVM (rustc version 1.80.1 (3f5fd8dd4 2024-08-06) (Homebrew))"
	.asciz	"src/main.rs/@/ex3fh3n5ov2e4ae57qbq9eh3c"
	.asciz	"/Users/yangzhenxun/Documents/GitHub/my-study/src.new/rust/system/cheer"
	.asciz	"core"
	.asciz	"fmt"
	.asciz	"rt"
	.asciz	"Alignment"
	.asciz	"u8"
	.asciz	"Left"
	.asciz	"Right"
	.asciz	"Center"
	.asciz	"Unknown"
	.asciz	"cheer"
	.asciz	"io_hlt"
	.asciz	"panic"
	.asciz	"rust_begin_unwind"
	.asciz	"main"
	.asciz	"info"
	.asciz	"&core::panic::panic_info::PanicInfo"
	.asciz	"panic_info"
	.asciz	"PanicInfo"
	.asciz	"payload"
	.asciz	"&(dyn core::any::Any + core::marker::Send)"
	.asciz	"pointer"
	.asciz	"(dyn core::any::Any + core::marker::Send)"
	.asciz	"vtable"
	.asciz	"&[usize; 4]"
	.asciz	"usize"
	.asciz	"__ARRAY_SIZE_TYPE__"
	.asciz	"message"
	.asciz	"option"
	.asciz	"Option<&core::fmt::Arguments>"
	.asciz	"u64"
	.asciz	"None"
	.asciz	"&core::fmt::Arguments"
	.asciz	"Arguments"
	.asciz	"pieces"
	.asciz	"&[&str]"
	.asciz	"data_ptr"
	.asciz	"&str"
	.asciz	"length"
	.asciz	"Option<&[core::fmt::rt::Placeholder]>"
	.asciz	"&[core::fmt::rt::Placeholder]"
	.asciz	"Placeholder"
	.asciz	"position"
	.asciz	"fill"
	.asciz	"char"
	.asciz	"align"
	.asciz	"flags"
	.asciz	"u32"
	.asciz	"precision"
	.asciz	"Count"
	.asciz	"Is"
	.asciz	"__0"
	.asciz	"Param"
	.asciz	"Implied"
	.asciz	"width"
	.asciz	"T"
	.asciz	"Some"
	.asciz	"args"
	.asciz	"&[core::fmt::rt::Argument]"
	.asciz	"Argument"
	.asciz	"ty"
	.asciz	"ArgumentType"
	.asciz	"value"
	.asciz	"&core::fmt::rt::{extern#0}::Opaque"
	.asciz	"{extern#0}"
	.asciz	"Opaque"
	.asciz	"formatter"
	.asciz	"fn(&core::fmt::rt::{extern#0}::Opaque, &mut core::fmt::Formatter) -> core::result::Result<(), core::fmt::Error>"
	.asciz	"result"
	.asciz	"Result<(), core::fmt::Error>"
	.asciz	"Ok"
	.asciz	"()"
	.asciz	"Error"
	.asciz	"E"
	.asciz	"Err"
	.asciz	"&mut core::fmt::Formatter"
	.asciz	"Formatter"
	.asciz	"Option<usize>"
	.asciz	"buf"
	.asciz	"&mut dyn core::fmt::Write"
	.asciz	"dyn core::fmt::Write"
	.asciz	"&[usize; 6]"
	.asciz	"location"
	.asciz	"&core::panic::location::Location"
	.asciz	"Location"
	.asciz	"file"
	.asciz	"line"
	.asciz	"col"
	.asciz	"can_unwind"
	.asciz	"bool"
	.asciz	"force_no_backtrace"
	.section	__DWARF,__apple_names,regular,debug
Lnames_begin:
	.long	1212240712
	.short	1
	.short	0
	.long	4
	.long	4
	.long	12
	.long	0
	.long	1
	.short	1
	.short	6
	.long	0
	.long	-1
	.long	2
	.long	3
	.long	81473412
	.long	270584624
	.long	2090499946
	.long	119828619
.set Lset9, LNames0-Lnames_begin
	.long	Lset9
.set Lset10, LNames1-Lnames_begin
	.long	Lset10
.set Lset11, LNames3-Lnames_begin
	.long	Lset11
.set Lset12, LNames2-Lnames_begin
	.long	Lset12
LNames0:
	.long	236
	.long	1
	.long	1138
	.long	0
LNames1:
	.long	243
	.long	1
	.long	1159
	.long	0
LNames3:
	.long	267
	.long	1
	.long	1199
	.long	0
LNames2:
	.long	249
	.long	1
	.long	1159
	.long	0
	.section	__DWARF,__apple_objc,regular,debug
Lobjc_begin:
	.long	1212240712
	.short	1
	.short	0
	.long	1
	.long	0
	.long	12
	.long	0
	.long	1
	.short	1
	.short	6
	.long	-1
	.section	__DWARF,__apple_namespac,regular,debug
Lnamespac_begin:
	.long	1212240712
	.short	1
	.short	0
	.long	10
	.long	10
	.long	12
	.long	0
	.long	1
	.short	1
	.short	6
	.long	0
	.long	-1
	.long	2
	.long	-1
	.long	3
	.long	-1
	.long	6
	.long	7
	.long	8
	.long	9
	.long	318227550
	.long	2090156110
	.long	255409292
	.long	270584624
	.long	1980029214
	.long	-1342284122
	.long	422565636
	.long	5863787
	.long	193491788
	.long	574740059
.set Lset13, Lnamespac6-Lnamespac_begin
	.long	Lset13
.set Lset14, Lnamespac0-Lnamespac_begin
	.long	Lset14
.set Lset15, Lnamespac3-Lnamespac_begin
	.long	Lset15
.set Lset16, Lnamespac4-Lnamespac_begin
	.long	Lset16
.set Lset17, Lnamespac9-Lnamespac_begin
	.long	Lset17
.set Lset18, Lnamespac7-Lnamespac_begin
	.long	Lset18
.set Lset19, Lnamespac8-Lnamespac_begin
	.long	Lset19
.set Lset20, Lnamespac2-Lnamespac_begin
	.long	Lset20
.set Lset21, Lnamespac1-Lnamespac_begin
	.long	Lset21
.set Lset22, Lnamespac5-Lnamespac_begin
	.long	Lset22
Lnamespac6:
	.long	488
	.long	1
	.long	693
	.long	0
Lnamespac0:
	.long	179
	.long	1
	.long	42
	.long	0
Lnamespac3:
	.long	230
	.long	1
	.long	1133
	.long	0
Lnamespac4:
	.long	243
	.long	1
	.long	561
	.long	0
Lnamespac9:
	.long	1172
	.long	1
	.long	641
	.long	0
Lnamespac7:
	.long	865
	.long	1
	.long	412
	.long	0
Lnamespac8:
	.long	1005
	.long	1
	.long	991
	.long	0
Lnamespac2:
	.long	188
	.long	1
	.long	52
	.long	0
Lnamespac1:
	.long	184
	.long	1
	.long	47
	.long	0
Lnamespac5:
	.long	313
	.long	1
	.long	566
	.long	0
	.section	__DWARF,__apple_types,regular,debug
Ltypes_begin:
	.long	1212240712
	.short	1
	.short	0
	.long	23
	.long	46
	.long	20
	.long	0
	.long	3
	.short	1
	.short	6
	.short	3
	.short	5
	.short	4
	.short	11
	.long	0
	.long	2
	.long	7
	.long	10
	.long	13
	.long	-1
	.long	15
	.long	-1
	.long	16
	.long	17
	.long	19
	.long	-1
	.long	22
	.long	24
	.long	27
	.long	29
	.long	32
	.long	34
	.long	35
	.long	37
	.long	-1
	.long	42
	.long	44
	.long	220205519
	.long	-594775205
	.long	193456014
	.long	1592673480
	.long	1855390635
	.long	-2052716040
	.long	-1449878611
	.long	1830702712
	.long	-1134209084
	.long	-863125541
	.long	277156213
	.long	2090120081
	.long	-305513218
	.long	232639254
	.long	1776306633
	.long	2087968388
	.long	2090147939
	.long	5862433
	.long	-938863729
	.long	-1182977908
	.long	-1142437763
	.long	-934778928
	.long	651940808
	.long	1086708957
	.long	110266314
	.long	193506143
	.long	1816246579
	.long	-2066133491
	.long	-1197510040
	.long	5862623
	.long	1209713282
	.long	1383688249
	.long	217729102
	.long	-715951632
	.long	-1988298567
	.long	1004366180
	.long	2089580953
	.long	5861270
	.long	2089401301
	.long	-1773357240
	.long	-309416900
	.long	-41616791
	.long	1004366114
	.long	1497627242
	.long	5863826
	.long	193506244
.set Lset23, Ltypes35-Ltypes_begin
	.long	Lset23
.set Lset24, Ltypes8-Ltypes_begin
	.long	Lset24
.set Lset25, Ltypes36-Ltypes_begin
	.long	Lset25
.set Lset26, Ltypes2-Ltypes_begin
	.long	Lset26
.set Lset27, Ltypes16-Ltypes_begin
	.long	Lset27
.set Lset28, Ltypes26-Ltypes_begin
	.long	Lset28
.set Lset29, Ltypes37-Ltypes_begin
	.long	Lset29
.set Lset30, Ltypes17-Ltypes_begin
	.long	Lset30
.set Lset31, Ltypes0-Ltypes_begin
	.long	Lset31
.set Lset32, Ltypes13-Ltypes_begin
	.long	Lset32
.set Lset33, Ltypes7-Ltypes_begin
	.long	Lset33
.set Lset34, Ltypes45-Ltypes_begin
	.long	Lset34
.set Lset35, Ltypes44-Ltypes_begin
	.long	Lset35
.set Lset36, Ltypes23-Ltypes_begin
	.long	Lset36
.set Lset37, Ltypes31-Ltypes_begin
	.long	Lset37
.set Lset38, Ltypes15-Ltypes_begin
	.long	Lset38
.set Lset39, Ltypes19-Ltypes_begin
	.long	Lset39
.set Lset40, Ltypes22-Ltypes_begin
	.long	Lset40
.set Lset41, Ltypes40-Ltypes_begin
	.long	Lset41
.set Lset42, Ltypes9-Ltypes_begin
	.long	Lset42
.set Lset43, Ltypes32-Ltypes_begin
	.long	Lset43
.set Lset44, Ltypes30-Ltypes_begin
	.long	Lset44
.set Lset45, Ltypes18-Ltypes_begin
	.long	Lset45
.set Lset46, Ltypes29-Ltypes_begin
	.long	Lset46
.set Lset47, Ltypes28-Ltypes_begin
	.long	Lset47
.set Lset48, Ltypes20-Ltypes_begin
	.long	Lset48
.set Lset49, Ltypes41-Ltypes_begin
	.long	Lset49
.set Lset50, Ltypes43-Ltypes_begin
	.long	Lset50
.set Lset51, Ltypes27-Ltypes_begin
	.long	Lset51
.set Lset52, Ltypes33-Ltypes_begin
	.long	Lset52
.set Lset53, Ltypes14-Ltypes_begin
	.long	Lset53
.set Lset54, Ltypes12-Ltypes_begin
	.long	Lset54
.set Lset55, Ltypes21-Ltypes_begin
	.long	Lset55
.set Lset56, Ltypes4-Ltypes_begin
	.long	Lset56
.set Lset57, Ltypes38-Ltypes_begin
	.long	Lset57
.set Lset58, Ltypes42-Ltypes_begin
	.long	Lset58
.set Lset59, Ltypes25-Ltypes_begin
	.long	Lset59
.set Lset60, Ltypes34-Ltypes_begin
	.long	Lset60
.set Lset61, Ltypes11-Ltypes_begin
	.long	Lset61
.set Lset62, Ltypes39-Ltypes_begin
	.long	Lset62
.set Lset63, Ltypes3-Ltypes_begin
	.long	Lset63
.set Lset64, Ltypes24-Ltypes_begin
	.long	Lset64
.set Lset65, Ltypes6-Ltypes_begin
	.long	Lset65
.set Lset66, Ltypes5-Ltypes_begin
	.long	Lset66
.set Lset67, Ltypes1-Ltypes_begin
	.long	Lset67
.set Lset68, Ltypes10-Ltypes_begin
	.long	Lset68
Ltypes35:
	.long	1047
	.long	1
	.long	471
	.short	19
	.byte	0
	.long	0
Ltypes8:
	.long	460
	.long	1
	.long	1313
	.short	36
	.byte	0
	.long	0
Ltypes36:
	.long	1055
	.long	1
	.long	1084
	.short	19
	.byte	0
	.long	0
Ltypes2:
	.long	277
	.long	1
	.long	1221
	.short	15
	.byte	0
	.long	0
Ltypes16:
	.long	602
	.long	1
	.long	795
	.short	19
	.byte	0
	.long	0
Ltypes26:
	.long	772
	.long	1
	.long	1471
	.short	19
	.byte	0
	.long	0
Ltypes37:
	.long	1059
	.long	1
	.long	1559
	.short	15
	.byte	0
	.long	0
Ltypes17:
	.long	640
	.long	1
	.long	1418
	.short	19
	.byte	0
	.long	0
Ltypes0:
	.long	191
	.long	1
	.long	57
	.short	4
	.byte	0
	.long	0
Ltypes13:
	.long	556
	.long	1
	.long	426
	.short	19
	.byte	0
	.long	0
Ltypes7:
	.long	454
	.long	1
	.long	1306
	.short	36
	.byte	0
	.long	0
Ltypes45:
	.long	1248
	.long	1
	.long	1657
	.short	36
	.byte	0
	.long	0
Ltypes44:
	.long	1214
	.long	1
	.long	646
	.short	19
	.byte	0
	.long	0
Ltypes23:
	.long	740
	.long	1
	.long	258
	.short	19
	.byte	0
	.long	0
Ltypes31:
	.long	893
	.long	1
	.long	1523
	.short	15
	.byte	0
	.long	0
Ltypes15:
	.long	590
	.long	1
	.long	1379
	.short	19
	.byte	0
	.long	0
Ltypes19:
	.long	696
	.long	1
	.long	1457
	.short	36
	.byte	0
	.long	0
Ltypes22:
	.long	733
	.long	1
	.long	237
	.short	19
	.byte	0
	.long	0
Ltypes40:
	.long	1113
	.long	1
	.long	1572
	.short	19
	.byte	0
	.long	0
Ltypes9:
	.long	495
	.long	1
	.long	698
	.short	19
	.byte	0
	.long	0
Ltypes32:
	.long	1012
	.long	1
	.long	996
	.short	19
	.byte	0
	.long	0
Ltypes30:
	.long	876
	.long	1
	.long	417
	.short	19
	.byte	0
	.long	0
Ltypes18:
	.long	670
	.long	2
	.long	93
	.short	19
	.byte	0
	.long	357
	.short	19
	.byte	0
	.long	0
Ltypes29:
	.long	830
	.long	1
	.long	1510
	.short	15
	.byte	0
	.long	0
Ltypes28:
	.long	811
	.long	1
	.long	309
	.short	19
	.byte	0
	.long	0
Ltypes20:
	.long	713
	.long	1
	.long	1464
	.short	36
	.byte	0
	.long	0
Ltypes41:
	.long	1139
	.long	1
	.long	1611
	.short	19
	.byte	0
	.long	0
Ltypes43:
	.long	1181
	.long	1
	.long	1644
	.short	15
	.byte	0
	.long	0
Ltypes27:
	.long	799
	.long	1
	.long	288
	.short	19
	.byte	0
	.long	0
Ltypes33:
	.long	1041
	.long	1
	.long	1045
	.short	19
	.byte	0
	.long	0
Ltypes14:
	.long	573
	.long	1
	.long	1340
	.short	19
	.byte	0
	.long	0
Ltypes12:
	.long	534
	.long	1
	.long	1327
	.short	15
	.byte	0
	.long	0
Ltypes21:
	.long	727
	.long	2
	.long	174
	.short	19
	.byte	0
	.long	390
	.short	19
	.byte	0
	.long	0
Ltypes4:
	.long	342
	.long	1
	.long	1234
	.short	19
	.byte	0
	.long	0
Ltypes38:
	.long	1085
	.long	1
	.long	479
	.short	19
	.byte	0
	.long	0
Ltypes42:
	.long	1160
	.long	1
	.long	1618
	.short	15
	.byte	0
	.long	0
Ltypes25:
	.long	762
	.long	3
	.long	764
	.short	19
	.byte	0
	.long	861
	.short	19
	.byte	0
	.long	959
	.short	19
	.byte	0
	.long	0
Ltypes34:
	.long	1044
	.long	1
	.long	1552
	.short	36
	.byte	0
	.long	0
Ltypes11:
	.long	529
	.long	3
	.long	746
	.short	19
	.byte	0
	.long	843
	.short	19
	.byte	0
	.long	941
	.short	19
	.byte	0
	.long	0
Ltypes39:
	.long	1095
	.long	1
	.long	892
	.short	19
	.byte	0
	.long	0
Ltypes3:
	.long	324
	.long	1
	.long	571
	.short	19
	.byte	0
	.long	0
Ltypes24:
	.long	746
	.long	1
	.long	279
	.short	19
	.byte	0
	.long	0
Ltypes6:
	.long	442
	.long	1
	.long	1280
	.short	15
	.byte	0
	.long	0
Ltypes5:
	.long	393
	.long	1
	.long	1273
	.short	19
	.byte	0
	.long	0
Ltypes1:
	.long	201
	.long	1
	.long	1126
	.short	36
	.byte	0
	.long	0
Ltypes10:
	.long	525
	.long	1
	.long	1320
	.short	36
	.byte	0
	.long	0
.subsections_via_symbols
	.section	__DWARF,__debug_line,regular,debug
Lsection_line:
Lline_table_start0:
