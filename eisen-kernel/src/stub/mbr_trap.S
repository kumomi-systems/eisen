.code16

.section .mbrtrap, "a"
mbr_trap_start:
  # Clear blinking cursor
  movw    $0x1003, %ax
  movw    $0x0000, %bx
  int     $0x10

  # Print trap message
  mov     mbr_trap_message, %si
  movb    $0x0E, %ah
  movb    $0x07, %bl
  .printstr:
    movb    (%si), %al
    cmpb    $0, %al
    je      .printstr_end
    int     $0x10
    inc     %si
    jmp     .printstr
  .printstr_end:

  # Actual trap
  jmp .

mbr_trap_message:
  .asciz "Eisen must be booted from 64-bit UEFI!"

.space 510-(.-mbr_trap_start)
.word 0xAA55
