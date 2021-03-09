;; GDT Assembly Code of WingOS (Same name on github)

global _x86_64_lgdt
_x86_64_lgdt:
  push rbp
  mov rbp, rsp
  lgdt [rdi]
  mov ax, 0x10
  mov ss, ax
  mov ds, ax
  mov es, ax
  mov rax, qword .trampoline
  push rsi
  push rax
  o64 retf

.trampoline:
  pop rbp
  ret
