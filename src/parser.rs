use crate::core_ir::*;
use crate::lexer;
use crate::location;
use crate::token::*;
use ariadne::Label;
use ariadne::Report;
use ariadne::ReportKind;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("unexpected token {actual:?}, expected one of {expected:?}")]
    UnexpectedToken {
        expected: Vec<String>,
        actual: Token,
    },
    #[error("unexpected end of file")]
    UnexpectedEOF { last: location::Location },
}

/// Convert Error to a ariadne::Report.
impl From<Error> for ariadne::Report<'_, location::Location> {
    fn from(error: Error) -> Self {
        match error {
            Error::UnexpectedToken { expected, actual } => {
                Report::build(ReportKind::Error, actual.location.clone())
                    .with_message("unexpected token")
                    .with_label(
                        Label::new(actual.location.clone())
                            .with_message(format!("expected one of {:?}", expected)),
                    )
                    .finish()
            }
            Error::UnexpectedEOF { last } => Report::build(ReportKind::Error, last.clone())
                .with_label(Label::new(last.clone()).with_message("unexpected end of file"))
                .finish(),
        }
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct Parser<'a> {
    lexer: lexer::Lexer<'a>,
    last_location: location::Location,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: lexer::Lexer<'a>) -> Self {
        Self {
            lexer: lexer.clone(),
            last_location: lexer.clone().next_token().unwrap().location.clone(),
        }
    }

    fn peek(&self) -> Result<Token> {
        self.lexer.clone().next_token().ok_or(Error::UnexpectedEOF {
            last: self.last_location.clone(),
        })
    }

    fn advance(&mut self) {
        let token = self.lexer.next_token();
        if let Some(token) = token {
            self.last_location = token.location.clone();
        }
    }

    /// Expect the next token to be a keyword.
    /// If the next token is a expected keyword, consume it and return it.
    /// Otherwise, return an error.
    fn expect_keyword(&mut self, keyword: &str) -> Result<Token> {
        let token = self.peek()?;

        if let TokenKind::Identifier(ident) = &token.kind {
            if ident == keyword {
                self.advance();
                Ok(token)
            } else {
                Err(Error::UnexpectedToken {
                    expected: vec![keyword.to_string()],
                    actual: token,
                })
            }
        } else {
            Err(Error::UnexpectedToken {
                expected: vec![keyword.to_string()],
                actual: token,
            })
        }
    }

    fn expect_punctuation(&mut self, punctuation: &str) -> Result<Token> {
        let token = self.peek()?;

        if let TokenKind::Punctuation(p) = &token.kind {
            if p == punctuation {
                self.advance();
                Ok(token)
            } else {
                Err(Error::UnexpectedToken {
                    expected: vec![punctuation.to_string()],
                    actual: token,
                })
            }
        } else {
            Err(Error::UnexpectedToken {
                expected: vec![punctuation.to_string()],
                actual: token,
            })
        }
    }

    fn eof(&self) -> bool {
        self.peek().is_err()
    }

    pub fn parse(&mut self) -> Result<Program<String>> {
        let mut program = Vec::new();

        while !self.eof() {
            program.push(self.definition()?);
        }

        Ok(program)
    }

    fn definition(&mut self) -> Result<Definition<String>> {
        let def = self.expect_keyword("def")?;
        let (name, _) = self.identifier()?;
        self.expect_punctuation("(")?;
        let (parameters, _) =
            self.sep_end(";", ",", |parser| parser.identifier().map(|(name, _)| name))?;
        let (returns, _) =
            self.sep_end(")", ",", |parser| parser.identifier().map(|(name, _)| name))?;
        self.expect_punctuation("=")?;
        let body = self.statement()?;
        let location = def.location.to(&body.location);
        Ok(Definition {
            location,
            name,
            parameters,
            returns,
            body,
        })
    }

    fn identifier(&mut self) -> Result<(String, location::Location)> {
        let token = self.peek()?;

        if let TokenKind::Identifier(ident) = &token.kind {
            self.advance();
            Ok((ident.clone(), token.location))
        } else {
            Err(Error::UnexpectedToken {
                expected: vec!["identifier".to_string()],
                actual: token,
            })
        }
    }

    fn statement(&mut self) -> Result<Statement<String>> {
        match self.peek()? {
            Token {
                kind: TokenKind::Identifier(ident),
                ..
            } => match ident.as_str() {
                "prim" => return self.prim(),
                "switch" => return self.switch(),
                "invoke" => return self.invoke(),
                _ => {}
            },
            _ => {}
        }
        let producer = self.producer()?;
        self.postfix_statement(producer)
    }

    fn producer(&mut self) -> Result<Producer<String>> {
        if let Token {
            kind: TokenKind::Identifier(i),
            ..
        } = self.peek()?
        {
            if i == "do" {
                return self.do_();
            }
            return self.variable();
        }

        if let Token {
            kind: TokenKind::Literal(literal),
            location,
        } = self.peek()?
        {
            self.advance();
            return Ok(Producer {
                location,
                kind: ProducerKind::Literal(literal),
            });
        }

        todo!()
    }

    fn do_(&mut self) -> Result<Producer<String>> {
        let do_token = self.expect_keyword("do")?;
        let (name, _) = self.identifier()?;
        let statement = self.statement()?;
        let location = do_token.location.to(&statement.location);

        Ok(Producer {
            location,
            kind: ProducerKind::Do(Do {
                name,
                body: Box::new(statement),
            }),
        })
    }

    fn variable(&mut self) -> Result<Producer<String>> {
        let (name, location) = self.identifier()?;
        // if next token is '(', then it is a constructor application
        if let Token {
            kind: TokenKind::Punctuation(p),
            ..
        } = self.peek()?
        {
            if p == "(" {
                return self.construct(name, location);
            }
        }

        Ok(Producer {
            location,
            kind: ProducerKind::Variable(Variable { name }),
        })
    }

    fn construct(
        &mut self,
        name: String,
        location: location::Location,
    ) -> Result<Producer<String>> {
        self.expect_punctuation("(")?;
        let (producers, _) = self.sep_end(";", ",", |parser| parser.producer())?;
        let (consumers, right_paren) = self.sep_end(")", ",", |parser| parser.consumer())?;

        let location = location.to(&right_paren.location);

        Ok(Producer {
            location,
            kind: ProducerKind::Construct(Construct {
                tag: name,
                producers,
                consumers,
            }),
        })
    }

    fn consumer(&mut self) -> Result<Consumer<String>> {
        if let Token {
            kind: TokenKind::Identifier(i),
            ..
        } = self.peek()?
        {
            if i == "then" {
                return self.then();
            }

            if i == "match" {
                return self.match_();
            }

            return self.covariable();
        }

        todo!()
    }

    fn then(&mut self) -> Result<Consumer<String>> {
        let then = self.expect_keyword("then")?;
        let (name, _) = self.identifier()?;
        let body = self.statement()?;
        let location = then.location.to(&body.location);

        Ok(Consumer {
            location,
            kind: ConsumerKind::Then(Then {
                name,
                body: Box::new(body),
            }),
        })
    }

    fn match_(&mut self) -> Result<Consumer<String>> {
        let match_ = self.expect_keyword("match")?;
        self.expect_punctuation("{")?;
        let (clauses, right_brace): (Vec<_>, _) =
            self.sep_end("}", ",", |parser| parser.clause())?;

        let location = match_.location.to(&right_brace.location);

        Ok(Consumer {
            location,
            kind: ConsumerKind::Match(Match { clauses }),
        })
    }

    fn clause(&mut self) -> Result<Clause<String>> {
        let (pattern, location) = self.pattern()?;
        self.expect_punctuation("->")?;
        let body = self.statement()?;
        let location = location.to(&body.location);

        Ok(Clause {
            location,
            pattern,
            body,
        })
    }

    fn pattern(&mut self) -> Result<(Pattern<String>, location::Location)> {
        let (tag, location) = self.identifier()?;
        self.expect_punctuation("(")?;
        let (parameters, _) =
            self.sep_end(";", ",", |parser| parser.identifier().map(|(name, _)| name))?;
        let (returns, right_paren) =
            self.sep_end(")", ",", |parser| parser.identifier().map(|(name, _)| name))?;
        let location = location.to(&right_paren.location);

        Ok((
            Pattern {
                tag,
                parameters,
                returns,
            },
            location,
        ))
    }

    fn covariable(&mut self) -> Result<Consumer<String>> {
        let (name, location) = self.identifier()?;
        Ok(Consumer {
            location,
            kind: ConsumerKind::Variable(Variable { name }),
        })
    }

    fn postfix_statement(&mut self, producer: Producer<String>) -> Result<Statement<String>> {
        match self.peek()? {
            Token {
                kind: TokenKind::Punctuation(p),
                ..
            } if p == "|" => {
                self.advance();
                let consumer = self.consumer()?;
                let location = producer.location.to(&consumer.location);
                Ok(Statement {
                    location,
                    kind: StatementKind::Cut(Cut { producer, consumer }),
                })
            }
            token => Err(Error::UnexpectedToken {
                expected: vec!["|".to_string()],
                actual: token,
            }),
        }
    }

    fn prim(&mut self) -> Result<Statement<String>> {
        let prim = self.expect_keyword("prim")?;
        self.expect_punctuation("[")?;
        let (name, _) = self.identifier()?;
        self.expect_punctuation("]")?;
        self.expect_punctuation("(")?;
        let (producers, _) = self.sep_end(";", ",", |parser| parser.producer())?;
        let (consumers, right_paren) = self.sep_end(")", ",", |parser| parser.consumer())?;
        let location = prim.location.to(&right_paren.location);
        Ok(Statement {
            location,
            kind: StatementKind::Prim(Prim {
                name,
                producers,
                consumers,
            }),
        })
    }

    fn switch(&mut self) -> Result<Statement<String>> {
        let switch = self.expect_keyword("switch")?;
        let producer = self.producer()?;
        self.expect_punctuation("{")?;
        let (branches, right_brace) = self.sep_end("}", ",", |parser| parser.branch())?;

        let location = switch.location.to(&right_brace.location);

        Ok(Statement {
            location,
            kind: StatementKind::Switch(Switch {
                scrutinee: producer,
                branches,
            }),
        })
    }

    fn branch(&mut self) -> Result<Branch<String>> {
        // Default branch
        if let Token {
            kind: TokenKind::Identifier(i),
            location,
        } = self.peek()?
        {
            if i == "_" {
                self.advance();
                self.expect_punctuation("->")?;
                let body = self.statement()?;
                let location = location.to(&body.location);

                return Ok(Branch {
                    location,
                    kind: BranchKind::DefaultBranch(body),
                });
            }
        }

        // Literal branch
        if let Token {
            kind: TokenKind::Literal(literal),
            location,
        } = self.peek()?
        {
            self.advance();
            self.expect_punctuation("->")?;
            let body = self.statement()?;
            let location = location.to(&body.location);

            return Ok(Branch {
                location,
                kind: BranchKind::LiteralBranch(LiteralBranch { literal, body }),
            });
        }

        Err(Error::UnexpectedToken {
            expected: vec!["literal".to_string(), "_".to_string()],
            actual: self.peek()?,
        })
    }

    fn sep_end<T>(
        &mut self,
        end: &str,
        sep: &str,
        parser: impl Fn(&mut Self) -> Result<T>,
    ) -> Result<(Vec<T>, Token)> {
        let mut items = Vec::new();

        while !self.eof() {
            match self.peek()? {
                Token {
                    kind: TokenKind::Punctuation(p),
                    ..
                } if p == end => {
                    let token = self.expect_punctuation(end)?;
                    return Ok((items, token));
                }
                _ => {}
            }

            // TODO: if the parser fails, probably we need the end token to recover
            let item = parser(self)?;
            items.push(item);

            match self.peek()? {
                Token {
                    kind: TokenKind::Punctuation(p),
                    ..
                } if p == sep => {
                    self.advance();
                    continue;
                }
                Token {
                    kind: TokenKind::Punctuation(p),
                    ..
                } if p == end => {
                    let token = self.expect_punctuation(end)?;
                    return Ok((items, token));
                }
                actual => {
                    return Err(Error::UnexpectedToken {
                        expected: vec![sep.to_string(), end.to_string()],
                        actual,
                    });
                }
            }
        }

        Err(Error::UnexpectedEOF {
            last: self.last_location.clone(),
        })
    }

    fn invoke(&mut self) -> Result<Statement<String>> {
        let invoke = self.expect_keyword("invoke")?;
        self.expect_punctuation("[")?;
        let (name, _) = self.identifier()?;
        self.expect_punctuation("]")?;

        self.expect_punctuation("(")?;
        let (producers, _) = self.sep_end(";", ",", |parser| parser.producer())?;
        let (consumers, right_paren) = self.sep_end(")", ",", |parser| parser.consumer())?;
        let location = invoke.location.to(&right_paren.location);
        Ok(Statement {
            location,
            kind: StatementKind::Invoke(Invoke {
                name,
                producers,
                consumers,
            }),
        })
    }
}
