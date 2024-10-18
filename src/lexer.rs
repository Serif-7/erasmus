use std::str::Chars;
use crate::types::{Register, Instruction};

pub struct Lexer<'a> {
    chars: Chars<'a>,
    current: Option<char>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Lexeme {
    Label(String),
    Instruction(String),
    Register(String),
    Number(i64),
    String(String),
    // Comma,
    OpenBracket,
    CloseBracket,
    Plus,
    Minus,
    Mult,
    Identifier(String),
}

impl<'a> Lexer<'a> {
    fn new(input: &'a str) -> Self {
        let mut chars = input.chars();
        let current = chars.next();
        return Lexer { chars, current }
    }

    fn next_char(&mut self) {
        self.current = self.chars.next();
    }

    fn tokenize(&mut self) -> Vec<Lexeme> {
        let mut tokens = Vec::new();

        while let Some(c) = self.current {
            match c {
                ' ' | '\t' => self.next_char(),
                ',' => {
                    // tokens.push(Lexeme::Comma);
                    self.next_char();
                }
                '[' => {
                    tokens.push(Lexeme::OpenBracket);
                    self.next_char();
                }
                ']' => {
                    tokens.push(Lexeme::CloseBracket);
                    self.next_char();
                }
                '+' => {
                    tokens.push(Lexeme::Plus);
                    self.next_char();
                }
                '-' => {
                    tokens.push(Lexeme::Minus);
                    self.next_char();
                }
                '*' => {
                    tokens.push(Lexeme::Mult);
                    self.next_char();
                }
                '"' => tokens.push(self.tokenize_string()),
                ';' => break, // Comment, ignore rest of line
                '0'..='9' => tokens.push(self.tokenize_number()),
                'a'..='z' | 'A'..='Z' | '_' | '.' => {
                    let ident = self.tokenize_identifier();
                    // println!("ident: {}", ident);
                    if ident.ends_with(':') {
                        tokens.push(Lexeme::Label(ident.trim_end_matches(':').to_string()));
                    } else if Register::is_register(&ident) {
                        tokens.push(Lexeme::Register(ident));
                    } else if Instruction::is_instruction(&ident) {
                        tokens.push(Lexeme::Instruction(ident));
                    } else {
                        tokens.push(Lexeme::Identifier(ident));
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

    fn tokenize_string(&mut self) -> Lexeme {
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
        return Lexeme::String(string)
    }

    fn tokenize_number(&mut self) -> Lexeme {
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
            Ok(n) => Lexeme::Number(n),
            Err(_) => {
                eprintln!("Failed to parse number: {}", number);
                Lexeme::Number(0) // Default to 0 on error, or handle as appropriate
            }
        }
        // return Lexer::Number(i64::from_str_radix(&number, 10).unwrap_or_else(|_| number.parse().unwrap()));
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

pub fn tokenize_line(line: &str) -> Vec<Lexeme> {
    return Lexer::new(line).tokenize()
}

