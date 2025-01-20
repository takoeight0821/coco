use crate::location;

// Core IR
#[derive(Debug)]
pub struct Producer<N> {
    pub location: location::Location,
    pub kind: ProducerKind<N>,
}

#[derive(Debug)]
pub enum ProducerKind<N> {
    Variable {
        name: N,
    },
    Literal {
        literal: Literal,
    },
    Do {
        name: N,
        body: Box<Statement<N>>,
    },
    Construct {
        tag: String,
        producers: Vec<Producer<N>>,
        consumers: Vec<Consumer<N>>,
    },
    Comatch {
        clauses: Vec<Coclause<N>>,
    },
}

#[derive(Debug)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(String),
}

#[derive(Debug)]
pub struct Coclause<N> {
    pub location: location::Location,
    pub copattern: Copattern<N>,
    pub body: Statement<N>,
}

#[derive(Debug)]
pub struct Copattern<N> {
    pub tag: String,
    pub parameters: Vec<N>,
    pub returns: Vec<N>,
}

#[derive(Debug)]
pub struct Consumer<N> {
    pub location: location::Location,
    pub kind: ConsumerKind<N>,
}

#[derive(Debug)]
pub enum ConsumerKind<N> {
    Finish,
    Variable {
        name: N,
    },
    Then {
        name: N,
        body: Box<Statement<N>>,
    },
    Destruct {
        tag: String,
        producers: Vec<Producer<N>>,
        consumers: Vec<Consumer<N>>,
    },
    Match {
        clauses: Vec<Clause<N>>,
    },
}

#[derive(Debug)]
pub struct Clause<N> {
    pub location: location::Location,
    pub pattern: Pattern<N>,
    pub body: Statement<N>,
}

#[derive(Debug)]
pub struct Pattern<N> {
    pub tag: String,
    pub parameters: Vec<N>,
    pub returns: Vec<N>,
}

#[derive(Debug)]
pub struct Statement<N> {
    pub location: location::Location,
    pub kind: StatementKind<N>,
}

#[derive(Debug)]
pub enum StatementKind<N> {
    Cut {
        producer: Producer<N>,
        consumer: Consumer<N>,
    },
    Prim {
        name: String,
        producers: Vec<Producer<N>>,
        consumers: Vec<Consumer<N>>,
    },
    Switch {
        scrutinee: Producer<N>,
        branches: Vec<Branch<N>>,
    },
    Invoke {
        name: N,
        producers: Vec<Producer<N>>,
        consumers: Vec<Consumer<N>>,
    },
}

#[derive(Debug)]
pub struct Branch<N> {
    pub location: location::Location,
    pub kind: BranchKind<N>,
}

#[derive(Debug)]
pub enum BranchKind<N> {
    Litearl {
        literal: Literal,
        body: Statement<N>,
    },
    Default {
        body: Statement<N>,
    },
}

#[derive(Debug)]
pub struct Definition<N> {
    pub location: location::Location,
    pub name: N,
    pub parameters: Vec<N>,
    pub returns: Vec<N>,
    pub body: Statement<N>,
}

pub type Program<N> = Vec<Definition<N>>;
