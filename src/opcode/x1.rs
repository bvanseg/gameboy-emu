use crate::lr35902::LR35902;

impl LR35902 {
    pub fn exec_x10_x1f(&mut self, opcode: u8) {
        match opcode {
            0x10 => self.x10_stop(),
            0x11 => self.x11_ld_de_imm16(),
            0x12 => self.x12_ld_de_a(),
            0x13 => self.x13_inc_de(),
            // 0x14 => self.x14(),
            // 0x15 => self.x15(),
            0x16 => self.x16_ld_d_imm8(),
            // 0x17 => self.x17(),
            // 0x18 => self.x18(),
            // 0x19 => self.x19(),
            // 0x1a => self.x1a(),
            // 0x1b => self.x1b(),
            // 0x1c => self.x1c(),
            // 0x1d => self.x1d(),
            // 0x1e => self.x1e(),
            // 0x1f => self.x1f(),
            _ => panic!("Unexpected opcode (0x10 - 0x1F): {}", opcode)
        }
    }

    pub fn x10_stop(&mut self) {
        self.fetch(); // stop consumes the next byte.
        self.cycles += 4;
    }

    pub fn x11_ld_de_imm16(&mut self) {
        let first = self.fetch();
        let second = self.fetch();
        let value = ((first << 8) as u16) | (second as u16);
        self.de.set(value);

        self.cycles += 12;
    }

    pub fn x12_ld_de_a(&mut self) {
        // TODO: Move value in A register into memory location pointed to by pointer in DE register.
        self.cycles += 8;
    }

    pub fn x13_inc_de(&mut self) {
        self.de.add(1);
        self.cycles += 8;
    }

    pub fn x16_ld_d_imm8(&mut self) {
        let next = self.fetch();
        self.de.upper = next;
        self.cycles += 8;
    }
}