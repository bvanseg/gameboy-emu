use crate::lr35902::LR35902;

impl LR35902 {
    pub fn x50_ld_d_b(&mut self) {
        self.de.upper = self.bc.upper;
        self.cycles += 4;
    }

    pub fn x51_ld_d_c(&mut self) {
        self.de.upper = self.bc.lower;
        self.cycles += 4;
    }

    pub fn x52_ld_d_d(&mut self) {
        self.de.upper = self.de.upper;
        self.cycles += 4;
    }

    pub fn x53_ld_d_e(&mut self) {
        self.de.upper = self.de.lower;
        self.cycles += 4;
    }

    pub fn x54_ld_d_h(&mut self) {
        self.de.upper = self.hl.upper;
        self.cycles += 4;
    }

    pub fn x55_ld_d_l(&mut self) {
        self.de.upper = self.hl.lower;
        self.cycles += 4;
    }

    pub fn x56_ld_d_hl(&mut self) {
        // TODO:
        // HL holds a pointer
        // Load the byte value from RAM that the HL pointer points to
        // Set D to that byte value.
        self.cycles += 8;
    }

    pub fn x57_ld_d_a(&mut self) {
        self.de.upper = self.af.upper;
        self.cycles += 4;
    }

    pub fn x58_ld_e_b(&mut self) {
        self.de.lower = self.bc.upper;
        self.cycles += 4;
    }

    pub fn x59_ld_e_c(&mut self) {
        self.de.lower = self.bc.lower;
        self.cycles += 4;
    }

    pub fn x5a_ld_e_d(&mut self) {
        self.de.lower = self.de.upper;
        self.cycles += 4;
    }

    pub fn x5b_ld_e_e(&mut self) {
        self.de.lower = self.de.lower;
        self.cycles += 4;
    }

    pub fn x5c_ld_e_h(&mut self) {
        self.de.lower = self.hl.upper;
        self.cycles += 4;
    }

    pub fn x5d_ld_e_l(&mut self) {
        self.de.lower = self.hl.lower;
        self.cycles += 4;
    }

    pub fn x5e_ld_e_hl(&mut self) {
        // TODO:
        // HL holds a pointer
        // Load the byte value from RAM that the HL pointer points to
        // Set E to that byte value.
        self.cycles += 8;
    }

    pub fn x5f_ld_e_a(&mut self) {
        self.de.lower = self.af.upper;
        self.cycles += 4;
    }
}