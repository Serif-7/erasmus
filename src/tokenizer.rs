use std::str::Chars;

pub struct Tokenizer<'a> {
    chars: Chars<'a>,
    current: Option<char>,
}

#[derive(Debug, PartialEq)]
pub enum Token {
    Label(String),
    Instruction(String),
    Register(String),
    Number(i64),
    String(String),
    Comma,
    OpenBracket,
    CloseBracket,
    Plus,
    Minus,
    Mult,
    Identifier(String),
}

impl<'a> Tokenizer<'a> {
    fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current = chars.next();
        return Tokenizer { chars, current }
    }

    fn next_char(&mut self) {
        self.current = self.chars.next();
    }

    fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while let Some(c) = self.current {
            match c {
                ' ' | '\t' => self.next_char(),
                ',' => {
                    tokens.push(Token::Comma);
                    self.next_char();
                }
                '[' => {
                    tokens.push(Token::OpenBracket);
                    self.next_char();
                }
                ']' => {
                    tokens.push(Token::CloseBracket);
                    self.next_char();
                }
                '+' => {
                    tokens.push(Token::Plus);
                    self.next_char();
                }
                '-' => {
                    tokens.push(Token::Minus);
                    self.next_char();
                }
                '*' => {
                    tokens.push(Token::Mult);
                    self.next_char();
                }
                '"' => tokens.push(self.tokenize_string()),
                ';' => break, // Comment, ignore rest of line
                '0'..='9' => tokens.push(self.tokenize_number()),
                'a'..='z' | 'A'..='Z' | '_' | '.' => {
                    let ident = self.tokenize_identifier();
                    println!("ident: {}", ident);
                    if ident.ends_with(':') {
                        tokens.push(Token::Label(ident.trim_end_matches(':').to_string()));
                    } else if is_register(&ident) {
                        tokens.push(Token::Register(ident));
                    } else if is_instruction(&ident) {
                        tokens.push(Token::Instruction(ident));
                    } else {
                        tokens.push(Token::Identifier(ident));
                    }
                }
                _ => {
                    eprintln!("Unexpected character: {}", c);
                    self.next_char();
                }
            }
        }

        return tokens
    }

    fn tokenize_string(&mut self) -> Token {
        self.next_char(); // Skip opening quote
        let mut string = String::new();
        while let Some(c) = self.current {
            if c == '"' {
                self.next_char();
                break;
            }
            string.push(c);
            self.next_char();
        }
        return Token::String(string)
    }

    fn tokenize_number(&mut self) -> Token {
        let mut number = String::new();
        let mut is_hex = false;
        
        // Check for hexadecimal prefix
        if self.current == Some('0') {
            number.push('0');
            self.next_char();
            if self.current == Some('x') || self.current == Some('X') {
                is_hex = true;
                number.push('x');
                self.next_char();
            }
        }
        while let Some(c) = self.current {
            if c.is_digit(10) || (is_hex && c.is_ascii_hexdigit()) {
                number.push(c);
                self.next_char();
                }
            else {
                break;
            }
        }
        let parsed_number = if is_hex {
            i64::from_str_radix(&number[2..], 16)
        } else {
            number.parse()
        };

        match parsed_number {
            Ok(n) => Token::Number(n),
            Err(_) => {
                eprintln!("Failed to parse number: {}", number);
                Token::Number(0) // Default to 0 on error, or handle as appropriate
            }
        }
        // return Token::Number(i64::from_str_radix(&number, 10).unwrap_or_else(|_| number.parse().unwrap()));
    }

    fn tokenize_identifier(&mut self) -> String {
        let mut ident = String::new();
        while let Some(c) = self.current {
            if c.is_alphanumeric() || c == '_' || c == '.' || c == ':' {
                ident.push(c);
                self.next_char();
            } else {
                break;
            }
        }
        return ident
    }
}

//TODO: Add all registers
pub fn is_register(s: &str) -> bool {
    // Add all valid register names for your target architecture
    return ["rax", "rbx", "rcx", "rdx", "rsi", "rdi", "rsp", "rbp", 
     "eax", "ebx", "ecx", "edx", "esi", "edi", "esp", "ebp",
     "r8", "r9", "r10", "r11", "r12", "r13", "r14", "r15"].contains(&s.to_lowercase().as_str());
}

//TODO: Add all instructions
pub fn is_instruction(s: &str) -> bool {
    return ["mov", "add"].contains(&s.to_lowercase().as_str());
}

pub fn tokenize_line(line: &str) -> Vec<Token> {
    return Tokenizer::new(line).tokenize()
}
