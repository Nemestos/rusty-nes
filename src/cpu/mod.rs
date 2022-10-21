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

    /*Control Flow */
    fn bcc(&mut self);
    fn bcs(&mut self);
    fn beq(&mut self);
    fn bne(&mut self);
    fn bmi(&mut self);
    fn bpl(&mut self);
    fn bvc(&mut self);
    fn bvs(&mut self);

    fn jmp(&mut self);
    fn jmp_indirect(&mut self);

    /*END Control Flow */

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
    fn jmp(&mut self) {
        let addr = self.mem_read_u16(self.program_counter);
        self.program_counter = addr;
    }

    fn jmp_indirect(&mut self) {
        let mem_address = self.mem_read_u16(self.program_counter);

        //bug on 6502 when fetch on a page boundary so just fetch lsb from 0xxff but msb from 0xx00

        let indirect_ref = if mem_address & 0x00FF == 0x00FF {
            let lo = self.mem_read(mem_address);
            let hi = self.mem_read(mem_address & 0xFF00);
            (hi as u16) << 8 | (lo as u16)
        } else {
            self.mem_read_u16(mem_address)
        };

        self.program_counter = indirect_ref;
    }

    /*END Control Flow */

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

    fn compare_handle(&mut self, mode: &AddressingMode, base: u8) {
        let addr = self.get_operand_address(mode);
        let data = self.mem_read(addr);
        self.status.set(CpuFlags::CARRY, base >= data);
        self.update_zero_and_negative_flags(base.wrapping_sub(data))
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
                /*CMP */
                0xc9 | 0xc5 | 0xd5 | 0xcd | 0xdd | 0xd9 | 0xc1 | 0xd1 => {
                    self.cmp(&opcode.mode);
                }
                /*CMP */
                0xc6 | 0xd6 | 0xce | 0xde => {
                    self.dec(&opcode.mode);
                }
                /*CMP */
                0x49 | 0x45 | 0x55 | 0x4d | 0x5d | 0x59 | 0x41 | 0x51 => {
                    self.eor(&opcode.mode);
                }

                /* SBC */
                0xe9 | 0xe5 | 0xf5 | 0xed | 0xfd | 0xf9 | 0xe1 | 0xf1 => {
                    self.sbc(&opcode.mode);
                }

                /* End Arithmetic & Logic */

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

                0xAA => self.tax(),
                0xe8 => self.inx(),
                0xc8 => self.iny(),
                0xe6 | 0xf6 | 0xee | 0xFE => self.inc(&opcode.mode),

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

                0x4c => self.jmp(),
                0x6c => self.jmp_indirect(),

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

    /*Arithmethic & logic */
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
    fn test_dec() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x05);
        cpu.load_and_run(vec![0xce, 0x10, 0x00]);
        assert_eq!(cpu.mem_read(0x10), 0x04);
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

    #[test]
    fn test_cmp() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x05);
        cpu.load_and_run(vec![0xa9, 0x10, 0xc9, 0x10, 0x00]);
        assert!(cpu.status.contains(CpuFlags::CARRY));

        cpu.mem_write(0x10, 0x010);
        cpu.load_and_run(vec![0xa9, 0x10, 0xc9, 0x10, 0x00]);
        assert!(cpu.status.contains(CpuFlags::ZERO));
    }

    #[test]
    fn test_eor() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x10, 0x49, 0xff]);
        assert_eq!(cpu.register_a, 0x10 ^ 0xff);
    }

    /*End Arithmethic & logic */

    /*A,X,Y Registers */

    #[test]
    fn test_cpx() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x05);
        cpu.load_and_run(vec![0xa2, 0x10, 0xe0, 0x10, 0x00]);
        assert!(cpu.status.contains(CpuFlags::CARRY));

        cpu.mem_write(0x10, 0x010);
        cpu.load_and_run(vec![0xa2, 0x10, 0xe0, 0x10, 0x00]);
        assert!(cpu.status.contains(CpuFlags::ZERO));
    }
    #[test]
    fn test_cpy() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x05);
        cpu.load_and_run(vec![0xa0, 0x10, 0xc0, 0x10, 0x00]);
        assert!(cpu.status.contains(CpuFlags::CARRY));

        cpu.mem_write(0x10, 0x010);
        cpu.load_and_run(vec![0xa0, 0x10, 0xc0, 0x10, 0x00]);
        assert!(cpu.status.contains(CpuFlags::ZERO));
    }
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

    #[test]
    fn test_dex() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x05, 0xca, 0x00]);
        assert_eq!(cpu.register_x, 0x04);
    }
    #[test]
    fn test_dey() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x05, 0x88, 0x00]);
        assert_eq!(cpu.register_y, 0x04);
    }

    #[test]
    fn test_ldx() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0xff, 0x00]);

        assert_eq!(cpu.register_x, 0xff);
    }
    #[test]
    fn test_ldy() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0xff, 0x00]);

        assert_eq!(cpu.register_y, 0xff);
    }

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
    fn test_inc() {
        let mut cpu = CPU::new();
        cpu.mem_write(0x10, 0x05);
        cpu.load_and_run(vec![0xee, 0x10]);

        assert_eq!(cpu.mem_read(0x10), 0x06);
    }

    #[test]
    fn test_inx() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x05, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0x06);
    }
    #[test]
    fn test_iny() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x05, 0xc8, 0x00]);

        assert_eq!(cpu.register_y, 0x06);
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }

    /*End A,X,Y Registers */

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }
}
