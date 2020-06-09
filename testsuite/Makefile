all:
	arm-none-eabi-gcc -mbig-endian -mcpu=arm926ej-s \
		arm_test.c -o arm_test.elf
	arm-none-eabi-objcopy -j .text -O binary arm_test.elf arm_test.bin

	arm-none-eabi-as -mbig-endian -mcpu=arm926ej-s \
		arm_misc_test.S -o arm_misc_test.o
	arm-none-eabi-objcopy -j .text -O binary arm_misc_test.o arm_misc_test.bin

	arm-none-eabi-as -mbig-endian -mcpu=arm926ej-s \
		arm_decode_test.S -o arm_decode_test.o
	arm-none-eabi-objcopy -j .text -O binary arm_decode_test.o arm_decode_test.bin


clean:
	rm -vf *.elf *.bin *.o
