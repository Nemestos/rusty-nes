use crate::opcodes;
use std::collections::HashMap;

bitflags! {

    pub struct CpuFlags:u8{
        const CARRY             = 0b00000001;
        const ZERO              = 0b00000010;
        const INTERRUPT_DISABLE = 0b00000100;
        const DECIMAL_MODE      = 0b00001000;
        const BREAK             = 0b00010000;
        const BREAK2            = 0b00100000;
        const OVERFLOW          = 0b01000000;
        const NEGATIV           = 0b10000000;
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum AddressingMode {
    Immediate,
    ZeroPage,
    ZeroPage_X,
    ZeroPage_Y,
    Absolute,
    Absolute_X,
    Absolute_Y,
    Indirect_X,
    Indirect_Y,
    NoneAddressing,
}

trait Mem {
    fn mem_read(&self, addr: u16) -> u8;
    fn mem_write(&mut self, addr: u16, data: u8);

    fn mem_read_u16(&self, pos: u16) -> u16 {
        let lowest = self.mem_read(pos) as u16;
        let highest = self.mem_read(pos + 1) as u16;
        // convert to little endian
        (highest << 8) | (lowest as u16)
    }

    fn mem_write_u16(&mut self, pos: u16, data: u16) {
        let highest = (data >> 8) as u8;
        let lowest = (data & 0xff) as u8;
        self.mem_write(pos, lowest);
        self.mem_write(pos + 1, highest);
    }
}

trait OpCodes {
    fn adc(&mut self, mode: &AddressingMode);
    fn and(&mut self, mode: &AddressingMode);
    fn asl_acu(&mut self);
    fn asl(&mut self, mode: &AddressingMode) -> u8;

    fn bit(&mut self, mode: &AddressingMode);

    /*Control Flow */
    fn bcc(&mut self);
    fn bcs(&mut self);
    fn beq(&mut self);
    fn bne(&mut self);
    fn bmi(&mut self);
    fn bpl(&mut self);
    fn bvc(&mut self);
    fn bvs(&mut self);
    /*END Control Flow */

    fn lda(&mut self, mode: &AddressingMode);
    fn sbc(&mut self, mode: &AddressingMode);
    fn sta(&mut self, mode: &AddressingMode);
    fn tax(&mut self);
    fn inx(&mut self);

    /*Status register */
    fn clc(&mut self);
    fn cld(&mut self);
    fn cli(&mut self);
    fn clv(&mut self);
    fn sec(&mut self);
    fn sed(&mut self);
    fn sei(&mut self);
    /*End Status register */
}

impl Mem for CPU {
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }
}

impl OpCodes for CPU {
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
    /*Control Flow */
    fn bcc(&mut self) {
        self.branch_handle(!self.status.contains(CpuFlags::CARRY));
    }
    fn bcs(&mut self) {
        self.branch_handle(self.status.contains(CpuFlags::CARRY));
    }
    fn beq(&mut self) {
        self.branch_handle(self.status.contains(CpuFlags::ZERO));
    }
    fn bne(&mut self) {
        self.branch_handle(!self.status.contains(CpuFlags::ZERO));
    }
    fn bmi(&mut self) {
        self.branch_handle(self.status.contains(CpuFlags::NEGATIV));
    }
    fn bpl(&mut self) {
        self.branch_handle(!self.status.contains(CpuFlags::NEGATIV));
    }
    fn bvc(&mut self) {
        self.branch_handle(!self.status.contains(CpuFlags::OVERFLOW));
    }
    fn bvs(&mut self) {
        self.branch_handle(self.status.contains(CpuFlags::OVERFLOW));
    }
    /*END Control Flow */

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn sbc(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(&mode);
        let data = self.mem_read(addr);
        self.add_to_register_a(((data as i8).wrapping_neg()) as u8);
    }

    fn sta(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        self.mem_write(addr, self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
    }

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
}

pub struct CPU {
    pub register_x: u8,
    pub register_a: u8,
    pub register_y: u8,
    pub status: CpuFlags,
    pub program_counter: u16,
    memory: [u8; 0xffff],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: CpuFlags::from_bits_truncate(0b100100),
            program_counter: 0,
            memory: [0; 0xffff],
        }
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        // we set the zero flag unset
        if result == 0 {
            self.status.insert(CpuFlags::ZERO);
        } else {
            self.status.remove(CpuFlags::ZERO);
        }
        //when negative flag is set
        if result & 0b1000_0000 != 0 {
            self.status.insert(CpuFlags::NEGATIV);
        } else {
            self.status.remove(CpuFlags::NEGATIV);
        }
    }
    fn set_register_a(&mut self, value: u8) {
        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn add_to_register_a(&mut self, data: u8) {
        let is_carry = self.status.contains(CpuFlags::CARRY);
        let sum = self.register_a as u16 + data as u16 + is_carry as u16;
        println!("{:?}", data);

        //when an another overflow on 2 bytes
        let carry = sum > 0xff;

        //we add carry(not the movie xd)
        if carry {
            self.status.insert(CpuFlags::CARRY);
        } else {
            self.status.remove(CpuFlags::CARRY);
        }
        let result = sum as u8;

        if (data ^ result) & (result ^ self.register_a) & 0x80 != 0 {
            self.status.insert(CpuFlags::OVERFLOW);
        } else {
            self.status.remove(CpuFlags::OVERFLOW)
        }
        self.set_register_a(result);
    }
    fn set_carry(&mut self) {
        self.status.insert(CpuFlags::CARRY);
    }
    fn remove_carry(&mut self) {
        self.status.remove(CpuFlags::CARRY);
    }
    fn branch_handle(&mut self, invariant: bool) {
        if invariant {
            let jump: i8 = self.mem_read(self.program_counter) as i8;
            let jump_addr = self
                .program_counter
                .wrapping_add(1)
                .wrapping_add(jump as u16);
            self.program_counter = jump_addr;
        }
    }
    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Immediate => self.program_counter,
            AddressingMode::ZeroPage => self.mem_read(self.program_counter) as u16,
            AddressingMode::Absolute => self.mem_read_u16(self.program_counter),
            AddressingMode::ZeroPage_X => {
                let pos = self.mem_read(self.program_counter);
                println!("{}", self.register_x);

                let addr = pos.wrapping_add(self.register_x) as u16;
                println!("{}", addr);
                addr
            }
            AddressingMode::ZeroPage_Y => {
                let pos = self.mem_read(self.program_counter);
                let addr = pos.wrapping_add(self.register_y) as u16;
                addr
            }

            AddressingMode::Absolute_X => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_x as u16);
                addr
            }
            AddressingMode::Absolute_Y => {
                let base = self.mem_read_u16(self.program_counter);
                let addr = base.wrapping_add(self.register_y as u16);
                addr
            }
            AddressingMode::Indirect_X => {
                let base = self.mem_read(self.program_counter);
                let ptr: u8 = (base as u8).wrapping_add(self.register_x);
                let lowest = self.mem_read(ptr as u16);
                let highest = self.mem_read(ptr.wrapping_add(1) as u16);
                (highest as u16) << 8 | (lowest as u16)
            }
            AddressingMode::Indirect_Y => {
                let base = self.mem_read(self.program_counter);

                let lowest = self.mem_read(base as u16);
                let highest = self.mem_read((base as u8).wrapping_add(1) as u16);
                let deref_base = (highest as u16) << 8 | (lowest as u16);
                let deref = deref_base.wrapping_add(self.register_y as u16);
                deref
            }
            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode)
            }
        }
    }

    pub fn load(&mut self, program: Vec<u8>) {
        self.memory[0x8000..(0x8000 + program.len())].copy_from_slice(&program[..]);
        self.mem_write_u16(0xFFFC, 0x8000);
    }

    pub fn run(&mut self) {
        let ref opcodes: HashMap<u8, &'static opcodes::OpCode> = *opcodes::OPCODES_MAP;
        loop {
            let code = self.mem_read(self.program_counter);
            self.program_counter += 1;

            let program_counter_state = self.program_counter;

            let opcode = opcodes
                .get(&code)
                .expect(&format!("OpCode {:x} is not recognized", code));

            match code {
                /* Arithmetic & Logic */

                /*ADC */
                0x69 | 0x65 | 0x75 | 0x6d | 0x7d | 0x79 | 0x61 | 0x71 => {
                    self.adc(&opcode.mode);
                }
                /*AND */
                0x29 | 0x25 | 0x35 | 0x2d | 0x3d | 0x39 | 0x21 | 0x31 => {
                    self.and(&opcode.mode);
                }
                /*ASL*/ 0x0a => self.asl_acu(),

                /* ASL */
                0x06 | 0x16 | 0x0e | 0x1e => {
                    self.asl(&opcode.mode);
                }
                /* BIT */
                0x24 | 0x2c => {
                    self.bit(&opcode.mode);
                }
                /* SBC */
                0xe9 | 0xe5 | 0xf5 | 0xed | 0xfd | 0xf9 | 0xe1 | 0xf1 => {
                    self.sbc(&opcode.mode);
                }

                /* End Arithmetic & Logic */

                /* A,X,Y Registers */

                /*LDA */
                0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1 => {
                    self.lda(&opcode.mode);
                }

                /* STA */
                0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => {
                    self.sta(&opcode.mode);
                }

                0xAA => self.tax(),
                0xe8 => self.inx(),

                /*End A,X,Y Registers */


                /*Control Flow */
                0x90 => self.bcc(),
                0xb0 => self.bcs(),
                0xf0 => self.beq(),
                0xd0 => self.bne(),
                0x30 => self.bmi(),
                0x10 => self.bpl(),
                0x50 => self.bvc(),
                0x70 => self.bvs(),

                /*END Control Flow */

                /*Status Register */
                0x18 => self.clc(),
                0xd8 => self.cld(),
                0x58 => self.cli(),
                0xb8 => self.clv(),
                0x38 => self.sec(),
                0xf8 => self.sed(),
                0x78 => self.sei(),
                /*End Status Register */
                0x00 => return,
                _ => todo!(),
            }
            if program_counter_state == self.program_counter {
                self.program_counter += (opcode.len - 1) as u16;
            }
        }
    }
    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = CpuFlags::from_bits_truncate(0b100100);

        self.program_counter = self.mem_read_u16(0xFFFC);
    }
    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    //lda
    #[test]
    fn test_0xa9_lda_immediate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 0x05);
        assert!(!cpu.status.contains(CpuFlags::ZERO));
        assert!(!cpu.status.contains(CpuFlags::NEGATIV));
    }

    #[test]
    fn test_0xa5_lda_zero_page() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x55);

        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);
        assert_eq!(cpu.register_a, 0x55);
    }
    #[test]
    fn test_0xb5_lda_zero_page_x() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x20, 0x55);

        cpu.load_and_run(vec![0xa9, 0x10, 0xaa, 0xb5, 0x10, 0x00]);
        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status.contains(CpuFlags::ZERO));
    }

    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0x00]);

        assert!(cpu.status.contains(CpuFlags::NEGATIV));
    }

    //tax
    #[test]
    fn test_0xaa_tax_copy_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0x00]);

        assert_eq!(cpu.register_a, cpu.register_x);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0A, 0xaa, 0x00]);

        assert_eq!(cpu.register_x, 10)
    }

    #[test]
    fn test_0xe8_increment_x() {
        let mut cpu = CPU::new();
        cpu.register_x = 0x00;
        cpu.load_and_run(vec![0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0x01);
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }
    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }

    #[test]
    fn test_and_working() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xEC, 0x29, 0xE0, 0x00]);
        assert_eq!(cpu.register_a, 0xe0);
    }

    #[test]
    fn test_adc() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x10, 0x69, 0x10, 0x00]);
        assert_eq!(cpu.register_a, 0x20);
        assert!(!cpu.status.contains(CpuFlags::CARRY));
        assert!(!cpu.status.contains(CpuFlags::OVERFLOW));
    }
    #[test]
    fn test_adc_carry() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0x69, 0x10, 0x00]);
        assert!(cpu.status.contains(CpuFlags::CARRY))
    }
    #[test]
    fn test_adc_overflow_two_positiv() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x50, 0x69, 0x50, 0x00]);
        assert!(cpu.status.contains(CpuFlags::OVERFLOW))
    }

    #[test]
    fn test_adc_overflow_two_negativ() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xd0, 0x69, 0x90, 0x00]);
        assert!(cpu.status.contains(CpuFlags::OVERFLOW))
    }

    #[test]
    fn test_sbc() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x50, 0xe9, 0x30, 0x00]);
        println!("{:?}", cpu.register_a);
        assert_eq!(cpu.register_a, 0x20);
    }
    #[test]
    fn test_sbc_carry() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0x69, 0x02, 0xe9, 0x01, 0x00]);
        assert!(cpu.status.contains(CpuFlags::CARRY));
    }

    #[test]
    fn test_asl_acu() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x50, 0x0A, 0x00]);
        assert_eq!(cpu.register_a, 0x50 << 1);
    }

    #[test]
    fn test_bit() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x50, 0x2c, 0x00, 0x00]);
        assert!(cpu.status.contains(CpuFlags::ZERO));

        cpu.mem_write(0x10, 0b1000_0000);
        cpu.load_and_run(vec![0xa9, 0b1111_1111, 0x2c, 0x10, 0x00]);
        assert!(cpu.status.contains(CpuFlags::NEGATIV));

        cpu.mem_write(0x10, 0b0100_0000);
        cpu.load_and_run(vec![0xa9, 0b1111_1111, 0x2c, 0x10, 0x00]);
        assert!(cpu.status.contains(CpuFlags::OVERFLOW))
    }
}
