%expect 0

%define api.parser.struct {Parser}
%define api.value.type {Value}
%define api.parser.check_debug { self.debug }

%define parse.error custom
%define parse.trace

%code use {
// all use goes here
 use crate::lexer::Lexer;
    use crate::lexer::Token;
    use crate::loc::Loc;
    use crate::value::*;
    use std::path::PathBuf;
    macro_rules! map_vals {
        ($e: expr, $p: ident) => {
            $e.iter().map(|v| $p::from(v.to_owned())).collect::<Vec<$p>>()
        };
    }
}

%code parser_fields {
    result: Option<i32>,
    /// Just an extra field for demonstration
    pub name: String,
    /// Enables debug printing
    pub debug: bool,
    pub program: Value
}

%code {
    // code
    
}

%token
    tPLUS   "+"
    tMINUS  "-"
    tMUL    "*"
    tDIV    "/"
    tLPAREN "("
    tRPAREN ")"
    tLBRACK "{"
    tRBRACK "}"
    tCOLON  ":"
    tPERIOD "."
    tAMPERSAND  "&"
    tSEMICOLON ";"
    tPATHSEP"::"
    tCOMMA  ","
    tASSIGN "="
    tGT     ">"
    tLT    "<"
    tLET    "let"
    tSTR    "str"
    tI32    "i32"
    tI64    "i64"
    tBOOL   "true or fals"
    kwBOOL  "bool"
    tU32    "u32"
    tU64    "u64"
    tI8     "i8"
    tIF     "if"
    tELSE   "else"
    tSTRING "Text wrapped in quotes"
    kwRETURN "return"
    kwSTRUCT "struct"
    kwIMPORT "import"
    kwNEW   "new"
    kwEXTERN "extern"

    tINFER "_"
    tIDENTIFIER "local variable or method"
    tNUM    "number"
    tFN     "fn"
    tERROR  "controlled YYERROR"
    tABORT  "controlled YYABORT"
    tACCEPT "controlled YYACCEPT"
%type <Value> ty item items program function ident ty_ident ty_idents none stmt stmts expr 
%%
program: items {
    self.result = Some(1);
    let m = $<ValueList>1.iter().map(|v| Item::from(v.to_owned())).collect::<Vec<_>>();
    let prog = Value::Program(Program(m));
    self.program = prog.clone();
    $$ = prog;
}
item: function {
    $$ = Value::Item(Item::Function($<Function>1))
}

items: item {
    $$ = Value::ValueList(vec![$1])
} | items item {
    let mut v = $<ValueList>1;
    v.push($2);
    $$ = Value::ValueList(v)
}
none: {
    $$ = Value::None
 }
function: ty ident tLPAREN ty_idents tRPAREN expr{
    $$ = Value::Function(Function{
        ty: $<Ty>1,
        name: $<Identifier>2,
        params: map_vals!($<ValueList>4, TypedIdent),
        expr: $<Expr>6
    })
}
ty_idents: none {
    $$ = Value::ValueList(vec![])
} | ty_ident {
    $$ = Value::ValueList(vec![$1])
} | ty_idents tCOMMA ty_ident {
    let mut v = $<ValueList>1;
    v.push($3);
    $$ = Value::ValueList(v)
}

ty_ident: ty ident {
    $$ = Value::TypedIdent(TypedIdent($<Ty>1, $<Identifier>2))
}
ty: tI32 {
    $$ = Value::Ty(Ty::I32)
} | tI64 {
    $$ = Value::Ty(Ty::I64)
} | ident {
    $$ = Value::Ty(Ty::Identifier($<Identifier>1))
}
ident: tIDENTIFIER {
    $$ = Value::Identifier(Identifier($<Token>1.token_value))
}


// Expr and statement

expr: tLBRACK stmts tRBRACK {
    $$ = Value::Expr(Expr::Block(map_vals!($<ValueList>2, Statement)))
} | ident tLPAREN tRPAREN {
    $$ = Value::Expr(Expr::FunctionCall($<Identifier>1))
}
 | tNUM {
    $$ = Value::Expr(Expr::Literal($<Token>1.token_value.parse().unwrap()))
}

stmts: none {
    $$ = Value::ValueList(vec![])
} | stmt {
    $$ = Value::ValueList(vec![$1])
} | stmts tSEMICOLON stmt {
    let mut v = $<ValueList>1;
    v.push($3);
    $$ = Value::ValueList(v)
}
stmt: kwRETURN expr {
    $$ = Value::Statement(Statement::Return($<Expr>2))
}
%%

impl Parser {
    /// "Sucess" status-code of the parser
    pub const ACCEPTED: i32 = -1;

    /// "Failure" status-code of the parser
    pub const ABORTED: i32 = -2;

    /// Constructor
    pub fn new(lexer: Lexer, name: &str) -> Self {
        Self {
            yy_error_verbose: true,
            yynerrs: 0,
            debug: false,
            yyerrstatus_: 0,
            yylexer: lexer,
            result: None,
            name: name.to_owned(),
            program: Value::Uninitialized
        }
    }

    /// Wrapper around generated `parse` method that also
    /// extracts `result` field and returns it.
    pub fn do_parse(mut self) -> (Option<i32>, String) {
        self.parse();
        (self.result, self.name)
    }

    fn next_token(&mut self) -> Token {
        self.yylexer.yylex()
    }

    
}