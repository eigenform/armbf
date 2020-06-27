.thumb

dataproc:
	adc r0, r1
	add r0, r1, #0
	add r0, r1, #7
	add r0, #0xff
	add r0, #0x0
	add r0, r1, r2
	add r0, r11
	add r0, pc, #0xf0
	add r0, sp, #0xf0
	add sp, #0xf0
	and r0, r1
	asr r0, r1, #0x20
	asr r0, r1
	bic r0, r1
	cmn r0, r1
	cmp r0, #0xff
	cmp r0, r1
	cmp r0, r10
	eor r0, r1
	lsl r0, r1, #0x1f
	lsl r0, r1
	lsr r0, r1, #0x1f
	lsr r0, r1
	mov r0, #0xff
	mov r0, r1
	mov r0, r10
	mul r0, r1
	mvn r0, r1
	neg r0, r1
	orr r0, r1
	ror r0, r1
	sbc r0, r1
	sub r0, r1, #0x7
	sub r0, #0xff
	sub r0, r1, r2
	sub sp, #0x80
	tst r0, r1


misc:
	bkpt 0xff
	swi 0xff

branch:
	//bl _armbranch
	//blx _armbranch
	//blx r2
	//blx r10
	//bx r3
	//bx r10
	//b _btest
	//b _thumbbranch


loadstoremult:
	ldmia r0, { r0-r7 }
	ldmia r7, { r0-r7 }
	stmia r0, { r1-r7 }
	stmia r7, { r0-r6 }

loadstore:
	ldr r0, [r1, #0x78]
	ldr r0, [r1, r2]
	ldr r0, [pc, #0x3f8]
	ldr r0, [sp, #0x3f8]
	ldrb r0, [r1, #0x1f]
	ldrb r0, [r1, r2]
	ldrh r0, [r1, #0x18]
	ldrh r0, [r1, r2]
	ldrsb r0, [r1, r2]
	ldrsh r0, [r1, r2]
	str r0, [r1, #0x78]
	str r0, [r1, r2]
	str r0, [sp, #0x3f8]
	strb r0, [r1, #0x1f]
	strb r0, [r1, r2]
	strh r0, [r1, #0x18]
	strh r0, [r1, r2]

pushpop:
	pop { r0-r7 }
	push { r0-r7 }


