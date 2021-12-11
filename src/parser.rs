/* A Bison parser, made by GNU Bison 3.8.2.  */

/* Skeleton implementation for Bison LALR(1) parsers in Rust

   Copyright (C) 2007-2015, 2018-2020 Free Software Foundation, Inc.

   This program is free software: you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation, either version 3 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program.  If not, see <https://www.gnu.org/licenses/>.  */

/* As a special exception, you may create a larger work that contains
   part or all of the Bison parser skeleton and distribute that work
   under terms of your choice, so long as that work isn't itself a
   parser generator using the skeleton or a modified version thereof
   as a parser skeleton.  Alternatively, if you modify or redistribute
   the parser skeleton itself, you may (at your option) remove this
   special exception, which will cause the skeleton and the resulting
   Bison output files to be licensed under the GNU General Public
   License without this special exception.

   This special exception was added by the Free Software Foundation in
   version 2.2 of Bison.  */

/* DO NOT RELY ON FEATURES THAT ARE NOT DOCUMENTED in the manual,
   especially those whose name start with YY_ or yy_.  They are
   private implementation details that can be changed or removed.  */




use std::convert::TryInto;


/* "%code use" blocks.  */
/* "src/parser.y":10  */

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

/* "src/parser.rs":59  */


/// A Bison parser, automatically generated from src/parser.y.
#[derive(Debug)]
pub struct Parser {
    /// Lexer that is used to get tokens
    pub yylexer: Lexer,
    // true if verbose error messages are enabled.
    #[allow(dead_code)]
    yy_error_verbose: bool,
    // number of errors so far
    yynerrs: i32,

    yyerrstatus_: i32,

    /* "%code parser_fields" blocks.  */
/* "src/parser.y":24  */

    result: Option<i32>,
    /// Just an extra field for demonstration
    pub name: String,
    /// Enables debug printing
    pub debug: bool,
    pub program: Value

/* "src/parser.rs":85  */

}

#[inline]
fn i32_to_usize(v: i32) -> usize {
    v as usize
}

/// Maps token ID into human-readable name
pub fn token_name(id: i32) -> &'static str { /* ' */
    let first_token = Lexer::YYerror;
    if id > first_token + 1 {
        let pos: usize = (id - first_token + 1)
            .try_into()
            .expect("failed to cast token id into usize, is it negative?");
        Lexer::TOKEN_NAMES[pos]
    } else if id == 0 {
        "EOF"
    } else {
        panic!("token_name fails, {} (first token = {})", id, first_token)
    }
}

/// Local alias
type YYLoc = Loc;

impl Parser {
    // Version number for the Bison executable that generated this parser.
    #[allow(dead_code)]
    const BISON_VERSION: &'static str = "30802";

}


fn make_yylloc(rhs: &YYStack, n: usize) -> YYLoc {
    if 0 < n {
        YYLoc {
            begin: rhs.location_at(n - 1).begin,
            end: rhs.location_at(0).end
        }
    } else {
        YYLoc {
            begin: rhs.location_at(0).end,
            end: rhs.location_at(0).end
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct SymbolKind { value: i32 }

impl SymbolKind {



    #[allow(non_upper_case_globals)]
    const S_YYEOF: i32 = 0;        /* "end of file"  */

    #[allow(non_upper_case_globals)]
    const S_YYerror: i32 = 1;      /* error  */

    #[allow(non_upper_case_globals)]
    const S_YYUNDEF: i32 = 2;      /* "invalid token"  */

    #[allow(non_upper_case_globals)]
    const S_tPLUS: i32 = 3;        /* "+"  */

    #[allow(non_upper_case_globals)]
    const S_tMINUS: i32 = 4;       /* "-"  */

    #[allow(non_upper_case_globals)]
    const S_tMUL: i32 = 5;         /* "*"  */

    #[allow(non_upper_case_globals)]
    const S_tDIV: i32 = 6;         /* "/"  */

    #[allow(non_upper_case_globals)]
    const S_tLPAREN: i32 = 7;      /* "("  */

    #[allow(non_upper_case_globals)]
    const S_tRPAREN: i32 = 8;      /* ")"  */

    #[allow(non_upper_case_globals)]
    const S_tLBRACK: i32 = 9;      /* "{"  */

    #[allow(non_upper_case_globals)]
    const S_tRBRACK: i32 = 10;     /* "}"  */

    #[allow(non_upper_case_globals)]
    const S_tCOLON: i32 = 11;      /* ":"  */

    #[allow(non_upper_case_globals)]
    const S_tPERIOD: i32 = 12;     /* "."  */

    #[allow(non_upper_case_globals)]
    const S_tAMPERSAND: i32 = 13;  /* "&"  */

    #[allow(non_upper_case_globals)]
    const S_tSEMICOLON: i32 = 14;  /* ";"  */

    #[allow(non_upper_case_globals)]
    const S_tPATHSEP: i32 = 15;    /* "::"  */

    #[allow(non_upper_case_globals)]
    const S_tCOMMA: i32 = 16;      /* ","  */

    #[allow(non_upper_case_globals)]
    const S_tASSIGN: i32 = 17;     /* "="  */

    #[allow(non_upper_case_globals)]
    const S_tGT: i32 = 18;         /* ">"  */

    #[allow(non_upper_case_globals)]
    const S_tLT: i32 = 19;         /* "<"  */

    #[allow(non_upper_case_globals)]
    const S_tLET: i32 = 20;        /* "let"  */

    #[allow(non_upper_case_globals)]
    const S_tSTR: i32 = 21;        /* "str"  */

    #[allow(non_upper_case_globals)]
    const S_tI32: i32 = 22;        /* "i32"  */

    #[allow(non_upper_case_globals)]
    const S_tI64: i32 = 23;        /* "i64"  */

    #[allow(non_upper_case_globals)]
    const S_tBOOL: i32 = 24;       /* "true or fals"  */

    #[allow(non_upper_case_globals)]
    const S_kwBOOL: i32 = 25;      /* "bool"  */

    #[allow(non_upper_case_globals)]
    const S_tU32: i32 = 26;        /* "u32"  */

    #[allow(non_upper_case_globals)]
    const S_tU64: i32 = 27;        /* "u64"  */

    #[allow(non_upper_case_globals)]
    const S_tI8: i32 = 28;         /* "i8"  */

    #[allow(non_upper_case_globals)]
    const S_tIF: i32 = 29;         /* "if"  */

    #[allow(non_upper_case_globals)]
    const S_tELSE: i32 = 30;       /* "else"  */

    #[allow(non_upper_case_globals)]
    const S_tSTRING: i32 = 31;     /* "Text wrapped in quotes"  */

    #[allow(non_upper_case_globals)]
    const S_kwRETURN: i32 = 32;    /* "return"  */

    #[allow(non_upper_case_globals)]
    const S_kwSTRUCT: i32 = 33;    /* "struct"  */

    #[allow(non_upper_case_globals)]
    const S_kwIMPORT: i32 = 34;    /* "import"  */

    #[allow(non_upper_case_globals)]
    const S_kwNEW: i32 = 35;       /* "new"  */

    #[allow(non_upper_case_globals)]
    const S_kwEXTERN: i32 = 36;    /* "extern"  */

    #[allow(non_upper_case_globals)]
    const S_tINFER: i32 = 37;      /* "_"  */

    #[allow(non_upper_case_globals)]
    const S_tIDENTIFIER: i32 = 38; /* "local variable or method"  */

    #[allow(non_upper_case_globals)]
    const S_tNUM: i32 = 39;        /* "number"  */

    #[allow(non_upper_case_globals)]
    const S_tFN: i32 = 40;         /* "fn"  */

    #[allow(non_upper_case_globals)]
    const S_tERROR: i32 = 41;      /* "controlled YYERROR"  */

    #[allow(non_upper_case_globals)]
    const S_tABORT: i32 = 42;      /* "controlled YYABORT"  */

    #[allow(non_upper_case_globals)]
    const S_tACCEPT: i32 = 43;     /* "controlled YYACCEPT"  */

    #[allow(non_upper_case_globals)]
    const S_YYACCEPT: i32 = 44;    /* $accept  */

    #[allow(non_upper_case_globals)]
    const S_program: i32 = 45;     /* program  */

    #[allow(non_upper_case_globals)]
    const S_item: i32 = 46;        /* item  */

    #[allow(non_upper_case_globals)]
    const S_items: i32 = 47;       /* items  */

    #[allow(non_upper_case_globals)]
    const S_none: i32 = 48;        /* none  */

    #[allow(non_upper_case_globals)]
    const S_function: i32 = 49;    /* function  */

    #[allow(non_upper_case_globals)]
    const S_ty_idents: i32 = 50;   /* ty_idents  */

    #[allow(non_upper_case_globals)]
    const S_ty_ident: i32 = 51;    /* ty_ident  */

    #[allow(non_upper_case_globals)]
    const S_ty: i32 = 52;          /* ty  */

    #[allow(non_upper_case_globals)]
    const S_ident: i32 = 53;       /* ident  */

    #[allow(non_upper_case_globals)]
    const S_expr: i32 = 54;        /* expr  */

    #[allow(non_upper_case_globals)]
    const S_stmts: i32 = 55;       /* stmts  */

    #[allow(non_upper_case_globals)]
    const S_stmt: i32 = 56;        /* stmt  */


    const VALUES_: &'static [SymbolKind] = &[ 
        SymbolKind { value: SymbolKind::S_YYEOF },
        SymbolKind { value: SymbolKind::S_YYerror },
        SymbolKind { value: SymbolKind::S_YYUNDEF },
        SymbolKind { value: SymbolKind::S_tPLUS },
        SymbolKind { value: SymbolKind::S_tMINUS },
        SymbolKind { value: SymbolKind::S_tMUL },
        SymbolKind { value: SymbolKind::S_tDIV },
        SymbolKind { value: SymbolKind::S_tLPAREN },
        SymbolKind { value: SymbolKind::S_tRPAREN },
        SymbolKind { value: SymbolKind::S_tLBRACK },
        SymbolKind { value: SymbolKind::S_tRBRACK },
        SymbolKind { value: SymbolKind::S_tCOLON },
        SymbolKind { value: SymbolKind::S_tPERIOD },
        SymbolKind { value: SymbolKind::S_tAMPERSAND },
        SymbolKind { value: SymbolKind::S_tSEMICOLON },
        SymbolKind { value: SymbolKind::S_tPATHSEP },
        SymbolKind { value: SymbolKind::S_tCOMMA },
        SymbolKind { value: SymbolKind::S_tASSIGN },
        SymbolKind { value: SymbolKind::S_tGT },
        SymbolKind { value: SymbolKind::S_tLT },
        SymbolKind { value: SymbolKind::S_tLET },
        SymbolKind { value: SymbolKind::S_tSTR },
        SymbolKind { value: SymbolKind::S_tI32 },
        SymbolKind { value: SymbolKind::S_tI64 },
        SymbolKind { value: SymbolKind::S_tBOOL },
        SymbolKind { value: SymbolKind::S_kwBOOL },
        SymbolKind { value: SymbolKind::S_tU32 },
        SymbolKind { value: SymbolKind::S_tU64 },
        SymbolKind { value: SymbolKind::S_tI8 },
        SymbolKind { value: SymbolKind::S_tIF },
        SymbolKind { value: SymbolKind::S_tELSE },
        SymbolKind { value: SymbolKind::S_tSTRING },
        SymbolKind { value: SymbolKind::S_kwRETURN },
        SymbolKind { value: SymbolKind::S_kwSTRUCT },
        SymbolKind { value: SymbolKind::S_kwIMPORT },
        SymbolKind { value: SymbolKind::S_kwNEW },
        SymbolKind { value: SymbolKind::S_kwEXTERN },
        SymbolKind { value: SymbolKind::S_tINFER },
        SymbolKind { value: SymbolKind::S_tIDENTIFIER },
        SymbolKind { value: SymbolKind::S_tNUM },
        SymbolKind { value: SymbolKind::S_tFN },
        SymbolKind { value: SymbolKind::S_tERROR },
        SymbolKind { value: SymbolKind::S_tABORT },
        SymbolKind { value: SymbolKind::S_tACCEPT },
        SymbolKind { value: SymbolKind::S_YYACCEPT },
        SymbolKind { value: SymbolKind::S_program },
        SymbolKind { value: SymbolKind::S_item },
        SymbolKind { value: SymbolKind::S_items },
        SymbolKind { value: SymbolKind::S_none },
        SymbolKind { value: SymbolKind::S_function },
        SymbolKind { value: SymbolKind::S_ty_idents },
        SymbolKind { value: SymbolKind::S_ty_ident },
        SymbolKind { value: SymbolKind::S_ty },
        SymbolKind { value: SymbolKind::S_ident },
        SymbolKind { value: SymbolKind::S_expr },
        SymbolKind { value: SymbolKind::S_stmts },
        SymbolKind { value: SymbolKind::S_stmt }
    ];

    pub(crate) fn get(n: i32) -> &'static SymbolKind {
        &Self::VALUES_[i32_to_usize(n)]
    }

    pub(crate) fn code(&self) -> i32 {
        self.value
    }

    /* YYNAMES_[SYMBOL-NUM] -- String name of the symbol SYMBOL-NUM.
    First, the terminals, then, starting at \a YYNTOKENS_, nonterminals.  */
    #[allow(non_upper_case_globals)]
const yynames_: &'static [&'static str] = &[ "end of file", "error", "invalid token", "+", "-", "*", "/", "(", ")",
  "{", "}", ":", ".", "&", ";", "::", ",", "=", ">", "<", "let", "str",
  "i32", "i64", "true or fals", "bool", "u32", "u64", "i8", "if", "else",
  "Text wrapped in quotes", "return", "struct", "import", "new", "extern",
  "_", "local variable or method", "number", "fn", "controlled YYERROR",
  "controlled YYABORT", "controlled YYACCEPT", "$accept", "program",
  "item", "items", "none", "function", "ty_idents", "ty_ident", "ty",
  "ident", "expr", "stmts", "stmt", "<<NULL>>" ] ;

    /* The user-facing name of this symbol.  */
    pub(crate) fn name(&self) -> String {
        let code: usize = self.code().try_into().unwrap();
        Self::yynames_[code].to_owned()
    }
}


const DYMMY_SYMBOL_KIND: SymbolKind = SymbolKind { value: 0 };

impl Lexer {
        /* Token kinds.  */
    /// Token `` "end of file" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const YYEOF: i32 = 0;
    /// Token `` error ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const YYerror: i32 = 256;
    /// Token `` "invalid token" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const YYUNDEF: i32 = 257;
    /// Token `` "+" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tPLUS: i32 = 258;
    /// Token `` "-" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tMINUS: i32 = 259;
    /// Token `` "*" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tMUL: i32 = 260;
    /// Token `` "/" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tDIV: i32 = 261;
    /// Token `` "(" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tLPAREN: i32 = 262;
    /// Token `` ")" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tRPAREN: i32 = 263;
    /// Token `` "{" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tLBRACK: i32 = 264;
    /// Token `` "}" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tRBRACK: i32 = 265;
    /// Token `` ":" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tCOLON: i32 = 266;
    /// Token `` "." ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tPERIOD: i32 = 267;
    /// Token `` "&" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tAMPERSAND: i32 = 268;
    /// Token `` ";" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tSEMICOLON: i32 = 269;
    /// Token `` "::" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tPATHSEP: i32 = 270;
    /// Token `` "," ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tCOMMA: i32 = 271;
    /// Token `` "=" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tASSIGN: i32 = 272;
    /// Token `` ">" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tGT: i32 = 273;
    /// Token `` "<" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tLT: i32 = 274;
    /// Token `` "let" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tLET: i32 = 275;
    /// Token `` "str" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tSTR: i32 = 276;
    /// Token `` "i32" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tI32: i32 = 277;
    /// Token `` "i64" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tI64: i32 = 278;
    /// Token `` "true or fals" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tBOOL: i32 = 279;
    /// Token `` "bool" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kwBOOL: i32 = 280;
    /// Token `` "u32" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tU32: i32 = 281;
    /// Token `` "u64" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tU64: i32 = 282;
    /// Token `` "i8" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tI8: i32 = 283;
    /// Token `` "if" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tIF: i32 = 284;
    /// Token `` "else" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tELSE: i32 = 285;
    /// Token `` "Text wrapped in quotes" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tSTRING: i32 = 286;
    /// Token `` "return" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kwRETURN: i32 = 287;
    /// Token `` "struct" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kwSTRUCT: i32 = 288;
    /// Token `` "import" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kwIMPORT: i32 = 289;
    /// Token `` "new" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kwNEW: i32 = 290;
    /// Token `` "extern" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const kwEXTERN: i32 = 291;
    /// Token `` "_" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tINFER: i32 = 292;
    /// Token `` "local variable or method" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tIDENTIFIER: i32 = 293;
    /// Token `` "number" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tNUM: i32 = 294;
    /// Token `` "fn" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tFN: i32 = 295;
    /// Token `` "controlled YYERROR" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tERROR: i32 = 296;
    /// Token `` "controlled YYABORT" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tABORT: i32 = 297;
    /// Token `` "controlled YYACCEPT" ``, to be returned by the scanner.
    #[allow(non_upper_case_globals, dead_code)]
    pub const tACCEPT: i32 = 298;


    // Deprecated, use YYEOF instead.
    #[allow(dead_code)]
    const EOF: i32 = Self::YYEOF;

    // Token values
    #[allow(dead_code)]
    pub(crate) const TOKEN_NAMES: &'static [&'static str] = &    [

    "YYEOF",

    "YYerror",

    "YYUNDEF",

    "tPLUS",

    "tMINUS",

    "tMUL",

    "tDIV",

    "tLPAREN",

    "tRPAREN",

    "tLBRACK",

    "tRBRACK",

    "tCOLON",

    "tPERIOD",

    "tAMPERSAND",

    "tSEMICOLON",

    "tPATHSEP",

    "tCOMMA",

    "tASSIGN",

    "tGT",

    "tLT",

    "tLET",

    "tSTR",

    "tI32",

    "tI64",

    "tBOOL",

    "kwBOOL",

    "tU32",

    "tU64",

    "tI8",

    "tIF",

    "tELSE",

    "tSTRING",

    "kwRETURN",

    "kwSTRUCT",

    "kwIMPORT",

    "kwNEW",

    "kwEXTERN",

    "tINFER",

    "tIDENTIFIER",

    "tNUM",

    "tFN",

    "tERROR",

    "tABORT",

    "tACCEPT",

]
;
}


impl Parser {

    fn yycdebug(&self, s: &str) {
        if  self.debug  {
            eprintln!("{}", s);
        }
    }

}

/// Local alias
type YYValue = Value;

#[derive(Debug)]
pub struct YYStackItem {
    pub state: i32,
    pub value: YYValue,
    pub loc: YYLoc,
}

#[derive(Debug)]
pub struct YYStack {
    pub stack: Vec<YYStackItem>,
}

impl YYStack {
    pub(crate) fn new() -> Self {
        Self {
          stack: Vec::with_capacity(20),
        }
    }

    pub(crate) fn push(&mut self, state: i32, value: YYValue, loc: YYLoc) {
        self.stack.push(YYStackItem { state, value, loc });
    }

    pub(crate) fn pop(&mut self) {
        self.stack.pop();
    }

    pub(crate) fn pop_n(&mut self, num: usize) {
        let len = self.stack.len() - num;
        self.stack.truncate(len);
    }

    pub(crate) fn state_at(&self, i: usize) -> i32 {
        self.stack[self.len() - 1 - i].state
    }

    pub(crate) fn location_at(&self, i: usize) -> &YYLoc {
        &self.stack[self.len() - 1 - i].loc
    }

    pub(crate) fn borrow_value_at(&self, i: usize) -> &YYValue {
        &self.stack[self.len() - 1 - i].value
    }

    pub(crate) fn owned_value_at(&mut self, i: usize) -> YYValue {
        let len = self.len();
        std::mem::take(&mut self.stack[len - 1 - i].value)
    }

    pub(crate) fn len(&self) -> usize {
      self.stack.len()
    }
}

impl std::fmt::Display for YYStack {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let states = self.stack.iter().map(|e| e.state.to_string()).collect::<Vec<String>>().join(" ");
        let values = self.stack.iter().map(|e| format!("{:?}", e.value)).collect::<Vec<String>>().join(" ");
        f.write_fmt(format_args!("Stack now states = {} / values = {:?} ", states, values))
    }
}

impl Parser {
  /// Returned by a Bison action in order to stop the parsing process and
  /// return success (true).
  pub(crate) const YYACCEPT: i32 = 0;

  /// Returned by a Bison action in order to stop the parsing process and
  /// return failure (false).
  pub(crate) const YYABORT: i32 = 1;

  /// Returned by a Bison action in order to start error recovery without
  /// printing an error message.
  pub(crate) const YYERROR: i32 = 2;

  /// Internal return codes that are not supported for user semantic
  /// actions.
  pub(crate) const YYERRLAB: i32 = 3;
  pub(crate) const YYNEWSTATE: i32 = 4;
  pub(crate) const YYDEFAULT: i32 = 5;
  pub(crate) const YYREDUCE: i32 = 6;
  pub(crate) const YYERRLAB1: i32 = 7;
  #[allow(dead_code)]
  pub(crate) const YYRETURN: i32 = 8;

  /// Whether error recovery is being done.  In this state, the parser
  /// reads token until it reaches a known state, and then restarts normal
  /// operation.
  #[allow(dead_code)]
  pub(crate) fn recovering(&self) -> bool {
      self.yyerrstatus_ == 0
  }

    // Compute post-reduction state.
    // yystate:   the current state
    // yysym:     the nonterminal to push on the stack
    fn yy_lr_goto_state(&self, yystate: i32, yysym: i32) -> i32 {
        let idx = i32_to_usize(yysym - Self::YYNTOKENS_);
        let yyr = Self::yypgoto_[idx] + yystate;
        if (0..=Self::YYLAST_).contains(&yyr) {
            let yyr = i32_to_usize(yyr);
            if Self::yycheck_[yyr] == yystate {
                return Self::yytable_[yyr];
            }
        }
        Self::yydefgoto_[idx]
    }

    fn yyaction(&mut self, yyn: i32, yystack: &mut YYStack, yylen: &mut usize) -> Result<i32, ()> {
        // If YYLEN is nonzero, implement the default value of the action:
        // '$$ = $1'.  Otherwise, use the top of the stack.
        //
        // Otherwise, the following line sets YYVAL to garbage.
        // This behavior is undocumented and Bison
        // users should not rely upon it.
        #[allow(unused_assignments)]
        let mut yyval: YYValue = YYValue::Uninitialized;
        let yyloc: YYLoc = make_yylloc(yystack, *yylen);

        self.yy_reduce_print(yyn, yystack);

        match yyn {
              2 =>  /* program: items  */
  /* "src/parser.y":83  */
               {
    self.result = Some(1);
    let m =  ValueList::from(yystack.owned_value_at(0)).iter().map(|v| Item::from(v.to_owned())).collect::<Vec<_>>();
    let prog = Value::Program(Program(m));
    self.program = prog.clone();
    yyval = prog;
},


  3 =>  /* item: function  */
  /* "src/parser.y":90  */
               {
    yyval = Value::Item(Item::Function( Function::from(yystack.owned_value_at(0))))
},


  4 =>  /* items: item  */
  /* "src/parser.y":94  */
            {
    yyval = Value::ValueList(vec![ yystack.owned_value_at(0)])
},


  5 =>  /* items: items item  */
  /* "src/parser.y":96  */
               {
    let mut v =  ValueList::from(yystack.owned_value_at(1));
    v.push( yystack.owned_value_at(0));
    yyval = Value::ValueList(v)
},


  6 =>  /* none: %empty  */
  /* "src/parser.y":101  */
      {
    yyval = Value::None
 },


  7 =>  /* function: ty ident "(" ty_idents ")" expr  */
  /* "src/parser.y":104  */
                                                 {
    yyval = Value::Function(Function{
        ty:  Ty::from(yystack.owned_value_at(5)),
        name:  Identifier::from(yystack.owned_value_at(4)),
        params: map_vals!( ValueList::from(yystack.owned_value_at(2)), TypedIdent),
        expr:  Expr::from(yystack.owned_value_at(0))
    })
},


  8 =>  /* ty_idents: none  */
  /* "src/parser.y":112  */
                {
    yyval = Value::ValueList(vec![])
},


  9 =>  /* ty_idents: ty_ident  */
  /* "src/parser.y":114  */
             {
    yyval = Value::ValueList(vec![ yystack.owned_value_at(0)])
},


  10 =>  /* ty_idents: ty_idents "," ty_ident  */
  /* "src/parser.y":116  */
                              {
    let mut v =  ValueList::from(yystack.owned_value_at(2));
    v.push( yystack.owned_value_at(0));
    yyval = Value::ValueList(v)
},


  11 =>  /* ty_ident: ty ident  */
  /* "src/parser.y":122  */
                   {
    yyval = Value::TypedIdent(TypedIdent( Ty::from(yystack.owned_value_at(1)),  Identifier::from(yystack.owned_value_at(0))))
},


  12 =>  /* ty: "i32"  */
  /* "src/parser.y":125  */
         {
    yyval = Value::Ty(Ty::I32)
},


  13 =>  /* ty: "i64"  */
  /* "src/parser.y":127  */
         {
    yyval = Value::Ty(Ty::I64)
},


  14 =>  /* ty: ident  */
  /* "src/parser.y":129  */
          {
    yyval = Value::Ty(Ty::Identifier( Identifier::from(yystack.owned_value_at(0))))
},


  15 =>  /* ident: "local variable or method"  */
  /* "src/parser.y":132  */
                   {
    yyval = Value::Identifier(Identifier( Token::from(yystack.owned_value_at(0)).token_value))
},


  16 =>  /* expr: "{" stmts "}"  */
  /* "src/parser.y":139  */
                            {
    yyval = Value::Expr(Expr::Block(map_vals!( ValueList::from(yystack.owned_value_at(1)), Statement)))
},


  17 =>  /* expr: ident "(" ")"  */
  /* "src/parser.y":141  */
                          {
    yyval = Value::Expr(Expr::FunctionCall( Identifier::from(yystack.owned_value_at(2))))
},


  18 =>  /* expr: "number"  */
  /* "src/parser.y":144  */
        {
    yyval = Value::Expr(Expr::Literal( Token::from(yystack.owned_value_at(0)).token_value.parse().unwrap()))
},


  19 =>  /* stmts: none  */
  /* "src/parser.y":148  */
            {
    yyval = Value::ValueList(vec![])
},


  20 =>  /* stmts: stmt  */
  /* "src/parser.y":150  */
         {
    yyval = Value::ValueList(vec![ yystack.owned_value_at(0)])
},


  21 =>  /* stmts: stmts ";" stmt  */
  /* "src/parser.y":152  */
                          {
    let mut v =  ValueList::from(yystack.owned_value_at(2));
    v.push( yystack.owned_value_at(0));
    yyval = Value::ValueList(v)
},


  22 =>  /* stmt: "return" expr  */
  /* "src/parser.y":157  */
                    {
    yyval = Value::Statement(Statement::Return( Expr::from(yystack.owned_value_at(0))))
},



/* "src/parser.rs":938  */

            _ => {}
        }

        if let YYValue::Uninitialized = yyval {
            panic!("yyval is Uninitialized in rule at line {}", Self::yyrline_[i32_to_usize(yyn)]);
        }

        self.yy_symbol_print("-> $$ =", SymbolKind::get(Self::yyr1_[i32_to_usize(yyn)]), &yyval, &yyloc);

        yystack.pop_n(*yylen);
        *yylen = 0;
        /* Shift the result of the reduction.  */
        let yystate = self.yy_lr_goto_state(yystack.state_at(0), Self::yyr1_[i32_to_usize(yyn)]);
        yystack.push(yystate, yyval, yyloc);
        Ok(Self::YYNEWSTATE)
    }

    // Print this symbol on YYOUTPUT.
    fn yy_symbol_print(&self, s: &str, yykind: &SymbolKind, yyvalue: &YYValue, yylocation: &YYLoc) {
        if  self.debug  {
            self.yycdebug(
                &format!("{}{} {:?} ( {:?}: {:?} )", // " fix highlighting
                s,
                if yykind.code() < Self::YYNTOKENS_ { " token " } else { " nterm " },
                yykind.name(),
                yylocation,
                yyvalue
                )
            )
        }
    }

    /// Parses given input. Returns true if the parsing was successful.
    pub fn parse(&mut self) -> bool {
        /* @$.  */
        let mut yyloc: YYLoc;
        
    /* Lookahead token kind.  */
    let mut yychar: i32 = Self::YYEMPTY_;
    /* Lookahead symbol kind.  */
    let mut yytoken = &DYMMY_SYMBOL_KIND;

    /* State.  */
    let mut yyn: i32 = 0;
    let mut yylen: usize = 0;
    let mut yystate: i32 = 0;
    let mut yystack = YYStack::new();
    let mut label: i32 = Self::YYNEWSTATE;

    /* The location where the error started.  */
    let mut yyerrloc: YYLoc = YYLoc { begin: 0, end: 0 };

    /* Location. */
    let mut yylloc: YYLoc = YYLoc { begin: 0, end: 0 };

    /* Semantic value of the lookahead.  */
    let mut yylval: YYValue = YYValue::Uninitialized;

        self.yycdebug("Starting parse");
        self.yyerrstatus_ = 0;
        self.yynerrs = 0;

        /* Initialize the stack.  */
        yystack.push(yystate, yylval.clone(), yylloc);

        loop {
            match label {
                // New state.  Unlike in the C/C++ skeletons, the state is already
                // pushed when we come here.

                Self::YYNEWSTATE => {
                    if  self.debug  {
                        self.yycdebug(&format!("Entering state {}", yystate));
                        eprintln!("{}", yystack);
                    }

                    /* Accept? */
                    if yystate == Self::YYFINAL_ {
                        return true;
                    }

                    /* Take a decision.  First try without lookahead.  */
                    yyn = Self::yypact_[i32_to_usize(yystate)];
                    if yy_pact_value_is_default(yyn) {
                        label = Self::YYDEFAULT;
                        continue;
                    }

                    /* Read a lookahead token.  */
                    if yychar == Self::YYEMPTY_ {
                        self.yycdebug("Reading a token");
                        let token = self.next_token();
                        yychar = token.token_type;
                        yylloc = token.loc;
                        yylval = YYValue::from_token(token);
                    }

                    /* Convert token to internal form.  */
                    yytoken = Self::yytranslate_(yychar);
                    self.yy_symbol_print("Next token is", yytoken, &yylval, &yylloc);

                    if yytoken == SymbolKind::get(1) {
                        // The scanner already issued an error message, process directly
                        // to error recovery.  But do not keep the error token as
                        // lookahead, it is too special and may lead us to an endless
                        // loop in error recovery. */
                        yychar = Lexer::YYUNDEF;
                        yytoken = SymbolKind::get(2);
                        yyerrloc = yylloc;
                        label = Self::YYERRLAB1;
                    } else {
                        // If the proper action on seeing token YYTOKEN is to reduce or to
                        // detect an error, take that action.
                        yyn += yytoken.code();
                        if yyn < 0 || Self::YYLAST_ < yyn || Self::yycheck_[i32_to_usize(yyn)] != yytoken.code() {
                            label = Self::YYDEFAULT;
                        }

                        /* <= 0 means reduce or error.  */
                        else {
                            yyn = Self::yytable_[i32_to_usize(yyn)];
                            if yyn <= 0 {
                                if yy_table_value_is_error(yyn) {
                                    label = Self::YYERRLAB;
                                } else {
                                    yyn = -yyn;
                                    label = Self::YYREDUCE;
                                }
                            } else {
                                /* Shift the lookahead token.  */
                                self.yy_symbol_print("Shifting", yytoken, &yylval, &yylloc);

                                /* Discard the token being shifted.  */
                                yychar = Self::YYEMPTY_;

                                /* Count tokens shifted since error; after three, turn off error status.  */
                                if self.yyerrstatus_ > 0 {
                                    self.yyerrstatus_ -= 1;
                                }

                                yystate = yyn;
                                yystack.push(yystate, std::mem::take(&mut yylval), std::mem::take(&mut yylloc));
                                label = Self::YYNEWSTATE;
                            }
                        }
                    }
                    continue;
                }, // YYNEWSTATE

                // yydefault -- do the default action for the current state.
                Self::YYDEFAULT => {
                    yyn = Self::yydefact_[i32_to_usize(yystate)];
                    if yyn == 0 {
                        label = Self::YYERRLAB;
                    } else {
                        label = Self::YYREDUCE;
                    }
                    continue;
                } // YYDEFAULT

                // yyreduce -- Do a reduction.
                Self::YYREDUCE => {
                    yylen = i32_to_usize(Self::yyr2_[i32_to_usize(yyn)]);
                    label = match self.yyaction(yyn, &mut yystack, &mut yylen) {
                        Ok(label) => label,
                        Err(_) => Self::YYERROR
                    };
                    yystate = yystack.state_at(0);
                    continue;
                }, // YYREDUCE

                // yyerrlab -- here on detecting error
                Self::YYERRLAB => {
                    /* If not already recovering from an error, report this error.  */
                    if self.yyerrstatus_ == 0 {
                        self.yynerrs += 1;
                        if yychar == Self::YYEMPTY_ {
                            yytoken = &DYMMY_SYMBOL_KIND;
                        }
                        self.report_syntax_error(&yystack, yytoken, yylloc);
                    }
                    yyerrloc = yylloc;
                    if self.yyerrstatus_ == 3 {
                        // If just tried and failed to reuse lookahead token after an error, discard it.

                        if yychar <= Lexer::YYEOF {
                            /* Return failure if at end of input.  */
                            if yychar == Lexer::YYEOF {
                                return false;
                            }
                        }
                        else {
                            yychar = Self::YYEMPTY_;
                        }
                    }

                    // Else will try to reuse lookahead token after shifting the error token.
                    label = Self::YYERRLAB1;
                    continue;
                }, // YYERRLAB

                // errorlab -- error raised explicitly by YYERROR.
                Self::YYERROR => {
                    /* Do not reclaim the symbols of the rule which action triggered
                    this YYERROR.  */
                    yystack.pop_n(yylen);
                    yylen = 0;
                    yystate = yystack.state_at(0);
                    label = Self::YYERRLAB1;
                    continue;
                }, // YYERROR

                // yyerrlab1 -- common code for both syntax error and YYERROR.
                Self::YYERRLAB1 => {
                    self.yyerrstatus_ = 3;       /* Each real token shifted decrements this.  */

                    // Pop stack until we find a state that shifts the error token.
                    loop {
                        yyn = Self::yypact_[i32_to_usize(yystate)];
                        if !yy_pact_value_is_default(yyn) {
                            yyn += SymbolKind { value: SymbolKind::S_YYerror }.code();
                            if (0..=Self::YYLAST_).contains(&yyn) {
                                let yyn_usize = i32_to_usize(yyn);
                                if Self::yycheck_[yyn_usize] == SymbolKind::S_YYerror {
                                    yyn = Self::yytable_[yyn_usize];
                                    if 0 < yyn {
                                        break;
                                    }
                                }
                            }
                        }

                        // Pop the current state because it cannot handle the error token.
                        if yystack.len() == 1 {
                            return false;
                        }

                        yyerrloc = *yystack.location_at(0);
                        yystack.pop();
                        yystate = yystack.state_at(0);
                        if  self.debug  {
                            eprintln!("{}", yystack);
                        }
                    }

                    if label == Self::YYABORT {
                        /* Leave the switch.  */
                        continue;
                    }

                    /* Muck with the stack to setup for yylloc.  */
                    yystack.push(0, YYValue::Uninitialized, yylloc);
                    yystack.push(0, YYValue::Uninitialized, yyerrloc);
                    yyloc = make_yylloc(&yystack, 2);
                    yystack.pop_n(2);

                    /* Shift the error token.  */
                    self.yy_symbol_print("Shifting", SymbolKind::get(Self::yystos_[i32_to_usize(yyn)]), &yylval, &yyloc);

                    yystate = yyn;
                    yystack.push(yyn, yylval.clone(), yyloc);
                    label = Self::YYNEWSTATE;
                    continue;
                }, // YYERRLAB1

                // Accept
                Self::YYACCEPT => {
                    return true;
                }

                // Abort.
                Self::YYABORT => {
                    return false;
                },

                _ => {
                    panic!("internal bison error: unknown label {}", label);
                }
            }
        }
    }
}

// Whether the given `yypact_` value indicates a defaulted state.
fn yy_pact_value_is_default(yyvalue: i32) -> bool {
    yyvalue == YYPACT_NINF_
}

// Whether the given `yytable_`
// value indicates a syntax error.
// yyvalue: the value to check
fn yy_table_value_is_error(yyvalue: i32) -> bool {
    yyvalue == YYTABLE_NINF_
}

const YYPACT_NINF_: i32 = -31;
const YYTABLE_NINF_: i32 = -1;

impl Parser {

/* YYPACT[STATE-NUM] -- Index in YYTABLE of the portion describing
   STATE-NUM.  */
  #[allow(non_upper_case_globals)]
const yypact_: &'static [i32] = &[    -20,   -31,   -31,   -31,     6,   -31,   -20,   -31,   -30,   -31,
     -31,   -31,     7,   -20,   -31,    -4,   -31,   -30,    -9,   -20,
     -31,   -17,   -31,     9,   -31,   -31,    -9,   -31,    -5,   -31,
      12,   -31,   -31,   -17,   -31,   -31 ];

/* YYDEFACT[STATE-NUM] -- Default reduction number in state STATE-NUM.
   Performed when YYTABLE does not specify something else to do.  Zero
   means the default is an error.  */
  #[allow(non_upper_case_globals)]
const yydefact_: &'static [i32] = &[      0,    12,    13,    15,     0,     4,     2,     3,     0,    14,
       1,     5,     0,     6,     8,     0,     9,     0,     0,     0,
      11,     6,    18,     0,     7,    10,     0,    19,     0,    20,
       0,    22,    16,     0,    17,    21 ];

/* YYPGOTO[NTERM-NUM].  */
  #[allow(non_upper_case_globals)]
const yypgoto_: &'static [i32] = &[    -31,   -31,    11,   -31,     0,   -31,   -31,     3,    -6,    -7,
      -3,   -31,    -8 ];

/* YYDEFGOTO[NTERM-NUM].  */
  #[allow(non_upper_case_globals)]
const yydefgoto_: &'static [i32] = &[      0,     4,     5,     6,    14,     7,    15,    16,     8,     9,
      24,    28,    29 ];

/* YYTABLE[YYPACT[STATE-NUM]] -- What to do in state STATE-NUM.  If
   positive, shift that token.  If negative, reduce the rule whose
   number is the opposite.  If YYTABLE_NINF, syntax error.  */
  #[allow(non_upper_case_globals)]
const yytable_: &'static [i32] = &[     21,    12,     1,     2,    18,    32,    10,    17,     3,    33,
      20,    23,    19,    17,    13,    26,    30,    11,     3,    23,
      34,    27,    25,    31,     0,    35,     0,     0,     0,     3,
      22 ];

#[allow(non_upper_case_globals)]
const yycheck_: &'static [i32] = &[      9,     8,    22,    23,     8,    10,     0,    13,    38,    14,
      17,    18,    16,    19,     7,    32,     7,     6,    38,    26,
       8,    21,    19,    26,    -1,    33,    -1,    -1,    -1,    38,
      39 ];

/* YYSTOS[STATE-NUM] -- The symbol kind of the accessing symbol of
   state STATE-NUM.  */
  #[allow(non_upper_case_globals)]
const yystos_: &'static [i32] = &[      0,    22,    23,    38,    45,    46,    47,    49,    52,    53,
       0,    46,    53,     7,    48,    50,    51,    52,     8,    16,
      53,     9,    39,    53,    54,    51,    32,    48,    55,    56,
       7,    54,    10,    14,     8,    56 ];

/* YYR1[RULE-NUM] -- Symbol kind of the left-hand side of rule RULE-NUM.  */
  #[allow(non_upper_case_globals)]
const yyr1_: &'static [i32] = &[      0,    44,    45,    46,    47,    47,    48,    49,    50,    50,
      50,    51,    52,    52,    52,    53,    54,    54,    54,    55,
      55,    55,    56 ];

/* YYR2[RULE-NUM] -- Number of symbols on the right-hand side of rule RULE-NUM.  */
  #[allow(non_upper_case_globals)]
const yyr2_: &'static [i32] = &[      0,     2,     1,     1,     1,     2,     0,     6,     1,     1,
       3,     2,     1,     1,     1,     1,     3,     3,     1,     1,
       1,     3,     2 ];


/* YYRLINE[YYN] -- Source line where rule number YYN was defined.  */
  #[allow(non_upper_case_globals)]
const yyrline_: &'static [i32] = &[      0,    83,    83,    90,    94,    96,   101,   104,   112,   114,
     116,   122,   125,   127,   129,   132,   139,   141,   144,   148,
     150,   152,   157 ];


  // Report on the debug stream that the rule yyrule is going to be reduced.
  fn yy_reduce_print(&self, yyrule: i32, yystack: &YYStack) {
        if !( self.debug ) {
            return;
        }

        let yylno = Self::yyrline_[i32_to_usize(yyrule)];
        let yynrhs = Self::yyr2_[i32_to_usize(yyrule)];
        // Print the symbols being reduced, and their result.
        self.yycdebug(&format!("Reducing stack by rule {} (line {}):", /* " fix */ yyrule - 1, yylno));

        // The symbols being reduced.
        for yyi in 0..yynrhs {
            let state: usize = i32_to_usize(yystack.state_at(i32_to_usize(yynrhs - (yyi + 1))));
            self.yy_symbol_print(
                &format!("   ${} =", yyi + 1),
                SymbolKind::get(Self::yystos_[state]),
                yystack.borrow_value_at(i32_to_usize(yynrhs - (yyi + 1))),
                yystack.location_at(i32_to_usize(yynrhs - (yyi + 1)))
            );
        }
  }

  /* YYTRANSLATE_(TOKEN-NUM) -- Symbol number corresponding to TOKEN-NUM
     as returned by yylex, with out-of-bounds checking.  */
  fn yytranslate_(t: i32) -> &'static SymbolKind
  {
        // Last valid token kind.
        let code_max: i32 = 298;
        if t <= 0 {
            SymbolKind::get(0)
        } else if t <= code_max {
            let t = i32_to_usize(t);
            SymbolKind::get(Self::yytranslate_table_[t])
        } else {
            SymbolKind::get(2)
        }
  }
  #[allow(non_upper_case_globals)]
const yytranslate_table_: &'static [i32] = &[      0,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     2,     2,     2,     2,
       2,     2,     2,     2,     2,     2,     1,     2,     3,     4,
       5,     6,     7,     8,     9,    10,    11,    12,    13,    14,
      15,    16,    17,    18,    19,    20,    21,    22,    23,    24,
      25,    26,    27,    28,    29,    30,    31,    32,    33,    34,
      35,    36,    37,    38,    39,    40,    41,    42,    43 ];


const YYLAST_: i32 = 30;
const YYEMPTY_: i32 = -2;
const YYFINAL_: i32 = 10;
const YYNTOKENS_: i32 = 44;


}

/* Unqualified %code blocks.  */
/* "src/parser.y":33  */

    // code
    

/* "src/parser.rs":1395  */


/* "src/parser.y":160  */


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
