
_start:

dp_instr:
	adc r4, r5, #4
	adc r4, r5, r6

	add r4, r5, #4
	add r4, r5, r6

	and r4, r5, #4
	and r4, r5, r6

branch_instr:
	b _b_test

_b_test:
	bl _bl_test

_bl_test:
	bkpt #0xcafe


	bic r4, r5, #4
	bic r4, r5, r6

	blx r0
	bx r0

	cdp 13, 15, cr0, cr0, cr0, 0

	clz r4, r5

	cmn r4, #4
	cmn r4, r5


	cmp r4, #4
	cmp r4, r5

	eor r4, r5, #4
	eor r4, r5, r6

	push { r3, r4, r5, r6, r7, r8, r9, sl, fp, ip, lr, pc }
	ldm sp, {r4, r5, r6, r7, r8, r9, sl, fp, sp, lr }

	ldr r3, =_start
	ldrb r3, =_start
	ldrh r3, =_start
	ldrsb r3, =_start
	ldrsh r3, =_start

	mcr 15, 0, r0, cr7, cr5, 0
	mrc 15, 0, r3, cr5, cr0, 0

	mla r3, r4, r5, r6

	mov r3, #0xff
	mov r3, r4

	mrs r3, cpsr
	msr cpsr_c, r0
	msr cpsr_c, #0xd0

	mul r3, r4, r5

	mvn r3, #0xff
	mvn r3, r4

	orr r4, r5, #4
	orr r4, r5, r6

	qadd r3, r4, r5
	qdadd r3, r4, r5
	qsub r3, r4, r5
	qdsub r3, r4, r5

	rsb r4, r5, #4
	rsb r4, r5, r6

	rsc r4, r5, #4
	rsc r4, r5, r6

	sbc r4, r5, #4
	sbc r4, r5, r6

	str r3, _start
	strb r3, _start
	strh r3, _start

	sub r4, r5, #4
	sub r4, r5, r6

	swp r3, r4, [r5]
	swpb r3, r4, [r5]

	swi #0xcafe

	teq r3, #0xff
	teq r3, r4

	tst r3, #0xff
	tst r3, r4

	umlal r3, r4, r5, r6
	umull r3, r4, r5, r6


