## Eisen Operating System
## Copyright (C) 2025  Kumomi Systems
## 
## This program is free software: you can redistribute it and/or modify
## it under the terms of the GNU General Public License as published by
## the Free Software Foundation, either version 3 of the License, or
## (at your option) any later version.
## 
## This program is distributed in the hope that it will be useful,
## but WITHOUT ANY WARRANTY; without even the implied warranty of
## MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
## GNU General Public License for more details.
## 
## You should have received a copy of the GNU General Public License
## along with this program. If not, see <https://www.gnu.org/licenses/>.

.code64
.text

.macro pushstate
  pushq %rax
  pushq %rbx
  pushq %rcx
  pushq %rdx
  pushq %rsi
  pushq %rdi
  pushq %rbp
  pushq %r8
  pushq %r9
  pushq %r10
  pushq %r11
  pushq %r12
  pushq %r13
  pushq %r14
  pushq %r15
.endm

.macro popstate
  popq  %r15
  popq  %r14
  popq  %r13
  popq  %r12
  popq  %r11
  popq  %r10
  popq  %r9
  popq  %r8
  popq  %rbp
  popq  %rdi
  popq  %rsi
  popq  %rdx
  popq  %rcx
  popq  %rbx
  popq  %rax
.endm

.altmacro
.macro isr_no_error number
  .global isr\number
  .align 0x10
  isr\number:
    cli
    push  $0x00
    push  $\number
    .iflt \number-0x20
      jmp isr_common
    .else
      jmp irq_common
    .endif
.endm

.macro isr_error number
  .global isr\number
  .align 0x10
  isr\number:
    cli
    nop
    nop
    push  $\number
    jmp   isr_common
.endm

.macro isr number
  .quad   isr\number
.endm

.global ISR_TABLE
.align 0x10
ISR_TABLE:
  .set x, 0
  .rept 256
    .quad isr0 + x
    .set  x, x+0x10
  .endr

.extern isr_handler
.align 0x10
isr_common:
  cld
  pushstate

  xor   %rdi, %rdi
  movb  120(%rsp), %dil
  movq  %rsp, %rsi
  call  isr_handler

  movq  %rax, %rsp

  popstate
  add   $0x10, %rsp
  iretq

.extern irq_handler
.align 0x10
irq_common:
  pushstate

  xor   %rdi, %rdi
  movb  120(%rsp), %dil
  movq  %rsp, %rsi
  call irq_handler

  popstate
  add   $0x10, %rsp
  sti
  iretq

isr_no_error  0
isr_no_error  1
isr_no_error  2
isr_no_error  3
isr_no_error  4
isr_no_error  5
isr_no_error  6
isr_no_error  7
isr_error     8
isr_no_error  9
isr_error     10
isr_error     11
isr_error     12
isr_error     13
isr_error     14
isr_no_error  15
isr_no_error  16
isr_error     17
isr_no_error  18
isr_no_error  19
isr_no_error  20
isr_no_error  21
isr_no_error  22
isr_no_error  23
isr_no_error  24
isr_no_error  25
isr_no_error  26
isr_no_error  27
isr_no_error  28
isr_no_error  29
isr_error     30
isr_no_error  31

.set x, 32
.rept 256-32
  isr_no_error %x
  .set x, x+1
.endr