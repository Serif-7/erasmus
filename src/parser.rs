use crate::types::{Register, Instruction};
use crate::lexer::Lexeme;

pub struct Parser {
    tokens: &Vec<Lexeme>,
    current: Option<&Lexeme>,
}

impl Parser {
    fn new(tokens: &Vec<Lexeme>) -> Self {
        let current = tokens.into_iter().next();
        return Parser { tokens, current}
    }

    fn parse(&self)
}
