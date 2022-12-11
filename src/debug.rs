
use crate::lr35902::LR35902;

impl LR35902 {
    pub fn dump_state(&self) {
        println!();
        println!("============= LR35902 STATE =============");
        println!("=== REGISTERS ===");
        println!("AF: {}, (A: {}, F: {})", self.af.get(), self.af.upper, self.af.lower);
        println!("BC: {}, (B: {}, C: {})", self.bc.get(), self.bc.upper, self.bc.lower);
        println!("DE: {}, (D: {}, E: {})", self.de.get(), self.de.upper, self.de.lower);
        println!("HL: {}, (H: {}, L: {})", self.hl.get(), self.hl.upper, self.hl.lower);
        println!();
        println!("SP: {}", self.sp.get());
        println!("PC: {}", self.pc.get());
        println!();
        println!("=== FLAGS ===");
        println!("Z: {}, N: {}, H: {}, C: {}", self.get_z_flag(), self.get_n_flag(), self.get_h_flag(), self.get_c_flag());
        println!("=========================================");
    }
}