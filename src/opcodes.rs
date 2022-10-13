use crate::cpu::AddressingMode;
pub struct OpCode {
    pub code: u8,
    pub human: &'static str,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl OpCode {
    fn new(code: u8, human: &'static str, len: u8, cycles: u8, mode: AddressingMode) -> Self {
        OpCode {
            code: code,
            human: human,
            len: len,
            cycles: cycles,
            mode: mode,
        }
    }
}
