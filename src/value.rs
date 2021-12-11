pub(crate) use crate::ast::*;
use crate::lexer::Token;
#[derive(Debug, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]

pub enum Value {
    Stolen,
    Uninitialized,
    None,
    Token(Token),
    Result(i32),
    Ty(Ty),
    Item(Item),
    Function(Function),
    Identifier(Identifier),
    Program(Program),
    ValueList(Vec<Value>),
    TypedIdent(TypedIdent),
    Statement(Statement),
    Expr(Expr)
}

macro_rules! impl_from {
    ($e: ident, $r: path, $t: ty, $z: expr) => {
        #[allow(non_snake_case)]
        impl $e {
            pub(crate) fn from(value: Value) -> $t {
                let vc = value.clone();
                match value {
                    $r(s) => $z(s),
                    other => panic!("wrong type, expected {:?}, got {:?}", vc, other),
                }
            }
        }
    };
}
macro_rules! mod_from1 {
    ($e: ident, $r: path, $t: ty, $z: expr) => {
        #[allow(non_snake_case)]
        pub mod $e {
            use super::Value;
            pub(crate) fn from(value: Value) -> $t {
                let vc = value.clone();
                match value {
                    $r(a) => $z(a),
                    other => panic!("wrong type, expected {:?}, got {:?}", vc, other),
                }
            }
        }
    };
}

impl_from!(
    Identifier,
    Value::Identifier,
    Identifier,
    |s: Identifier| { s }
);
impl_from!(Ty, Value::Ty, Ty, |s: Ty| { s });
impl_from!(Item, Value::Item, Item, |s: Item| { s });
impl_from!(Function, Value::Function, Function, |s: Function| { s });
impl_from!(
    TypedIdent,
    Value::TypedIdent,
    TypedIdent,
    |s: TypedIdent| { s }
);
impl_from!(Program, Value::Program, Program, |s: Program| { s });
impl_from!(Expr, Value::Expr, Expr, |s: Expr| { s });
impl_from!(Statement, Value::Statement, Statement, |s: Statement| { s });

mod_from1!(ValueList, Value::ValueList, Vec<Value>, |s: Vec<Value>| {
    s
});

// impl_from!(Ty, Value::Ty, Ty, |s: Ty| { s });

impl Default for Value {
    fn default() -> Self {
        Self::Stolen
    }
}
impl Value {
    /// Required method, parser expects it to be defined.
    ///
    /// Constructor for `Value::Token(token)` variant.
    pub(crate) fn from_token(value: Token) -> Self {
        Self::Token(value)
    }
    pub fn to_token(&self) -> Token {
        if let Self::Token(t) = self {
            return t.to_owned();
        } else {
            panic!("called `to_token` on a non-token value. {:#?}", self);
        }
    }
}
