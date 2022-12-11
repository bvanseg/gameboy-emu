use crate::lr35902::LR35902;

impl LR35902 {
    pub fn x16_ld_d_imm8(&mut self) {
        let next = self.fetch();
        self.de.upper = next;
        self.cycles += 8;
    }
}