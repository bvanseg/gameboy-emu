use crate::lr35902::LR35902;

impl LR35902 {
    pub fn exec_x40_x4f(&mut self, opcode: u8) {
        match opcode {
            0x40 => self.x40_ld_b_b(),
            0x41 => self.x41_ld_b_c(),
            0x42 => self.x42_ld_b_d(),
            0x43 => self.x43_ld_b_e(),
            0x44 => self.x44_ld_b_h(),
            0x45 => self.x45_ld_b_l(),
            0x46 => self.x46_ld_b_hl(),
            0x47 => self.x47_ld_b_a(),
            0x48 => self.x48_ld_c_b(),
            0x49 => self.x49_ld_c_c(),
            0x4a => self.x4a_ld_c_d(),
            0x4b => self.x4b_ld_c_e(),
            0x4c => self.x4c_ld_c_h(),
            0x4d => self.x4d_ld_c_l(),
            0x4e => self.x4e_ld_c_hl(),
            0x4f => self.x4f_ld_c_a(),
            _ => panic!("Unexpected opcode (0x40 - 0x4F): {}", opcode)
        }
    }

    pub fn x40_ld_b_b(&mut self) {
        self.bc.upper = self.bc.upper;
        self.cycles += 4;
    }

    pub fn x41_ld_b_c(&mut self) {
        self.bc.upper = self.bc.lower;
        self.cycles += 4;
    }

    pub fn x42_ld_b_d(&mut self) {
        self.bc.upper = self.de.upper;
        self.cycles += 4;
    }

    pub fn x43_ld_b_e(&mut self) {
        self.bc.upper = self.de.lower;
        self.cycles += 4;
    }

    pub fn x44_ld_b_h(&mut self) {
        self.bc.upper = self.hl.upper;
        self.cycles += 4;
    }

    pub fn x45_ld_b_l(&mut self) {
        self.bc.upper = self.hl.lower;
        self.cycles += 4;
    }

    pub fn x46_ld_b_hl(&mut self) {
        // TODO:
        // HL holds a pointer
        // Load the byte value from RAM that the HL pointer points to
        // Set B to that byte value.
        self.cycles += 8;
    }

    pub fn x47_ld_b_a(&mut self) {
        self.bc.upper = self.af.upper;
        self.cycles += 4;
    }

    pub fn x48_ld_c_b(&mut self) {
        self.bc.lower = self.bc.upper;
        self.cycles += 4;
    }

    pub fn x49_ld_c_c(&mut self) {
        self.bc.lower = self.bc.lower;
        self.cycles += 4;
    }

    pub fn x4a_ld_c_d(&mut self) {
        self.bc.lower = self.de.upper;
        self.cycles += 4;
    }

    pub fn x4b_ld_c_e(&mut self) {
        self.bc.lower = self.de.lower;
        self.cycles += 4;
    }

    pub fn x4c_ld_c_h(&mut self) {
        self.bc.lower = self.hl.upper;
        self.cycles += 4;
    }

    pub fn x4d_ld_c_l(&mut self) {
        self.bc.lower = self.hl.lower;
        self.cycles += 4;
    }

    pub fn x4e_ld_c_hl(&mut self) {
        // TODO:
        // HL holds a pointer
        // Load the byte value from RAM that the HL pointer points to
        // Set C to that byte value.
        self.cycles += 8;
    }

    pub fn x4f_ld_c_a(&mut self) {
        self.bc.lower = self.af.upper;
        self.cycles += 4;
    }
}