use crate::opcodes::OpCode;

use super::AddressingMode;
use super::CPU;
pub mod test;

pub trait RegisterOpCodes {
    /*A,X,Y Registers */

    fn cpx(&mut self, mode: &AddressingMode);
    fn cpy(&mut self, mode: &AddressingMode);
    fn dex(&mut self);
    fn dey(&mut self);

    fn inx(&mut self);
    fn iny(&mut self);
    fn inc(&mut self, mode: &AddressingMode);

    fn lda(&mut self, mode: &AddressingMode);
    fn ldx(&mut self, mode: &AddressingMode);
    fn ldy(&mut self, mode: &AddressingMode);

    fn sta(&mut self, mode: &AddressingMode);
    fn stx(&mut self, mode: &AddressingMode);
    fn sty(&mut self, mode: &AddressingMode);

    fn tax(&mut self);
    fn tay(&mut self);

    fn txa(&mut self);
    fn tya(&mut self);

    fn tsx(&mut self);
    fn txs(&mut self);

    /*End A,X,Y Registers */

    fn handle_register_ops(&mut self, opcode: &OpCode, code: u8);
}

impl RegisterOpCodes for CPU {
    /*A,X,Y Registers */

    fn cpx(&mut self, mode: &AddressingMode) {
        self.compare_handle(mode, self.register_x);
    }
    fn cpy(&mut self, mode: &AddressingMode) {
        self.compare_handle(mode, self.register_y);
    }

    fn dex(&mut self) {
        let result = self.register_x.wrapping_sub(1);
        self.register_x = result;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn dey(&mut self) {
        let result = self.register_y.wrapping_sub(1);
        self.register_y = result;
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }
    fn iny(&mut self) {
        self.register_y = self.register_y.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn inc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        let result = data.wrapping_add(1);
        self.mem_write(addr, result);
        self.update_zero_and_negative_flags(result);
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn ldx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_x = value;
        self.update_zero_and_negative_flags(self.register_x);
    }
    fn ldy(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_y = value;
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }

    fn stx(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_x);
    }

    fn sty(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_y);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }
    fn tay(&mut self) {
        self.register_y = self.register_a;
        self.update_zero_and_negative_flags(self.register_y);
    }

    fn txa(&mut self) {
        self.register_a = self.register_x;
        self.update_zero_and_negative_flags(self.register_a);
    }
    fn tya(&mut self) {
        self.register_a = self.register_y;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tsx(&mut self) {
        self.register_x = self.stack_ptr;
        self.update_zero_and_negative_flags(self.register_x);
    }
    fn txs(&mut self) {
        self.stack_ptr = self.register_x;
    }

    /*End A,X,Y Registers */

    fn handle_register_ops(&mut self, opcode: &OpCode, code: u8) {
        match code {
            /* A,X,Y Registers */
            0xe0 | 0xe4 | 0xec => {
                self.cpx(&opcode.mode);
            }
            0xc0 | 0xc4 | 0xcc => {
                self.cpy(&opcode.mode);
            }
            0xca => {
                self.dex();
            }
            0x88 => {
                self.dey();
            }
            0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1 => {
                self.lda(&opcode.mode);
            }

            0xa2 | 0xa6 | 0xb6 | 0xae | 0xbe => {
                self.ldx(&opcode.mode);
            }
            0xa0 | 0xa4 | 0xb4 | 0xac | 0xbc => {
                self.ldy(&opcode.mode);
            }

            0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => {
                self.sta(&opcode.mode);
            }
            0x86 | 0x96 | 0x8e => {
                self.stx(&opcode.mode);
            }
            0x84 | 0x94 | 0x8c => {
                self.sty(&opcode.mode);
            }

            0xAA => self.tax(),
            0xA8 => self.tay(),

            0x8a => self.txa(),
            0x98 => self.tya(),

            0xba => self.tsx(),
            0x9a => self.txs(),

            0xe8 => self.inx(),
            0xc8 => self.iny(),
            0xe6 | 0xf6 | 0xee | 0xFE => self.inc(&opcode.mode),

            /*End A,X,Y Registers */
            _ => return,
        }
    }
}
