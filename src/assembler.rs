use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use crate::lexer;
use crate::lexer::Lexeme;
use crate::types::{Instruction, Register, Line};
use crate::parser;

struct Assembler {
    symbol_table: HashMap<String, u32>,
    lines: Vec<Line>,
    binary_output: Vec<u8>,
}

impl Assembler {
    fn new() -> Self {
        Assembler {
            symbol_table: HashMap::new(),
            lines: Vec::new(),
            binary_output: Vec::new(),
        }
    }

    //read instructions
    fn read_input(&mut self, filename: &str) -> std::io::Result<()> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            if let Ok(l) = line {
                let tokens = lexer::tokenize_line(&l);
                let parsed_line = parser::parse(tokens);
                self.lines.push(parsed_line);
            }
            // let line = line?;
            // let trimmed = line.trim();
            // if !trimmed.is_empty() && !trimmed.starts_with(';') {
            //     self.instructions.push(trimmed.to_string());
            // }
        }

        return Ok(())
    }

    fn first_pass(&mut self) {
        // Implement first pass logic (symbol table generation)
    }

      fn second_pass(&mut self) {
        // Implement second pass logic (instruction translation)
    }

    // fn encode_instruction(token: &Lexeme) -> u64 {
    //     match token {
    //         Lexeme::Instruction => match instr {
    //             Instruction::Mov => Some(0x88),
    //             "add" => Some(0x01),
    //             "sub" => Some(0x29),
    //             "push" => Some(0x50),
    //             "pop" => Some(0x58),
    //             _ => 0,  // Unknown instruction
    //         },
    //         _ => 0,  // Not an instruction token
    //     }
    // }

    //generate binary output from instruction tokens
    //NOTE: Does it matter if this is pub or not?
    pub fn assemble(&mut self) {
        for instr in &self.lines {
            
            
        }
    }

    pub fn write_output(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        file.write_all(&self.binary_output)?;
        Ok(())
    }
}

