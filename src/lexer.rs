use crate::{core_ir, token};

pub struct Lexer<'a> {
    pub file: String,
    pub source: std::str::Chars<'a>,
    pub cursor: usize,
}

impl<'a> Lexer<'a> {
    pub fn new(file: String, source: &'a str) -> Self {
        Self {
            file,
            source: source.chars(),
            cursor: 0,
        }
    }

    /// Returns the current character without advancing the cursor.
    fn peek(&self) -> Option<char> {
        self.source.clone().nth(self.cursor)
    }

    /// Advance the cursor.
    fn advance(&mut self) {
        self.cursor += 1;
    }

    /// Advances the cursor until the predicate returns false.
    /// Returns the substring that was skipped.
    /// The cursor is positioned at the first character that failed the predicate.
    fn skip_while<P>(&mut self, mut predicate: P) -> String
    where
        P: FnMut(char) -> bool,
    {
        let start = self.cursor;
        while let Some(c) = self.peek() {
            if !predicate(c) {
                break;
            }
            self.advance();
        }
        self.source
            .clone()
            .skip(start)
            .take(self.cursor - start)
            .collect()
    }

    /// Returns the next token.
    /// The cursor is positioned at the first character after the token.
    pub fn next_token(&mut self) -> Option<token::Token> {
        self.skip_while(char::is_whitespace);
        let start = self.cursor;
        match self.peek() {
            None => None,
            Some(c) if is_identifier_start(c) => {
                let text = self.skip_while(is_identifier_continue);
                Some(if text == "true" {
                    token::Token::new(
                        self.file.clone(),
                        start,
                        self.cursor,
                        token::TokenKind::Literal(core_ir::Literal::Bool(true)),
                    )
                } else if text == "false" {
                    token::Token::new(
                        self.file.clone(),
                        start,
                        self.cursor,
                        token::TokenKind::Literal(core_ir::Literal::Bool(false)),
                    )
                } else {
                    token::Token::new(
                        self.file.clone(),
                        start,
                        self.cursor,
                        token::TokenKind::Identifier(text),
                    )
                })
            }
            Some(c) if is_int_start(c) => {
                let text = self.skip_while(is_int_continue);
                if self.peek() == Some('.') {
                    self.advance();
                    let text = text + "." + &self.skip_while(is_int_continue);
                    Some(token::Token::new(
                        self.file.clone(),
                        start,
                        self.cursor,
                        token::TokenKind::Literal(core_ir::Literal::Float(text.parse().unwrap())),
                    ))
                } else {
                    Some(token::Token::new(
                        self.file.clone(),
                        start,
                        self.cursor,
                        token::TokenKind::Literal(core_ir::Literal::Int(text.parse().unwrap())),
                    ))
                }
            }
            Some(c) if c == '"' => {
                self.advance();
                while self.peek() != None {
                    if self.peek() == Some('\\') {
                        self.advance();
                        self.advance();
                    }
                    if self.peek() == Some('"') {
                        break;
                    }
                    self.advance();
                }
                if self.peek() != Some('"') {
                    panic!("unterminated string literal");
                }
                self.advance();
                Some(token::Token::new(
                    self.file.clone(),
                    start,
                    self.cursor,
                    token::TokenKind::Literal(core_ir::Literal::String(
                        self.source
                            .clone()
                            .skip(start + 1)
                            .take(self.cursor - start - 2)
                            .collect(),
                    )),
                ))
            }
            Some(c) => {
                self.advance();
                match c {
                    '(' | ')' | '{' | '}' | '[' | ']' | '<' | '>' | ',' | ';' | ':' | '.' | '='
                    | '|' => Some(token::Token::new(
                        self.file.clone(),
                        start,
                        self.cursor,
                        token::TokenKind::Punctuation(c.to_string()),
                    )),
                    '-' => {
                        if self.peek() == Some('>') {
                            self.advance();
                            Some(token::Token::new(
                                self.file.clone(),
                                start,
                                self.cursor,
                                token::TokenKind::Punctuation("->".to_string()),
                            ))
                        } else {
                            panic!("unexpected character: {}", c);
                        }
                    }
                    _ => panic!("unexpected character: {}", c),
                }
            }
        }
    }
}

fn is_int_start(c: char) -> bool {
    c.is_digit(10)
}

fn is_int_continue(c: char) -> bool {
    c.is_digit(10)
}

fn is_identifier_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

fn is_identifier_continue(c: char) -> bool {
    assert!(is_identifier_start(c));
    c.is_alphanumeric() || c == '_'
}
