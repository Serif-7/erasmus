use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

struct Assembler {
    symbol_table: HashMap<String, u32>,
    instructions: Vec<String>,
    binary_output: Vec<u8>,
}

impl Assembler {
    fn new() -> Self {
        Assembler {
            symbol_table: HashMap::new(),
            instructions: Vec::new(),
            binary_output: Vec::new(),
        }
    }

    //read instructions
    fn read_input(&mut self, filename: &str) -> std::io::Result<()> {
        let file = File::open(filename)?;
        let reader = BufReader::new(file);

        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();
            if !trimmed.is_empty() && !trimmed.starts_with(';') {
                self.instructions.push(trimmed.to_string());
            }
        }

        Ok(())
    }

    fn first_pass(&mut self) {
        // Implement first pass logic (symbol table generation)
    }

    fn second_pass(&mut self) {
        // Implement second pass logic (instruction translation)
    }

    // encode a single instruction
    fn encode(&self) {
        for instr in &self.instructions {
            let tokens = instr.split_whitespace();
            for tok in tokens {
                println!("{}", tok);
            }
        }
    }

    fn write_output(&self, filename: &str) -> std::io::Result<()> {
        let mut file = File::create(filename)?;
        file.write_all(&self.binary_output)?;
        Ok(())
    }
}

