use crate::cpu::control_flow_ops::ControlOpCodes;
use crate::cpu::logic_ops::LogicOpCodes;
use crate::cpu::register_ops::RegisterOpCodes;
use crate::cpu::stack_ops::StackOpCodes;
use crate::cpu::status_ops::StatusOpCodes;

use crate::opcodes;
use std::collections::HashMap;

use self::other_ops::OtherOpCodes;
pub mod control_flow_ops;
pub mod logic_ops;
pub mod other_ops;
pub mod register_ops;
pub mod stack_ops;
pub mod status_ops;

const STACK_PTR_START: u16 = 0x0100;

const STACK_PTR_END: u16 = 0x01FF;
const STACK_PTR_RESET: u8 = 0xFD;

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
pub struct CPU {
    pub register_x: u8,
    pub register_a: u8,
    pub register_y: u8,
    pub stack_ptr: u8,
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
            stack_ptr: STACK_PTR_RESET,
            status: CpuFlags::from_bits_truncate(0b100100),
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
    fn stack_push(&mut self, data: u8) {
        self.mem_write(STACK_PTR_START + self.stack_ptr as u16, data);
        self.stack_ptr = self.stack_ptr.wrapping_sub(1);
    }
    fn stack_pull(&mut self) -> u8 {
        self.stack_ptr = self.stack_ptr.wrapping_add(1);
        let data = self.mem_read(STACK_PTR_START + self.stack_ptr as u16);
        data
    }
    fn stack_push_u16(&mut self, data: u16) {
        let hi = (data >> 8) as u8;
        let lo = (data & 0xff) as u8;
        self.stack_push(hi);
        self.stack_push(lo);
    }
    fn stack_pull_u16(&mut self) -> u16 {
        let lo = self.stack_pull() as u16;
        let hi = self.stack_pull() as u16;

        hi << 8 | lo
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

                let addr = pos.wrapping_add(self.register_x) as u16;
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
            if code == 0x00 {
                return;
            }

            self.handle_control_flow_ops(opcode, code);
            self.handle_logic_ops(opcode, code);
            self.handle_register_ops(opcode, code);
            self.handle_status_ops(opcode, code);
            self.handle_stack_ops(opcode, code);
            self.handle_other_ops(opcode, code);

            if program_counter_state == self.program_counter {
                self.program_counter += (opcode.len - 1) as u16;
            }
        }
    }
    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.status = CpuFlags::from_bits_truncate(0b100100);
        self.stack_ptr = STACK_PTR_RESET;
        self.program_counter = self.mem_read_u16(0xFFFC);
    }
    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }
}
