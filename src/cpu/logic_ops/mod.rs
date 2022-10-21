use crate::cpu::CpuFlags;

use super::AddressingMode;
use super::CPU;

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
    fn sbc(&mut self, mode: &AddressingMode);

    /*End Arithmetic Logic */
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
        println!("{:?}", data);
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

    fn sbc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(&mode);
        let data = self.mem_read(addr);
        self.add_to_register_a(((data as i8).wrapping_neg()) as u8);
    }
    /*End Arithmetic & Logic */
}
