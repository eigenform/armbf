// Just an exhaustive list of all the instructions we support.

// Data-processing
_dataproc_imm:
	adc r4, r5, #4
	add r4, r5, #4
	and r4, r5, #4
	bic r4, r5, #4
	cmn r4, #4
	cmp r4, #4
	eor r4, r5, #4
	mov r3, #0xff
	mvn r3, #0xff
	orr r4, r5, #4
	rsb r4, r5, #4
	rsc r4, r5, #4
	sbc r4, r5, #4
	sub r4, r5, #4
	teq r3, #0xff
	tst r3, #0xff

_dataproc_shift:
	adc r4, r5, r6, lsl #4
	adc r4, r5, r6, lsr #4
	adc r4, r5, r6, asr #4
	adc r4, r5, r6, ror #4
	adc r4, r5, r6, lsl r7
	adc r4, r5, r6, lsr r7
	adc r4, r5, r6, asr r7
	adc r4, r5, r6, ror r7

	add r4, r5, r6, lsl #4
	add r4, r5, r6, lsr #4
	add r4, r5, r6, asr #4
	add r4, r5, r6, ror #4
	add r4, r5, r6, lsl r7
	add r4, r5, r6, lsr r7
	add r4, r5, r6, asr r7
	add r4, r5, r6, ror r7

	and r4, r5, r6, lsl #4
	and r4, r5, r6, lsr #4
	and r4, r5, r6, asr #4
	and r4, r5, r6, ror #4
	and r4, r5, r6, lsl r7
	and r4, r5, r6, lsr r7
	and r4, r5, r6, asr r7
	and r4, r5, r6, ror r7

	bic r4, r5, r6, lsl #4
	bic r4, r5, r6, lsr #4
	bic r4, r5, r6, asr #4
	bic r4, r5, r6, ror #4
	bic r4, r5, r6, lsl r7
	bic r4, r5, r6, lsr r7
	bic r4, r5, r6, asr r7
	bic r4, r5, r6, ror r7

	cmn r4, r5, lsl #4
	cmn r4, r5, lsr #4
	cmn r4, r5, asr #4
	cmn r4, r5, ror #4
	cmn r4, r5, lsl r7
	cmn r4, r5, lsr r7
	cmn r4, r5, asr r7
	cmn r4, r5, ror r7

	cmp r4, r5, lsl #4
	cmp r4, r5, lsr #4
	cmp r4, r5, asr #4
	cmp r4, r5, ror #4
	cmp r4, r5, lsl r7
	cmp r4, r5, lsr r7
	cmp r4, r5, asr r7
	cmp r4, r5, ror r7

	eor r4, r5, r6, lsl #4
	eor r4, r5, r6, lsr #4
	eor r4, r5, r6, asr #4
	eor r4, r5, r6, ror #4
	eor r4, r5, r6, lsl r7
	eor r4, r5, r6, lsr r7
	eor r4, r5, r6, asr r7
	eor r4, r5, r6, ror r7

	mov r4, r5, lsl #4
	mov r4, r5, lsr #4
	mov r4, r5, asr #4
	mov r4, r5, ror #4
	mov r4, r5, lsl r7
	mov r4, r5, lsr r7
	mov r4, r5, asr r7
	mov r4, r5, ror r7

	mvn r4, r5, lsl #4
	mvn r4, r5, lsr #4
	mvn r4, r5, asr #4
	mvn r4, r5, ror #4
	mvn r4, r5, lsl r7
	mvn r4, r5, lsr r7
	mvn r4, r5, asr r7
	mvn r4, r5, ror r7

	orr r4, r5, r6, lsl #4
	orr r4, r5, r6, lsr #4
	orr r4, r5, r6, asr #4
	orr r4, r5, r6, ror #4
	orr r4, r5, r6, lsl r7
	orr r4, r5, r6, lsr r7
	orr r4, r5, r6, asr r7
	orr r4, r5, r6, ror r7

	rsb r4, r5, r6, lsl #4
	rsb r4, r5, r6, lsr #4
	rsb r4, r5, r6, asr #4
	rsb r4, r5, r6, ror #4
	rsb r4, r5, r6, lsl r7
	rsb r4, r5, r6, lsr r7
	rsb r4, r5, r6, asr r7
	rsb r4, r5, r6, ror r7

	rsc r4, r5, r6, lsl #4
	rsc r4, r5, r6, lsr #4
	rsc r4, r5, r6, asr #4
	rsc r4, r5, r6, ror #4
	rsc r4, r5, r6, lsl r7
	rsc r4, r5, r6, lsr r7
	rsc r4, r5, r6, asr r7
	rsc r4, r5, r6, ror r7

	sbc r4, r5, r6, lsl #4
	sbc r4, r5, r6, lsr #4
	sbc r4, r5, r6, asr #4
	sbc r4, r5, r6, ror #4
	sbc r4, r5, r6, lsl r7
	sbc r4, r5, r6, lsr r7
	sbc r4, r5, r6, asr r7
	sbc r4, r5, r6, ror r7

	sub r4, r5, r6, lsl #4
	sub r4, r5, r6, lsr #4
	sub r4, r5, r6, asr #4
	sub r4, r5, r6, ror #4
	sub r4, r5, r6, lsl r7
	sub r4, r5, r6, lsr r7
	sub r4, r5, r6, asr r7
	sub r4, r5, r6, ror r7

	teq r4, r5, lsl #4
	teq r4, r5, lsr #4
	teq r4, r5, asr #4
	teq r4, r5, ror #4
	teq r4, r5, lsl r7
	teq r4, r5, lsr r7
	teq r4, r5, asr r7
	teq r4, r5, ror r7

	tst r4, r5, lsl #4
	tst r4, r5, lsr #4
	tst r4, r5, asr #4
	tst r4, r5, ror #4
	tst r4, r5, lsl r7
	tst r4, r5, lsr r7
	tst r4, r5, asr r7
	tst r4, r5, ror r7

_dataproc_shift_none:
	adc r4, r5, r6
	add r4, r5, r6
	and r4, r5, r6
	bic r4, r5, r6
	cmn r4, r5
	cmp r4, r5
	eor r4, r5, r6
	mov r3, r4
	mvn r3, r4
	orr r4, r5, r6
	rsb r4, r5, r6
	rsc r4, r5, r6
	sbc r4, r5, r6
	sub r4, r5, r6
	teq r3, r4
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

_swap:
	swp r3, r4, [r5]
	swpb r3, r4, [r5]

//_loadstore_multi_push:
//	push { r0-r9, sl, fp, ip, lr, pc }
//	stm sp!, { r0-r9, sl, fp, ip, lr, pc }
//
//_loadstore_multi_pop:
//	pop  { r0-r9, sl, fp, ip, lr, pc }
//	ldm sp!, { r0-r9, sl, fp, ip, lr, pc }

_loadstore_multi:
	stm r0, {r1-r9}
	ldm r0, {r1-r9}
	stm r0!, {r1-r9}
	ldm r0!, {r1-r9}

// Control
_misc_control:
	bkpt #0xcafe
	swi #0xcafe
	clz r4, r5

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

// Status register
_status_reg:
	mrs r3, cpsr
	mrs r3, spsr
	msr cpsr_c, r0
	msr cpsr_x, r0
	msr cpsr_f, r0
	msr cpsr_s, r0
	msr cpsr_fsxc, #0xd0
	msr cpsr_fc, #0xd0
	msr cpsr_fx, #0xd0
	msr cpsr_xc, #0xd0
	msr cpsr_sc, #0xd0


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

