use crate::opcodes::OpCode;

use super::CpuFlags;
use super::CPU;
pub mod test;

pub trait StackOpCodes {
    /*Stack related */
    fn pha(&mut self);
    fn php(&mut self);

    /*END Stack related */
    fn handle_stack_ops(&mut self, opcode: &OpCode, code: u8);
}

impl StackOpCodes for CPU {
    /*Stack related */
    fn pha(&mut self) {
        self.stack_push(self.register_a);
    }
    fn php(&mut self) {
        let mut data = self.status;
        data.insert(CpuFlags::BREAK);
        data.insert(CpuFlags::BREAK2);
        self.stack_push(data.bits());
    }

    /*END Stack related */

    fn handle_stack_ops(&mut self, opcode: &OpCode, code: u8) {
        match code {
            /*Stack related */
            0x48 => self.pha(),
            0x08 => self.php(),

            /*END Stack related */
            _ => return,
        }
    }
}
