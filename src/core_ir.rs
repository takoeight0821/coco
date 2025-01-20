use crate::location;

// Core IR
#[derive(Debug)]
pub struct Producer<N> {
    pub location: location::Location,
    pub kind: ProducerKind<N>,
}

#[derive(Debug)]
pub struct Variable<N> {
    pub name: N,
}

#[derive(Debug)]
pub struct Do<N> {
    pub name: N,
    pub body: Box<Statement<N>>,
}

#[derive(Debug)]
pub struct Construct<N> {
    pub tag: String,
    pub producers: Vec<Producer<N>>,
    pub consumers: Vec<Consumer<N>>,
}

#[derive(Debug)]
pub struct Comatch<N> {
    pub clauses: Vec<Coclause<N>>,
}

#[derive(Debug)]
pub enum ProducerKind<N> {
    Variable(Variable<N>),
    Literal(Literal),
    Do(Do<N>),
    Construct(Construct<N>),
    Comatch(Comatch<N>),
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
pub struct Then<N> {
    pub name: N,
    pub body: Box<Statement<N>>,
}

#[derive(Debug)]
pub struct Destruct<N> {
    pub tag: String,
    pub producers: Vec<Producer<N>>,
    pub consumers: Vec<Consumer<N>>,
}

#[derive(Debug)]
pub struct Match<N> {
    pub clauses: Vec<Clause<N>>,
}

#[derive(Debug)]
pub enum ConsumerKind<N> {
    Finish,
    Variable(Variable<N>),
    Then(Then<N>),
    Destruct(Destruct<N>),
    Match(Match<N>),
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
pub struct Cut<N> {
    pub producer: Producer<N>,
    pub consumer: Consumer<N>,
}

#[derive(Debug)]
pub struct Prim<N> {
    pub name: String,
    pub producers: Vec<Producer<N>>,
    pub consumers: Vec<Consumer<N>>,
}

#[derive(Debug)]
pub struct Switch<N> {
    pub scrutinee: Producer<N>,
    pub branches: Vec<Branch<N>>,
}

#[derive(Debug)]
pub struct Invoke<N> {
    pub name: N,
    pub producers: Vec<Producer<N>>,
    pub consumers: Vec<Consumer<N>>,
}

#[derive(Debug)]
pub enum StatementKind<N> {
    Cut(Cut<N>),
    Prim(Prim<N>),
    Switch(Switch<N>),
    Invoke(Invoke<N>),
}

#[derive(Debug)]
pub struct Branch<N> {
    pub location: location::Location,
    pub kind: BranchKind<N>,
}

#[derive(Debug)]
pub struct LiteralBranch<N> {
    pub literal: Literal,
    pub body: Statement<N>,
}

#[derive(Debug)]
pub enum BranchKind<N> {
    LiteralBranch(LiteralBranch<N>),
    DefaultBranch(Statement<N>),
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
