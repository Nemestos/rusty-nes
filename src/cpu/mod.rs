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

pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub program_counter: u16,
    memory: [u8; 0xffff],
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            program_counter: 0,
            memory: [0; 0xffff],
        }
    }
    fn mem_read(&self, addr: u16) -> u8 {
        self.memory[addr as usize]
    }

    fn mem_write(&mut self, addr: u16, data: u8) {
        self.memory[addr as usize] = data;
    }

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

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        // we set the zero flag unset
        if result == 0 {
            self.status = self.status | 0b0000_0010;
        } else {
            self.status = self.status & 0b1111_1101;
        }
        //when negative flag is set
        if result & 0b1000_0000 != 0 {
            self.status = self.status | 0b1000_0000;
        } else {
            self.status = self.status & 0b0111_1111;
        }
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let addr = self.get_operand_address(mode);
        let value = self.mem_read(addr);

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;
        self.update_zero_and_negative_flags(self.register_x);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);
        self.update_zero_and_negative_flags(self.register_x);
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
        loop {
            let opcode = self.mem_read(self.program_counter);
            self.program_counter += 1;

            match opcode {
                0xA9 => {
                    self.lda(&AddressingMode::Immediate);
                    self.program_counter += 1;
                }
                0xA5 => {
                    self.lda(&AddressingMode::ZeroPage);
                    self.program_counter += 1;
                }
                0xAD => {
                    self.lda(&AddressingMode::Absolute);
                    self.program_counter += 1;
                }

                0xB5 => {
                    self.lda(&AddressingMode::ZeroPage_X);
                    self.program_counter += 1;
                }
                0xBD => {
                    self.lda(&AddressingMode::Absolute_X);
                    self.program_counter += 1;
                }
                0xB9 => {
                    self.lda(&AddressingMode::Absolute_Y);
                    self.program_counter += 1;
                }
                0xA1 => {
                    self.lda(&AddressingMode::Indirect_X);
                    self.program_counter += 1;
                }
                0xB1 => {
                    self.lda(&AddressingMode::Indirect_Y);
                    self.program_counter += 1;
                }
                0xAA => self.tax(),
                0xE8 => self.inx(),
                0x00 => return,
                _ => todo!(),
            }
        }
    }
    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = 0;

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
        assert!(cpu.status & 0b0000_0010 == 0b00);
        assert!(cpu.status & 0b0000_0010 == 0b00);
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
        assert!(cpu.status & 0b0000_0010 == 0b10)
    }

    #[test]
    fn test_0xa9_lda_negative_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0x00]);

        assert!(cpu.status & 0b1000_0000 == 0b1000_0000)
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
}
