
pub struct Reg16 {
    pub upper: u8,
    pub lower: u8
}

impl Reg16 {
    pub fn new() -> Self {
        return Reg16 { upper: 0, lower: 0 }
    }

    pub fn get(&self) -> u16 {
        return ((self.upper as u16) << 8) + ((self.lower as u16) << 0);
    }

    pub fn set(&mut self, value: u16) {
        self.upper = (value >> 8) as u8;
        self.lower = (value >> 0) as u8;
    }

    pub fn add(&mut self, value: u16) {
        self.set(self.get() + value);
    }

    pub fn sub(&mut self, value: u16) {
        self.set(self.get() - value);
    }
}