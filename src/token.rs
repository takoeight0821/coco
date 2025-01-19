use crate::{core_ir, location};

#[derive(Debug)]
pub struct Token {
    pub location: location::Location,
    pub kind: TokenKind,
}

impl Token {
    pub fn new(file: String, start: usize, end: usize, kind: TokenKind) -> Self {
        Self {
            location: location::Location { file, start, end },
            kind,
        }
    }

    pub fn is_identifier(&self) -> bool {
        matches!(&self.kind, TokenKind::Identifier(_))
    }

    pub fn is_literal(&self) -> bool {
        matches!(&self.kind, TokenKind::Literal(_))
    }

    pub fn is_punctuation(&self, punctuation: &str) -> bool {
        match &self.kind {
            TokenKind::Punctuation(p) => p == punctuation,
            _ => false,
        }
    }
}

#[derive(Debug)]
pub enum TokenKind {
    Identifier(String),
    Literal(core_ir::Literal),
    Punctuation(String),
}
