_start:
	mrs r0, cpsr
	mrs r0, spsr
	
	msr cpsr_c, r0
	msr cpsr_c, #0xdb
	

