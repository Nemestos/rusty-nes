use crate::opcodes::OpCode;

use super::CPU;
pub mod test;

pub trait OtherOpCodes {
    /*Other Flow */
    fn nop(&mut self);
    fn rti(&mut self);

    /*END Other Flow */
    fn handle_other_ops(&mut self, opcode: &OpCode, code: u8);
}

impl OtherOpCodes for CPU {
    /*Other Flow */
    fn nop(&mut self) {
        self.program_counter = self.program_counter.wrapping_add(1);
    }
    fn rti(&mut self) {
        let status = self.stack_pull();
        self.status.bits = status & 0b1101_1111;

        let pc = self.stack_pull_u16();
        self.program_counter = pc;
    }

    /*END Other Flow */

    fn handle_other_ops(&mut self, _opcode: &OpCode, code: u8) {
        match code {
            /*Other Flow */
            0xea => self.nop(),
            0x40 => self.rti(),

            /*END Other Flow */
            _ => return,
        }
    }
}
