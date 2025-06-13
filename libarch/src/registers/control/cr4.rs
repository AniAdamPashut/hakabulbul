use bitflags::bitflags;


// Go to ./docs/control registers.md
bitflags! {
    pub struct CR4 : u64 {
        const VME = 1 << 0;
        const PVI = 1 << 1;
        const TSD = 1 << 2;
        const DE = 1 << 3;
        const PSE = 1 << 4;
        const PAE = 1 << 5;
        const MCE = 1 << 6;
        const PGE = 1 << 7;
        const PCE = 1 << 8;
        const OSFXSR = 1 << 9;
        const OSXMMEXCPT = 1 << 10;
        const UMIP = 1 << 11;
        const LA57 = 1 << 12;
        const VMXE = 1 << 13;
        const SMXE = 1 << 14;
        const FSGSBASE = 1 << 16;
        const PCIDE = 1 << 17;
        const OSXSAVE = 1 << 18;
        const KL = 1 << 19;
        const SMEP = 1 << 20;
        const SMAP = 1 << 21;
        const PKE = 1 << 22;
        const CET = 1 << 23;
        const PKS = 1 << 24;
        const UINTR = 1 << 25;
        const LAM_SUP = 1 << 28;
    }
}