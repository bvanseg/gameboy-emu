use crate::lr35902::LR35902;

impl LR35902 {
    pub fn exec_x00_x0f(&mut self, opcode: u8) {
        match opcode {
            0x00 => self.x00_nop(),
            0x01 => self.x01_ld_bc_imm16(),
            0x02 => self.x02_ld_bc_a(),
            0x03 => self.x03_inc_bc(),
            0x04 => self.x04_inc_b(),
            0x05 => self.x05_dec_b(),
            0x06 => self.x06_ld_b_imm8(),
            0x07 => self.x07_rlc_a(),
            0x08 => self.x08_ld_a16_sp(),
            0x09 => self.x09_add_hl_bc(),
            0x0a => self.x0a_ld_a_bc(),
            0x0b => self.x0b_dec_bc(),
            0x0c => self.x0c_inc_c(),
            0x0d => self.x0d_dec_c(),
            0x0e => self.x0e_ld_c_imm8(),
            0x0f => self.x0f_rrc_a(),
            _ => panic!("Unexpected opcode (0x00 - 0x0F): {}", opcode)
        }
    }

    pub fn x00_nop(&mut self) {
        self.cycles += 4;
    }

    pub fn x01_ld_bc_imm16(&mut self) {
        let upper = (self.fetch() as u16) << 8;
        let lower = self.fetch() as u16;
        self.bc.set(upper | lower);

        self.cycles += 12;
    }

    pub fn x02_ld_bc_a(&mut self) {
        // TODO: Move value in A register into memory location pointed to by pointer in BC register.
        self.cycles += 8;
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

    pub fn x07_rlc_a(&mut self) {
        // TODO: Rotate left circular A register
        self.cycles += 4;
    }

    pub fn x08_ld_a16_sp(&mut self) {
        // TODO:
        self.cycles += 20;
    }

    pub fn x09_add_hl_bc(&mut self) {
        let snapshot = self.hl.get();
        self.hl.add(self.bc.get());

        // Handle flags.
        self.set_n_flag(false);
        self.check_h_flag(true, snapshot, self.bc.get());
        // TODO: Check carry flag.

        self.cycles += 8;
    }

    pub fn x0a_ld_a_bc(&mut self) {
        // TODO:
        // BC is a pointer, dereference and load byte value from memory into A register.
        self.cycles += 8;
    }

    pub fn x0b_dec_bc(&mut self) {
        self.bc.sub(1);
        self.cycles += 8;
    }

    pub fn x0c_inc_c(&mut self) {
        let snapshot = self.bc.lower;
        self.bc.lower += 1;

        self.check_z_flag(self.bc.lower as u16);
        self.set_n_flag(false);
        self.check_h_flag(true, snapshot as u16, 1);

        self.cycles += 4;
    }

    pub fn x0d_dec_c(&mut self) {
        let snapshot = self.bc.lower;
        self.bc.lower -= 1;

        self.check_z_flag(self.bc.lower as u16);
        self.set_n_flag(true);
        self.check_h_flag(false, snapshot as u16, 1);

        self.cycles += 4;
    }

    pub fn x0e_ld_c_imm8(&mut self) {
        let next = self.fetch();
        self.bc.lower = next;

        self.cycles += 4;
    }

    pub fn x0f_rrc_a(&mut self) {
        // TODO: Rotate right circular A register
        self.cycles += 4;
    }
}