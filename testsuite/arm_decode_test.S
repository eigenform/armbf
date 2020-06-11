// arm_decode_test.S
// Just an exhaustive list of all the instructions we support.

.include "macro.S"

_DpRotImm:
	DpRotImm_1 r4, 0x80000000
	DpRotImm_1 r4, 0x100
	DpRotImm_1 r4, 0
	DpRotImm_2 r4, r5, 0x80000000
	DpRotImm_2 r4, r5, 0x100
	DpRotImm_2 r4, r5, 0

_DpShiftImm:
	DpShiftImm_1 r3, r4, lsl, 4
	DpShiftImm_1 r3, r4, lsr, 4
	DpShiftImm_1 r3, r4, asr, 4
	DpShiftImm_1 r3, r4, ror, 4
	DpShiftImm_2 r3, r4, r5, lsl, 4
	DpShiftImm_2 r3, r4, r5, lsr, 4
	DpShiftImm_2 r3, r4, r5, asr, 4
	DpShiftImm_2 r3, r4, r5, ror, 4

_DpShiftReg:
	DpShiftReg_1 r3, r4, lsl, r5
	DpShiftReg_1 r3, r4, lsr, r5
	DpShiftReg_1 r3, r4, asr, r5
	DpShiftReg_1 r3, r4, ror, r5
	DpShiftReg_2 r3, r4, r5, lsl, r6
	DpShiftReg_2 r3, r4, r5, lsr, r6
	DpShiftReg_2 r3, r4, r5, asr, r6
	DpShiftReg_2 r3, r4, r5, ror, r6

_StatusRegisters:
	MrsReg r3
	MsrImm 0x80000000
	MsrImm 0x100
	MsrImm 0
	MsrReg r3

_Branch_Label:
_Branch:
	BranchImm _Branch_Label
	BranchReg r0
	BranchReg r1
	BranchReg r2
	BranchReg r3
	BranchReg r4
	BranchReg r5
	BranchReg r6
	BranchReg r7
	BranchReg r8
	BranchReg r9
	BranchReg r10
	BranchReg r11
	BranchReg r12
	BranchReg r13
	BranchReg r14

_LsImm:
	LsImm r3, r4, 0xfff
	LsImm r3, r4, -0xfff
	LsImm r3, r4, 0x100
	LsImm r3, r4, 0

_LsShift:
	LsShift r3, r4, r5

_loadstore:
	ldr r3, =_loadstore
	ldr r3, [r4, r5]
	ldr r3, [r4, #2]
	str r3, _loadstore
	str r3, [r4, r5]
	str r3, [r4, #2]

_loadstore_byte:
	ldrb r3, =_loadstore
	strb r3, _loadstore

_loadstore_half:
	ldrh r3, =_loadstore
	ldrh r3, [r4]
	ldrh r3, [r4, #4]
	ldrh r3, [r4, r5]
	strh r3, _loadstore
	strh r3, [r4]
	strh r3, [r4, #2]
	strh r3, [r4, r5]

_loadstore_shalf:
	ldrsh r3, =_loadstore
	ldrsh r3, [r4, r5]

_loadstore_sbyte:
	ldrsb r3, =_loadstore
	ldrsb r3, [r4, r5]


// Load/store multiple
_loadstore_multi:
	stm r0, {r1-r9}
	ldm r0, {r1-r9}
	stm r0!, {r1-r9}
	ldm r0!, {r1-r9}

// Misc/Control
_misc_control:
	bkpt #0xcafe
	swi #0xcafe
	clz r4, r5
	swp r3, r4, [r5]
	swpb r3, r4, [r5]

// Coprocessor 
_coproc:
	cdp 13, 15, cr0, cr0, cr0, 0
	mcr 15, 0, r0, cr7, cr5, 0
	mrc 15, 0, r3, cr5, cr0, 0

// Saturated add/sub
_sat_add_sub:
	qadd r3, r4, r5
	qdadd r3, r4, r5
	qsub r3, r4, r5
	qdsub r3, r4, r5

// Multiplies and extended signed multiplies
_mul:
	mul r3, r4, r5
_mla:
	mla r3, r4, r5, r6
_umull:
	umull r3, r4, r5, r6
	umlal r3, r4, r5, r6
	smlal r3, r4, r5, r6
	smull r3, r4, r5, r6
_smla_xy:
	smlabb r3, r4, r5, r6
	smlabt r3, r4, r5, r6
	smlatb r3, r4, r5, r6
	smlatt r3, r4, r5, r6
_smlal_xy:
	smlalbb r3, r4, r5, r6
	smlalbt r3, r4, r5, r6
	smlaltb r3, r4, r5, r6
	smlaltt r3, r4, r5, r6
_smlaw_y:
	smlawb r3, r4, r5, r6
	smlawt r3, r4, r5, r6
_smulw_y:
	smulwb r3, r4, r5
	smulwt r3, r4, r5
_smul_xy:
	smulbb r3, r4, r5
	smulbt r3, r4, r5
	smultb r3, r4, r5
	smultt r3, r4, r5

