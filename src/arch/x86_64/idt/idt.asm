global _x86_64_lidt
_x86_64_lidt:
    lidt [rdi]
    ret