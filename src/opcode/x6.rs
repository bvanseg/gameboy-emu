use crate::lr35902::LR35902;

impl LR35902 {
    pub fn x60_ld_h_b(&mut self) {
        self.hl.upper = self.bc.upper;
        self.cycles += 4;
    }

    pub fn x61_ld_h_c(&mut self) {
        self.hl.upper = self.bc.lower;
        self.cycles += 4;
    }

    pub fn x62_ld_h_d(&mut self) {
        self.hl.upper = self.de.upper;
        self.cycles += 4;
    }

    pub fn x63_ld_h_e(&mut self) {
        self.hl.upper = self.de.lower;
        self.cycles += 4;
    }

    pub fn x64_ld_h_h(&mut self) {
        self.hl.upper = self.hl.upper;
        self.cycles += 4;
    }

    pub fn x65_ld_h_l(&mut self) {
        self.hl.upper = self.hl.lower;
        self.cycles += 4;
    }

    pub fn x66_ld_h_hl(&mut self) {
        // TODO:
        // HL holds a pointer
        // Load the byte value from RAM that the HL pointer points to
        // Set H to that byte value.
        self.cycles += 8;
    }

    pub fn x67_ld_h_a(&mut self) {
        self.hl.upper = self.af.upper;
        self.cycles += 4;
    }

    pub fn x68_ld_l_b(&mut self) {
        self.hl.lower = self.bc.upper;
        self.cycles += 4;
    }

    pub fn x69_ld_l_c(&mut self) {
        self.hl.lower = self.bc.lower;
        self.cycles += 4;
    }

    pub fn x6a_ld_l_d(&mut self) {
        self.hl.lower = self.de.upper;
        self.cycles += 4;
    }

    pub fn x6b_ld_l_e(&mut self) {
        self.hl.lower = self.de.lower;
        self.cycles += 4;
    }

    pub fn x6c_ld_l_h(&mut self) {
        self.hl.lower = self.hl.upper;
        self.cycles += 4;
    }

    pub fn x6d_ld_l_l(&mut self) {
        self.hl.lower = self.hl.lower;
        self.cycles += 4;
    }

    pub fn x6e_ld_l_hl(&mut self) {
        // TODO:
        // HL holds a pointer
        // Load the byte value from RAM that the HL pointer points to
        // Set L to that byte value.
        self.cycles += 8;
    }

    pub fn x6f_ld_l_a(&mut self) {
        self.hl.lower = self.af.upper;
        self.cycles += 4;
    }
}