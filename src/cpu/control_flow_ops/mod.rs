use crate::opcodes::OpCode;

use super::CpuFlags;
use super::CPU;
pub mod test;

pub trait ControlOpCodes {
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

    fn jsr(&mut self);
    fn rts(&mut self);

    /*END Control Flow */
    fn handle_control_flow_ops(&mut self, opcode: &OpCode, code: u8);
}

impl ControlOpCodes for CPU {
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
    fn jsr(&mut self) {
        self.stack_push_u16(self.program_counter + 2 - 1);
        let target = self.mem_read_u16(self.program_counter);
        self.program_counter = target;
    }
    fn rts(&mut self) {
        self.program_counter = self.stack_pull_u16() + 1;
    }

    /*END Control Flow */

    fn handle_control_flow_ops(&mut self, opcode: &OpCode, code: u8) {
        match code {
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

            0x20 => self.jsr(),
            0x60 => self.rts(),

            /*END Control Flow */
            _ => return,
        }
    }
}
