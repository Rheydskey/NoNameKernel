;; GDT Assembly Code of WingOS (Same name on github)

bits 64
global _x86_64_lgdt
_x86_64_lgdt:
  push rbp
  mov rbp, rsp
  lgdt [rdi]
  mov rax, 16
  mov ss, rax
  mov ds, rax
  mov es, rax
  mov rax, qword .trampoline
  push rsi
  push rax
  o64 retf

.trampoline:
  pop rbp
  ret