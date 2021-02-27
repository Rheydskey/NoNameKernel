set_gdt:
    mov eax, [esp + 4]
    lgdt [eax]
    mov ax, 0x10 ; New data selector
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax
    mov ss, ax
    ret