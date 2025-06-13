pub struct CR8(u64);

impl CR8 {
    pub fn priority(&self) -> u8 {
        let mask: u64 = 0b1111;
        return (self.0 & mask) as u8;
    }
}