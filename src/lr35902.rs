use crate::register::{Reg16};

pub struct LR35902 {
    pub cycles: i64, 

    pub af: Reg16,
    pub bc: Reg16,
    pub de: Reg16,
    pub hl: Reg16,

    pub sp: Reg16,
    pub pc: Reg16
}

impl LR35902 {
    pub fn new() -> Self {
        return LR35902 {
            cycles: 0,

            af: Reg16::new(),
            bc: Reg16::new(),
            de: Reg16::new(),
            hl: Reg16::new(),
            
            sp: Reg16::new(),
            pc: Reg16::new()
        }
    }
}