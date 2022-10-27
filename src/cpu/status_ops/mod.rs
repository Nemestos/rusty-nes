use crate::{cpu::CpuFlags, opcodes::OpCode};

use super::CPU;
pub mod test;

pub trait StatusOpCodes {
    /*Status register */
    fn clc(&mut self);
    fn cld(&mut self);
    fn cli(&mut self);
    fn clv(&mut self);
    fn sec(&mut self);
    fn sed(&mut self);
    fn sei(&mut self);
    /*End Status register */
    fn handle_status_ops(&mut self, opcode: &OpCode, code: u8);
}

impl StatusOpCodes for CPU {
    /*Status register */
    fn clc(&mut self) {
        self.status.remove(CpuFlags::CARRY);
    }
    fn cld(&mut self) {
        self.status.remove(CpuFlags::DECIMAL_MODE);
    }
    fn cli(&mut self) {
        self.status.remove(CpuFlags::INTERRUPT_DISABLE);
    }
    fn clv(&mut self) {
        self.status.remove(CpuFlags::OVERFLOW);
    }

    fn sec(&mut self) {
        self.status.insert(CpuFlags::CARRY);
    }
    fn sed(&mut self) {
        self.status.insert(CpuFlags::DECIMAL_MODE);
    }
    fn sei(&mut self) {
        self.status.insert(CpuFlags::INTERRUPT_DISABLE);
    }
    /*End Status register */

    fn handle_status_ops(&mut self, _opcode: &OpCode, code: u8) {
        match code {
            /*Status Register */
            0x18 => self.clc(),
            0xd8 => self.cld(),
            0x58 => self.cli(),
            0xb8 => self.clv(),
            0x38 => self.sec(),
            0xf8 => self.sed(),
            0x78 => self.sei(),
            /*End Status Register */
            _ => return,
        }
    }
}
