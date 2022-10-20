use crate::cpu::AddressingMode;
use std::collections::HashMap;

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

lazy_static! {
    pub static ref CPU_OPS_CODES: Vec<OpCode> = vec![

        /* Arithmetic & logic */

        OpCode::new(0x69,"ADC",2,2,AddressingMode::Immediate),
        OpCode::new(0x65,"ADC",2,3,AddressingMode::ZeroPage),
        OpCode::new(0x75,"ADC",2,4,AddressingMode::ZeroPage_X),
        OpCode::new(0x6D,"ADC",3,4,AddressingMode::Absolute),
        OpCode::new(0x7D,"ADC",3,4,AddressingMode::Absolute_X),
        OpCode::new(0x79,"ADC",3,4,AddressingMode::Absolute_Y),
        OpCode::new(0x61,"ADC",2,6,AddressingMode::Indirect_X),
        OpCode::new(0x71,"ADC",2,5,AddressingMode::Indirect_Y),

        OpCode::new(0x29,"AND",2,2,AddressingMode::Immediate),
        OpCode::new(0x25,"AND",2,3,AddressingMode::ZeroPage),
        OpCode::new(0x35,"AND",2,4,AddressingMode::ZeroPage_X),
        OpCode::new(0x2d,"AND",3,4,AddressingMode::Absolute),
        OpCode::new(0x3d,"AND",3,4,AddressingMode::Absolute_X),
        OpCode::new(0x39,"AND",3,4,AddressingMode::Absolute_Y),
        OpCode::new(0x21,"AND",2,6,AddressingMode::Indirect_X),
        OpCode::new(0x31,"AND",2,5,AddressingMode::Indirect_Y),

        OpCode::new(0x0A,"ASL",1,2,AddressingMode::NoneAddressing),
        OpCode::new(0x06,"ASL",2,5,AddressingMode::ZeroPage),
        OpCode::new(0x16,"ASL",2,6,AddressingMode::ZeroPage_X),
        OpCode::new(0x0E,"ASL",3,6,AddressingMode::Absolute),
        OpCode::new(0x1E,"ASL",3,7,AddressingMode::Absolute_X),

        OpCode::new(0x24,"BIT",2,3,AddressingMode::ZeroPage),
        OpCode::new(0x2C,"BIT",2,3,AddressingMode::Absolute),

        OpCode::new(0xC9,"CMP",2,2,AddressingMode::Immediate),
        OpCode::new(0xC5,"CMP",2,3,AddressingMode::ZeroPage),
        OpCode::new(0xD5,"CMP",2,4,AddressingMode::ZeroPage_X),
        OpCode::new(0xCD,"CMP",3,4,AddressingMode::Absolute),
        OpCode::new(0xDD,"CMP",3,4,AddressingMode::Absolute_X),
        OpCode::new(0xC9,"CMP",2,2,AddressingMode::Absolute_Y),
        OpCode::new(0xC1,"CMP",2,6,AddressingMode::Indirect_X),
        OpCode::new(0xD1,"CMP",2,5,AddressingMode::Indirect_Y),

        OpCode::new(0xE0,"CMX",2,2,AddressingMode::Immediate),
        OpCode::new(0xE4,"CMX",2,3,AddressingMode::ZeroPage),
        OpCode::new(0xEC,"CMX",3,4,AddressingMode::Absolute),



        OpCode::new(0xe9,"SBC",2,2,AddressingMode::Immediate),
        OpCode::new(0xe5,"SBC",2,3,AddressingMode::ZeroPage),
        OpCode::new(0xf5,"SBC",2,4,AddressingMode::ZeroPage_X),
        OpCode::new(0xed,"SBC",3,4,AddressingMode::Absolute),
        OpCode::new(0xfd,"SBC",3,4,AddressingMode::Absolute_X),
        OpCode::new(0xf9,"SBC",3,4,AddressingMode::Absolute_Y),
        OpCode::new(0xe1,"SBC",2,6,AddressingMode::Indirect_X),
        OpCode::new(0xf1,"SBC",2,5,AddressingMode::Indirect_Y),

        /* End Arithmetic & logic */



        /*Interupts */
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing),

        /*End Interupts */

        /*A,X,Y Registers */
        OpCode::new(0xaa, "TAX", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xe8, "INX", 1, 2, AddressingMode::NoneAddressing),

        OpCode::new(0xa9, "LDA", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa5, "LDA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb5, "LDA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0xad, "LDA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbd, "LDA", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_X),
        OpCode::new(0xb9, "LDA", 3, 4/*+1 if page crossed*/, AddressingMode::Absolute_Y),
        OpCode::new(0xa1, "LDA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0xb1, "LDA", 2, 5/*+1 if page crossed*/, AddressingMode::Indirect_Y),



        OpCode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPage_X),
        OpCode::new(0x8d, "STA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x9d, "STA", 3, 5, AddressingMode::Absolute_X),
        OpCode::new(0x99, "STA", 3, 5, AddressingMode::Absolute_Y),
        OpCode::new(0x81, "STA", 2, 6, AddressingMode::Indirect_X),
        OpCode::new(0x91, "STA", 2, 6, AddressingMode::Indirect_Y),

        /*End A,X,Y Registers */


        /* Control flow */
        OpCode::new(0x90, "BCC", 2, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xB0, "BCS", 2, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xF0, "BEQ", 2, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xD0, "BNE", 2, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x30, "BMI", 2, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x10, "BPL", 2, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x50, "BVC", 2, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x70, "BVS", 2, 2, AddressingMode::NoneAddressing),
        /* End Control flow */

        /*Status register */
        OpCode::new(0x18, "CLC", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xD8, "CLD", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x58, "CLI", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xB8, "CLV", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x38, "SEC", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xF8, "SED", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x78, "SEI", 1, 2, AddressingMode::NoneAddressing),


    ];
    pub static ref OPCODES_MAP:HashMap<u8,&'static OpCode> = {
        let mut map = HashMap::new();

        for cpuop in &*CPU_OPS_CODES {
            map.insert(cpuop.code, cpuop);
        }
        map
    };
}
