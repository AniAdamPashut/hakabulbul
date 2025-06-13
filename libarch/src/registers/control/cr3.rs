use bitflags::bitflags;


// Go to ./docs/control registers.md
bitflags! {
    pub struct CR3 : u64 {
        const PWT = 1 << 3;
        const PCD = 1 << 4;
        const LAM_U57 = 1 << 61;
        const LAM_U48 = 1 << 62;
    }
}