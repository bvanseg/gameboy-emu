use crate::lr35902::LR35902;

impl LR35902 {
    pub fn x00_nop(&mut self) {
        self.cycles += 4;
    }

    pub fn x01_ld_bc_imm16(&mut self) {
        let upper = (self.fetch() as u16) << 8;
        let lower = self.fetch() as u16;
        self.bc.set(upper | lower);
        self.cycles += 12;
    }

    pub fn x03_inc_bc(&mut self) {
        self.bc.add(1);
        self.cycles += 8;
    }

    pub fn x04_inc_b(&mut self) {
        let snapshot = self.bc.upper;
        self.bc.upper += 1;

        // Handle flags.
        self.check_z_flag(self.bc.upper as u16);
        self.set_n_flag(false);
        self.check_h_flag(true, snapshot as u16, 1);

        self.cycles += 4;
    }

    pub fn x05_dec_b(&mut self) {
        let snapshot = self.bc.upper;
        self.bc.upper -= 1;

        // Handle flags.
        self.check_z_flag(self.bc.upper as u16);
        self.set_n_flag(true);
        self.check_h_flag(false, snapshot as u16, 1);

        self.cycles += 4;
    }

    pub fn x06_ld_b_imm8(&mut self) {
        let next = self.fetch();
        self.bc.upper = next;
        self.cycles += 8;
    }
}