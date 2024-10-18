use std::{default, str::FromStr};
use strum_macros::EnumString;


#[derive(Debug)]
enum Words {
    WORD,
    DWORD,
    QWORD,
    DQWORD,
}

#[derive(Debug)]
pub enum Line {
  // PPDirective,
  // Macro,
  Directive,
  Normal {
    label: Option<String>,
    instruction: Option<Instruction>,
    operands: Option<[Operand; 3]>
  }
}

// TODO: write a macro to replace EnumString
#[derive(Debug, EnumString)]
pub enum Register {
    //64 bit registers
    RAX,
    RBX,
    RCX,
    RDX,
    RSI,
    RDI,
    RSP,
    RBP,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    RIP,

    //32 bit registers
    EAX,
    EBX,
    ECX,
    EDX,
    ESI,
    EDI,
    ESP,
    EBP,
    EIP,

    //16 bit registers
    AX,
    BX,
    CX,
    SP,
    BP,
    DI,
    SI,
    DX,

    //8 bit registers
    AH, // higher byte of AL (hence the 'h')
    AL,
    BH,
    BL,
    CH,
    CL,
    DH,
    DL,
}

impl Register {
    
    pub fn is_register(s: &str) -> bool {
        // Add all valid register names for your target architecture
        // return ["rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rsp", "rbp", 
        //  "eax", "ebx", "ecx", "edx", "esi", "edi", "esp", "ebp",
        //  "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15"].contains(&s.to_lowercase().as_str());

         if let Ok(_) = Register::from_str(&s.to_uppercase()) {
             true
         }
         else {
             false
         }
    }
}

#[derive(Debug)]
pub enum Operand {
    Immediate(u64),
    Register(Register),
    Address(u64),
}

// sorted by opcode number
#[derive(Debug, EnumString)]
pub enum Instruction {
    ADD,
    PUSH,
    POP,
    OR,
    ADC, // add with carry
    SUB,
    SBB,
    AND,
    XOR,
    NOT,
    MOV,
    XCHG,
    BSWAP, // BYTE SWAP
    CWD, // CONVERT WORD TO DWORD
    CDQ, // CONVERT DWORD TO QWORD
    MUL,
    DIV, //UNSIGNED DIVIDE
    INC,
    DEC,
    NEG,
    CMP,
    
    
    
}

impl Instruction {
    pub fn is_instruction(s: &str) -> bool {
        // return ["mov", "add"].contains(&s.to_lowercase().as_str());
         match Instruction::from_str(&s.to_uppercase()) {
             Ok(_) => true,
             Err(_) => false,
         }
    }

    // pub fn encode(&self) -> Vec<u8> {
        
    // }
}
