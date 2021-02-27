.global _x86_64_lgdt
.p2align 4
_x86_64_lgdt:
    lgdt (%rdi)
    retq