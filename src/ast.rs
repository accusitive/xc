#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Ty {
    I32,
    I64,
    Identifier(Identifier),
}
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Item {
    Function(Function),
}
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Function {
    pub name: Identifier,
    pub ty: Ty,
    pub params: Vec<TypedIdent>,
    pub expr: Expr
}
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Identifier(pub String);
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct TypedIdent(pub Ty, pub Identifier);
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Program(pub Vec<Item>);
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Statement {
    Return(Expr)
}
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expr {
    Block(Vec<Statement>),
    Literal(u64),
    FunctionCall(Identifier)
}