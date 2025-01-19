use crate::location;

// Core IR
pub struct Producer<N> {
    pub location: location::Location,
    pub kind: ProducerKind<N>,
}

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

pub struct Coclause<N> {
    pub location: location::Location,
    pub copattern: Copattern<N>,
    pub body: Statement<N>,
}

pub struct Copattern<N> {
    pub tag: String,
    pub producers: Vec<N>,
    pub consumers: Vec<N>,
}

pub struct Consumer<N> {
    pub location: location::Location,
    pub kind: ConsumerKind<N>,
}

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

pub struct Clause<N> {
    pub location: location::Location,
    pub pattern: Pattern<N>,
    pub body: Statement<N>,
}

pub struct Pattern<N> {
    pub tag: String,
    pub producers: Vec<N>,
    pub consumers: Vec<N>,
}

pub struct Statement<N> {
    pub location: location::Location,
    pub kind: StatementKind<N>,
}

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
        default: Box<Statement<N>>,
    },
    Invoke {
        name: N,
        producers: Vec<Producer<N>>,
        consumers: Vec<Consumer<N>>,
    },
}

pub struct Branch<N> {
    pub location: location::Location,
    pub literal: Literal,
    pub body: Statement<N>,
}

pub struct Definition<N> {
    pub location: location::Location,
    pub name: N,
    pub parameters: Vec<N>,
    pub returns: Vec<N>,
    pub body: Statement<N>,
}

pub type Program<N> = Vec<Definition<N>>;
