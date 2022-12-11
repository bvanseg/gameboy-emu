use crate::lr35902::LR35902;

impl LR35902 {
    pub fn x70_ld_hl_b(&mut self) {
        // TODO:
        // HL represents a pointer
        // Move the byte value in the register into RAM.
        self.cycles += 8;
    }

    pub fn x71_ld_hl_c(&mut self) {
        // TODO:
        // HL represents a pointer
        // Move the byte value in the register into RAM.
        self.cycles += 8;
    }

    pub fn x72_ld_hl_d(&mut self) {
        // TODO:
        // HL represents a pointer
        // Move the byte value in the register into RAM.
        self.cycles += 8;
    }

    pub fn x73_ld_hl_e(&mut self) {
        // TODO:
        // HL represents a pointer
        // Move the byte value in the register into RAM.
        self.cycles += 8;
    }

    pub fn x74_ld_hl_h(&mut self) {
        // TODO:
        // HL represents a pointer
        // Move the byte value in the register into RAM.
        self.cycles += 8;
    }

    pub fn x75_ld_hl_l(&mut self) {
        // TODO:
        // HL represents a pointer
        // Move the byte value in the register into RAM.
        self.cycles += 8;
    }

    pub fn x76_halt(&mut self) {
        // TODO: Halt
        self.cycles += 4;
    }

    pub fn x77_ld_hl_a(&mut self) {
        // TODO:
        // HL represents a pointer
        // Move the byte value in the register into RAM.
        self.cycles += 8;
    }

    pub fn x78_ld_a_b(&mut self) {
        self.af.upper = self.bc.upper;
        self.cycles += 4;
    }

    pub fn x79_ld_a_c(&mut self) {
        self.af.upper = self.bc.lower;
        self.cycles += 4;
    }

    pub fn x7a_ld_a_d(&mut self) {
        self.af.upper = self.de.upper;
        self.cycles += 4;
    }

    pub fn x7b_ld_a_e(&mut self) {
        self.af.upper = self.de.lower;
        self.cycles += 4;
    }

    pub fn x7c_ld_a_h(&mut self) {
        self.af.upper = self.hl.upper;
        self.cycles += 4;
    }

    pub fn x7d_ld_a_l(&mut self) {
        self.af.upper = self.hl.lower;
        self.cycles += 4;
    }

    pub fn x7e_ld_a_hl(&mut self) {
        // TODO:
        // HL represents a pointer
        // Move the byte value in RAM into the A register.
        self.cycles += 8;
    }

    pub fn x7f_ld_a_a(&mut self) {
        self.af.upper = self.af.upper;
        self.cycles += 4;
    }
}