# Control Registers
> Information is brought from the Volume 3A, Ch. 2 of the Intel 64 and IA-32 Software Developer's Manual  

## CR0
`CR0` is a register whos purpose is to contain system control flags that affect the operating mode and states of the processor 

A word on the `WAIT/FWAIT` instruction. This instruction causes the processor to check for and handle pending unmansked floating-point exceptions before proceeding. `FWAIT` is just an alternative mnemonic for the `WAIT` instruction. This instruction throws a `#NM` exception when CR0.MP and CR0.TS are set. [For Further Reading](https://www.felixcloutier.com/x86/wait:fwait)

### flags:

- CR0.PE | "Protection Enabled" - When this bit is set protected mode is enabled. NOTE: this does not enable paging directly, it requires the `PG` also be set. When this flag is set without the `PG` flag, only segment-level protection is enabled.
- CR0.MP | "Monitor Co-Processor" - Controls the interactions of the `WAIT/FWAIT` instructions with the `TS` flag. If this flag is clear, the value of the `TS` flag is utterly ignored and the `WAIT/FWAIT` instructions will execute. When set with the `TS` flag the `WAIT/FWAIT` instructions will trigger a `#NM` Exception (device-not-available)
- CR0.EM | "Emulation" - Indicates that the processor that doesn't have a x87 FPU when set. When clear, an FPU is present. NOTE: when set MMX instructions and SSE/SSE2/SSE3/SSSE3/SSE4 extensions will throw a `#UD` exepction (invalid-opcode). On any modern 32bit or 64bit CPU this should be enabled at all times.
- CR0.TS | "Task Switched" - Allows for a lazy loading of the x87FPU/MMX/SSE/SSE2/SSE3/SSSE3/SSE4 context until the instruction is actually executed. If the `EM` flag is set this flag has no effect. If the `MP` flag is set this flag has no effect also. In both cases a `#NM` execption would be thrown
- CR0.ET | "Extension type" - Reserved in some old processors. Read only and set to 1.
- CR0.NE | "Numeric Error" - allows the reporting of x87 FPU errors 
- CR0.WP | "Write Protect" - When clear, allows supervisor-level to write into read-only pages.
- CR0.AM | "Alignment Mask" - When set, automatic alignment checking is enabled. Disabled when clear. NOTE: Alignment checking will only occur if the AC flag (in RFLAGS) is also set, the CPL is 3 and the processor is in protected mode or virtual-8086 mode.
- CR0.NW | "Not Write-through" - Together with the `CD` flag define the cache settings. When both are clear the cache is in normal mode. This flag cannot be set when the `CD` flag is clear, will trigger the general-protection interrupt.
- CR0.CD | "Cache Disable" - Changes how the processor interacts with his internal and external caches with `NW`.
- CR0.PG | "Paging" - Enables paging. When the `PE` flag is clear this flag has no effect, moreover, when the `PE` flag is clear and this flag is set a general-protection `#GP` exception is thrown

## CR1

This register is reserved and should not be used at all

## CR2

Contains the linear address that caused a page-fault.

## CR3

- CR3.PWT | "Page-level Write-Through" - Controls the memory type used to access the first paging structure of the current paging-structure hierarchy
- CR3.PCD | "Page-level Cache Disable" - Same as before. My guess is that their relationship is the same as the `CR0.NW` and `CR0.CD` flags.
- CR3.LAM_U57 | "User LAM57 Enable" - Enables the Linear Address Masking for user pointers between 57..62. Overrides `LAM_U48`
- CR3.LAM_U48 | "User LAM48 Enable" - Same as the `LAM_U57` but the bit range is 48..62.