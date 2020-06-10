// Just an exhaustive list of all the instructions we support.

// Data-processing
_dataproc:
	adc r4, r5, #4
	adc r4, r5, r6
	add r4, r5, #4
	add r4, r5, r6
	and r4, r5, #4
	and r4, r5, r6
	bic r4, r5, #4
	bic r4, r5, r6
	cmn r4, #4
	cmn r4, r5
	cmp r4, #4
	cmp r4, r5
	eor r4, r5, #4
	eor r4, r5, r6
	mov r3, #0xff
	mov r3, r4
	mvn r3, #0xff
	mvn r3, r4
	orr r4, r5, #4
	orr r4, r5, r6
	rsb r4, r5, #4
	rsb r4, r5, r6
	rsc r4, r5, #4
	rsc r4, r5, r6
	sbc r4, r5, #4
	sbc r4, r5, r6
	sub r4, r5, #4
	sub r4, r5, r6
	teq r3, #0xff
	teq r3, r4
	tst r3, #0xff
	tst r3, r4

// Branching
_branch:
	b _branch
	bl _branch
	bx r3
	blx r3
	blx _branch

// Load/store
_loadstore:
	ldr r3, =_loadstore
	ldr r3, [r4, r5]
	ldr r3, [r4, #2]
	ldrb r3, =_loadstore
	ldrh r3, =_loadstore
	ldrh r3, [r4]
	ldrh r3, [r4, #4]
	ldrh r3, [r4, r5]
	ldrsb r3, =_loadstore
	ldrsb r3, [r4, r5]
	ldrsh r3, =_loadstore
	ldrsh r3, [r4, r5]
	str r3, _loadstore
	str r3, [r4, r5]
	str r3, [r4, #2]
	strb r3, _loadstore
	strh r3, _loadstore
	strh r3, [r4, #2]
	strh r3, [r4, r5]
	swp r3, r4, [r5]
	swpb r3, r4, [r5]
	push { r3, r4, r5, r6, r7, r8, r9, sl, fp, ip, lr, pc }
	ldm sp, {r4, r5, r6, r7, r8, r9, sl, fp, sp, lr }

// Control
_control_test:
	bkpt #0xcafe
	cdp 13, 15, cr0, cr0, cr0, 0
	clz r4, r5
	mcr 15, 0, r0, cr7, cr5, 0
	mrc 15, 0, r3, cr5, cr0, 0
	mrs r3, cpsr
	msr cpsr_c, r0
	msr cpsr_c, #0xd0
	swi #0xcafe

// Saturated add/sub
_sat_add_sub:
	qadd r3, r4, r5
	qdadd r3, r4, r5
	qsub r3, r4, r5
	qdsub r3, r4, r5

// Multiplies
_mul:
	mul r3, r4, r5
_mla:
	mla r3, r4, r5, r6
_umull:
	umull r3, r4, r5, r6
	umlal r3, r4, r5, r6
	smlal r3, r4, r5, r6
	smull r3, r4, r5, r6



// Signed multiplies (extended)
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

