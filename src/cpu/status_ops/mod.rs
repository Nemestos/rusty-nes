use crate::cpu::CpuFlags;

use super::CPU;

pub trait StatusOpCodes {
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

impl StatusOpCodes for CPU {
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
