use crate::lr35902::LR35902;

impl LR35902 {
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