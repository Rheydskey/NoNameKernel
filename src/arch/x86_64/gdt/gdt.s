global _x86_64_lgdt
_x86_64_lgdt:
    lgdt [rdi]
    retq