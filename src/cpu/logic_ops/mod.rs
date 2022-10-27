use crate::cpu::CpuFlags;
use crate::mem::Mem;
use crate::opcodes::OpCode;

use super::AddressingMode;
use super::CPU;
pub mod test;

pub trait LogicOpCodes {
    /*Arithmetic Logic */
    fn adc(&mut self, mode: &AddressingMode);
    fn and(&mut self, mode: &AddressingMode);
    fn asl(&mut self, mode: &AddressingMode) -> u8;
    fn asl_acu(&mut self);
    fn bit(&mut self, mode: &AddressingMode);
    fn cmp(&mut self, mode: &AddressingMode);
    fn dec(&mut self, mode: &AddressingMode);
    fn eor(&mut self, mode: &AddressingMode);

    fn lsr(&mut self, mode: &AddressingMode) -> u8;
    fn lsr_acu(&mut self);

    fn ora(&mut self, mode: &AddressingMode);

    fn rol_acu(&mut self);
    fn rol(&mut self, mode: &AddressingMode);

    fn ror_acu(&mut self);
    fn ror(&mut self, mode: &AddressingMode);

    fn sbc(&mut self, mode: &AddressingMode);

    /*End Arithmetic Logic */
    fn handle_logic_ops(&mut self, opcode: &OpCode, code: u8);
}

impl LogicOpCodes for CPU {
    /*Arithmetic & Logic */
    fn adc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        self.add_to_register_a(data);
    }
    fn and(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        self.set_register_a(data & self.register_a);
    }
    fn asl_acu(&mut self) {
        let mut data = self.register_a;

        // if bit 7 is set
        if data >> 7 == 1 {
            self.set_carry();
        } else {
            self.remove_carry()
        }
        data = data << 1;
        self.set_register_a(data);
    }
    fn asl(&mut self, mode: &AddressingMode) -> u8 {
        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);

        // if bit 7 is set
        if data >> 7 == 1 {
            self.set_carry();
        } else {
            self.remove_carry()
        }
        data = data << 1;
        self.mem_write(addr, data);
        self.update_zero_and_negative_flags(data);
        data
    }
    fn bit(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        let and = self.register_a & data;
        self.status.set(CpuFlags::ZERO, and == 0);
        self.status.set(CpuFlags::NEGATIV, data & 0b1000_0000 > 0);
        self.status.set(CpuFlags::OVERFLOW, data & 0b0100_0000 > 0);
    }
    fn cmp(&mut self, mode: &AddressingMode) {
        self.compare_handle(mode, self.register_a);
    }
    fn dec(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        let result = data.wrapping_sub(1);
        self.mem_write(addr, result);
        self.update_zero_and_negative_flags(result);
    }
    fn eor(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        let result = self.register_a ^ data;
        self.register_a = result;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn lsr_acu(&mut self) {
        let mut data = self.register_a;

        // if bit 7 is set
        if data & 0b0000_0001 == 1 {
            self.set_carry();
        } else {
            self.remove_carry()
        }
        data = data >> 1;
        self.set_register_a(data);
    }
    fn lsr(&mut self, mode: &AddressingMode) -> u8 {
        let addr = self.get_operand_address(mode);
        let mut data = self.mem_read(addr);

        // if bit 7 is set
        if data & 0b0000_0001 == 1 {
            self.set_carry();
        } else {
            self.remove_carry()
        }
        data = data >> 1;
        self.mem_write(addr, data);
        self.update_zero_and_negative_flags(data);
        data
    }

    fn ora(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        let result = self.register_a | data;
        self.set_register_a(result);
    }

    fn rol_acu(&mut self) {
        let data = self.register_a;

        if data >> 7 == 1 {
            self.set_carry();
        } else {
            self.remove_carry();
        }

        let result = (data << 1) | (data >> 7);
        self.set_register_a(result);
    }
    fn rol(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        if data >> 7 == 1 {
            self.set_carry();
        } else {
            self.remove_carry();
        }

        let result = (data << 1) | (data >> 7);
        self.mem_write(addr, result);
    }

    fn ror_acu(&mut self) {
        let data = self.register_a;

        if data >> 7 == 1 {
            self.set_carry();
        } else {
            self.remove_carry();
        }

        let result = (data >> 1) | (data << 7);
        self.set_register_a(result);
    }
    fn ror(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);

        if data >> 7 == 1 {
            self.set_carry();
        } else {
            self.remove_carry();
        }

        let result = (data >> 1) | (data << 7);
        self.mem_write(addr, result);
    }

    fn sbc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(&mode);
        let data = self.mem_read(addr);
        self.add_to_register_a(((data as i8).wrapping_neg().wrapping_sub(1)) as u8);
    }
    /*End Arithmetic & Logic */

    fn handle_logic_ops(&mut self, opcode: &OpCode, code: u8) {
        match code {
            /* Arithmetic & Logic */
            0x69 | 0x65 | 0x75 | 0x6d | 0x7d | 0x79 | 0x61 | 0x71 => {
                self.adc(&opcode.mode);
            }
            0x29 | 0x25 | 0x35 | 0x2d | 0x3d | 0x39 | 0x21 | 0x31 => {
                self.and(&opcode.mode);
            }
            0x0a => self.asl_acu(),
            0x06 | 0x16 | 0x0e | 0x1e => {
                self.asl(&opcode.mode);
            }
            0x24 | 0x2c => {
                self.bit(&opcode.mode);
            }
            0xc9 | 0xc5 | 0xd5 | 0xcd | 0xdd | 0xd9 | 0xc1 | 0xd1 => {
                self.cmp(&opcode.mode);
            }
            0xc6 | 0xd6 | 0xce | 0xde => {
                self.dec(&opcode.mode);
            }
            0x49 | 0x45 | 0x55 | 0x4d | 0x5d | 0x59 | 0x41 | 0x51 => {
                self.eor(&opcode.mode);
            }
            0x4a => self.lsr_acu(),

            0x46 | 0x56 | 0x4e | 0x5e => {
                self.lsr(&opcode.mode);
            }
            0x09 | 0x05 | 0x15 | 0x0d | 0x1d | 0x19 | 0x01 | 0x11 => {
                self.ora(&opcode.mode);
            }

            0x2a => self.rol_acu(),
            0x26 | 0x36 | 0x2e | 0x3e => {
                self.rol(&opcode.mode);
            }

            0x6a => self.ror_acu(),
            0x66 | 0x76 | 0x6e | 0x7e => {
                self.ror(&opcode.mode);
            }
            0xe9 | 0xe5 | 0xf5 | 0xed | 0xfd | 0xf9 | 0xe1 | 0xf1 => {
                self.sbc(&opcode.mode);
            }

            /* End Arithmetic & Logic */
            _ => return,
        }
    }
}
