use crate::lr35902::LR35902;

impl LR35902 {
    pub fn x26_ld_h_imm8(&mut self) {
        let next = self.fetch();
        self.hl.upper = next;
        
        self.cycles += 8;
    }
}