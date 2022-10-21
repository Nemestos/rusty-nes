use super::AddressingMode;
use super::CPU;

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

    fn tax(&mut self);

    /*End A,X,Y Registers */
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

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    /*End A,X,Y Registers */
}
