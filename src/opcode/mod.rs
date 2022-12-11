use std::process::exit;

use crate::lr35902::LR35902;

mod x0;
mod x1;
mod x2;
mod x3;
mod x4;
mod x5;
mod x6;
mod x7;

impl LR35902 {
    fn fetch(&mut self) -> u8 {
        self.pc.add(1);
        // TODO: return opcode @ program counter.
        return 0;
    }

    fn exec(&mut self, opcode: u8) {
        match opcode {
            0x00 => self.x00_nop(),
            0x01 => self.x01_ld_bc_imm16(),
            0x02 => {}, // TODO:
            0x03 => self.x03_inc_bc(),
            0x04 => self.x04_inc_b(),
            0x05 => self.x05_dec_b(),
            0x06 => self.x06_ld_b_imm8(),
            0x10 => exit(0),

            0x16 => self.x16_ld_d_imm8(),

            0x26 => self.x26_ld_h_imm8(),

            _ => println!("Unknown opcode detected!")
        }
    }
}