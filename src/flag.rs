use crate::lr35902::LR35902;

pub enum Flag {
    Z = 7,
    N = 6,
    H = 5,
    C = 4
}

impl LR35902 {
    pub fn get_flag(&self, flag: Flag) -> bool {
        return (self.af.lower >> flag as u8) & 0b000000001 == 1;
    }

    pub fn set_flag(&mut self, flag: Flag, value: bool) {
        match value {
            true => self.af.lower |= 1 << flag as u8,
            false => self.af.lower &= !(1 << flag as u8)
        }
    }

    pub fn get_z_flag(&self) -> bool {
        return self.get_flag(Flag::Z);
    }

    pub fn get_n_flag(&self) -> bool {
        return self.get_flag(Flag::N);
    }

    pub fn get_h_flag(&self) -> bool {
        return self.get_flag(Flag::H);
    }

    pub fn get_c_flag(&self) -> bool {
        return self.get_flag(Flag::C);
    }

    pub fn set_z_flag(&mut self, value: bool) {
        self.set_flag(Flag::Z, value);
    }

    pub fn set_n_flag(&mut self, value: bool) {
        self.set_flag(Flag::N, value);
    }

    pub fn set_h_flag(&mut self, value: bool) {
        self.set_flag(Flag::H, value);
    }

    pub fn set_c_flag(&mut self, value: bool) {
        self.set_flag(Flag::C, value);
    }

    pub fn check_z_flag(&mut self, value: u16) {
        self.set_z_flag(value == 0);
    }

    pub fn check_h_flag(&mut self, add: bool, a: u16, b: u16) {

        let did_half_carry = if add {
            (((a & 0xf) + (b & 0xf)) & 0x10) == 0x10 // Addition
        } else {
            ((a as i16) & 0x0F) - ((b as i16) & 0x0F) < 0 // Subtraction
        };

        self.set_h_flag(did_half_carry);
    }
}