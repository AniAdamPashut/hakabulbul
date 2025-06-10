# Interrupt Descriptor Table

The interrupte descriptor table is just a huge array compromised on pointers slices into parts for some reason

The table is 256 entries long (for the 256 interrupts). Entries below 32 are reserved for the cpu and special cases.
[further reading](https://wiki.osdev.org/Interrupt_Descriptor_Table)