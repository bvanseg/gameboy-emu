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
            0x00..=0x0f => self.exec_x00_x0f(opcode),
            0x10..=0x1f => self.exec_x10_x1f(opcode),

            0x26 => self.x26_ld_h_imm8(),

            0x40..=0x4f => self.exec_x40_x4f(opcode),

            _ => println!("Unknown opcode detected!")
        }
    }
}