#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use lexer::Lexer;
mod lexer {
    use crate::{loc::Loc, value::Value};
    use peekmore::{PeekMore, PeekMoreIterator};
    pub struct Lexer {
        chars: PeekMoreIterator<std::vec::IntoIter<char>>,
        pub spaces: String,
        pub col: usize,
        pub line: usize,
        pub is_comment: bool,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Lexer {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Lexer {
                    chars: ref __self_0_0,
                    spaces: ref __self_0_1,
                    col: ref __self_0_2,
                    line: ref __self_0_3,
                    is_comment: ref __self_0_4,
                } => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Lexer");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "chars",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "spaces",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "col",
                        &&(*__self_0_2),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "line",
                        &&(*__self_0_3),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "is_comment",
                        &&(*__self_0_4),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl Lexer {
        pub fn new(input: &str) -> Self {
            Lexer {
                chars: input.chars().collect::<Vec<_>>().into_iter().peekmore(),
                col: 0,
                line: 0,
                spaces: String::new(),
                is_comment: false,
            }
        }
        pub fn yylex(&mut self) -> Token {
            self.next().unwrap()
        }
        fn bracket_to_token(c: char) -> Option<i32> {
            match c {
                '(' => Some(Lexer::tLPAREN),
                ')' => Some(Lexer::tRPAREN),
                '{' => Some(Lexer::tLBRACK),
                '}' => Some(Lexer::tRBRACK),
                _ => None,
            }
        }
    }
    static TOKS: &[(char, i32)] = &[
        (':', Lexer::tCOLON),
        (',', Lexer::tCOMMA),
        ('.', Lexer::tPERIOD),
        ('=', Lexer::tASSIGN),
        ('+', Lexer::tPLUS),
        ('-', Lexer::tMINUS),
        ('*', Lexer::tMUL),
        ('/', Lexer::tDIV),
        ('_', Lexer::tINFER),
        (';', Lexer::tSEMICOLON),
        ('&', Lexer::tAMPERSAND),
        ('>', Lexer::tGT),
        ('<', Lexer::tLT),
    ];
    impl Iterator for Lexer {
        type Item = Token;
        #[allow(unused_macros)]
        fn next(&mut self) -> Option<Self::Item> {
            loop {
                let m = match self.chars.next() {
                    Some('#') => {
                        self.is_comment = true;
                        continue;
                    }
                    Some(n @ '\n') => {
                        self.spaces.push(n);
                        self.col = 0;
                        self.line += 1;
                        self.is_comment = false;
                        continue;
                    }
                    Some(_) if self.is_comment => continue,
                    Some(c) if Self::bracket_to_token(c).is_some() => Some(Token {
                        token_type: Self::bracket_to_token(c).unwrap(),
                        token_value: c.to_string(),
                        spaces_before: std::mem::take(&mut self.spaces),
                        loc: Loc {
                            begin: self.col,
                            end: {
                                self.col += 1;
                                self.col
                            },
                        },
                    }),
                    Some(':') if self.chars.peek_nth(0) == Some(&':') => {
                        self.chars.next().unwrap();
                        Some(Token {
                            loc: Loc {
                                begin: self.col,
                                end: {
                                    self.col += 2;
                                    self.col
                                },
                            },
                            token_type: Self::tPATHSEP,
                            token_value: "::".to_string(),
                            spaces_before: std::mem::take(&mut self.spaces),
                        })
                    }
                    Some('"') => {
                        let mut tokens = ::alloc::vec::Vec::new();
                        let mut current = 0;
                        while let Some(value) = self.chars.peek_nth(current) {
                            if *value == '\"' {
                                break;
                            }
                            if *value == '\\' {
                                let c = { self.chars.peek_nth(current + 1).unwrap() };
                                let escaped = match c {
                                    'n' => '\n',
                                    't' => '\t',
                                    '\\' => '\\',
                                    '"' => '"',
                                    'r' => '\r',
                                    ' ' => ::core::panicking::panic_fmt(
                                        ::core::fmt::Arguments::new_v1(
                                            &["space"],
                                            &match () {
                                                _args => [],
                                            },
                                        ),
                                    ),
                                    _ => ::core::panicking::panic_fmt(
                                        ::core::fmt::Arguments::new_v1_formatted(
                                            &["Invalid escaped character "],
                                            &match (&c.escape_unicode(),) {
                                                _args => [::core::fmt::ArgumentV1::new(
                                                    _args.0,
                                                    ::core::fmt::Debug::fmt,
                                                )],
                                            },
                                            &[::core::fmt::rt::v1::Argument {
                                                position: 0usize,
                                                format: ::core::fmt::rt::v1::FormatSpec {
                                                    fill: ' ',
                                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                                    flags: 4u32,
                                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                                    width: ::core::fmt::rt::v1::Count::Implied,
                                                },
                                            }],
                                            unsafe { ::core::fmt::UnsafeArg::new() },
                                        ),
                                    ),
                                };
                                current += 1;
                                tokens.push(escaped);
                            } else {
                                tokens.push(*value);
                            }
                            current += 1;
                        }
                        for _ in 0..current + 1 {
                            self.chars.next();
                        }
                        let token_value = tokens.iter().fold(String::new(), |mut s, c| {
                            s.push(*c);
                            s
                        });
                        Some(Token {
                            loc: Loc {
                                begin: self.col,
                                end: {
                                    self.col += tokens.len();
                                    self.col
                                },
                            },
                            token_type: Self::tSTRING,
                            token_value,
                            spaces_before: std::mem::take(&mut self.spaces),
                        })
                    }
                    Some(c) if c.is_alphabetic() => {
                        let mut tokens = <[_]>::into_vec(box [c]);
                        let mut current = 0;
                        while let Some(value) = self.chars.peek_nth(current) {
                            if !char::is_alphanumeric(*value) {
                                break;
                            }
                            tokens.push(*value);
                            current += 1;
                        }
                        for _ in 0..current {
                            self.chars.next();
                        }
                        let token_value = tokens.iter().fold(String::new(), |mut s, c| {
                            s.push(*c);
                            s
                        });
                        let token_type = match token_value.as_str() {
                            "fn" => Self::tFN,
                            "let" => Self::tLET,
                            "return" => Self::kwRETURN,
                            "extern" => Self::kwEXTERN,
                            "struct" => Self::kwSTRUCT,
                            "import" => Self::kwIMPORT,
                            "new" => Self::kwNEW,
                            "i32" => Self::tI32,
                            "i64" => Self::tI64,
                            "true" => Self::tBOOL,
                            "false" => Self::tBOOL,
                            "bool" => Self::kwBOOL,
                            "u32" => Self::tU32,
                            "u64" => Self::tU64,
                            "i8" => Self::tI8,
                            "if" => Self::tIF,
                            "else" => Self::tELSE,
                            _ => Self::tIDENTIFIER,
                        };
                        Some(Token {
                            loc: Loc {
                                begin: self.col,
                                end: {
                                    self.col += tokens.len();
                                    self.col
                                },
                            },
                            token_type,
                            token_value,
                            spaces_before: std::mem::take(&mut self.spaces),
                        })
                    }
                    Some(c) if c.is_numeric() => {
                        let mut tokens = <[_]>::into_vec(box [c]);
                        let mut current = 0;
                        while let Some(value) = self.chars.peek_nth(current) {
                            let valid = char::is_numeric(*value) || c == '_';
                            if !valid {
                                break;
                            }
                            tokens.push(*value);
                            current += 1;
                        }
                        for _ in 0..current {
                            self.chars.next();
                        }
                        let token_value = tokens.iter().fold(String::new(), |mut s, c| {
                            s.push(*c);
                            s
                        });
                        Some(Token {
                            loc: Loc {
                                begin: self.col,
                                end: {
                                    self.col += tokens.len();
                                    self.col
                                },
                            },
                            token_type: Self::tNUM,
                            token_value,
                            spaces_before: std::mem::take(&mut self.spaces),
                        })
                    }
                    Some(c) if TOKS.iter().map(|t| t.0).collect::<Vec<_>>().contains(&c) => {
                        let (_c, ty) = *TOKS.iter().find(|t| t.0 == c).unwrap();
                        Some(Token {
                            loc: Loc {
                                begin: self.col,
                                end: {
                                    self.col += 1;
                                    self.col
                                },
                            },
                            token_type: ty,
                            token_value: c.to_string(),
                            spaces_before: std::mem::take(&mut self.spaces),
                        })
                    }
                    Some(s @ ' ') => {
                        self.spaces.push(s);
                        self.col += 1;
                        continue;
                    }
                    None => Some(Token {
                        token_type: Self::YYEOF,
                        token_value: "".to_string(),
                        spaces_before: std::mem::take(&mut self.spaces),
                        loc: Loc {
                            begin: self.col,
                            end: {
                                self.col += 1;
                                self.col
                            },
                        },
                    }),
                    _ => continue,
                };
                return m;
            }
        }
    }
    pub struct Token {
        pub token_type: i32,
        pub token_value: String,
        pub loc: Loc,
        pub spaces_before: String,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Token {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Token {
                    token_type: ref __self_0_0,
                    token_value: ref __self_0_1,
                    loc: ref __self_0_2,
                    spaces_before: ref __self_0_3,
                } => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_struct(f, "Token");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "token_type",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "token_value",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "loc",
                        &&(*__self_0_2),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "spaces_before",
                        &&(*__self_0_3),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for Token {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Token {
        #[inline]
        fn eq(&self, other: &Token) -> bool {
            match *other {
                Token {
                    token_type: ref __self_1_0,
                    token_value: ref __self_1_1,
                    loc: ref __self_1_2,
                    spaces_before: ref __self_1_3,
                } => match *self {
                    Token {
                        token_type: ref __self_0_0,
                        token_value: ref __self_0_1,
                        loc: ref __self_0_2,
                        spaces_before: ref __self_0_3,
                    } => {
                        (*__self_0_0) == (*__self_1_0)
                            && (*__self_0_1) == (*__self_1_1)
                            && (*__self_0_2) == (*__self_1_2)
                            && (*__self_0_3) == (*__self_1_3)
                    }
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Token) -> bool {
            match *other {
                Token {
                    token_type: ref __self_1_0,
                    token_value: ref __self_1_1,
                    loc: ref __self_1_2,
                    spaces_before: ref __self_1_3,
                } => match *self {
                    Token {
                        token_type: ref __self_0_0,
                        token_value: ref __self_0_1,
                        loc: ref __self_0_2,
                        spaces_before: ref __self_0_3,
                    } => {
                        (*__self_0_0) != (*__self_1_0)
                            || (*__self_0_1) != (*__self_1_1)
                            || (*__self_0_2) != (*__self_1_2)
                            || (*__self_0_3) != (*__self_1_3)
                    }
                },
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Token {
        #[inline]
        fn clone(&self) -> Token {
            match *self {
                Token {
                    token_type: ref __self_0_0,
                    token_value: ref __self_0_1,
                    loc: ref __self_0_2,
                    spaces_before: ref __self_0_3,
                } => Token {
                    token_type: ::core::clone::Clone::clone(&(*__self_0_0)),
                    token_value: ::core::clone::Clone::clone(&(*__self_0_1)),
                    loc: ::core::clone::Clone::clone(&(*__self_0_2)),
                    spaces_before: ::core::clone::Clone::clone(&(*__self_0_3)),
                },
            }
        }
    }
    impl Token {
        pub fn from(v: Value) -> Token {
            match v {
                Value::Token(t) => t,
                _ => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["wrong"],
                    &match () {
                        _args => [],
                    },
                )),
            }
        }
    }
}
mod loc {
    ///
    #[repr(C)]
    pub struct Loc {
        /// Begin of the `Loc` range
        pub begin: usize,
        /// End of the `Loc` range
        pub end: usize,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Loc {
        #[inline]
        fn clone(&self) -> Loc {
            {
                let _: ::core::clone::AssertParamIsClone<usize>;
                let _: ::core::clone::AssertParamIsClone<usize>;
                *self
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for Loc {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for Loc {
        #[inline]
        fn eq(&self, other: &Loc) -> bool {
            match *other {
                Loc {
                    begin: ref __self_1_0,
                    end: ref __self_1_1,
                } => match *self {
                    Loc {
                        begin: ref __self_0_0,
                        end: ref __self_0_1,
                    } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &Loc) -> bool {
            match *other {
                Loc {
                    begin: ref __self_1_0,
                    end: ref __self_1_1,
                } => match *self {
                    Loc {
                        begin: ref __self_0_0,
                        end: ref __self_0_1,
                    } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                },
            }
        }
    }
    impl ::core::marker::StructuralEq for Loc {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::Eq for Loc {
        #[inline]
        #[doc(hidden)]
        #[no_coverage]
        fn assert_receiver_is_total_eq(&self) -> () {
            {
                let _: ::core::cmp::AssertParamIsEq<usize>;
                let _: ::core::cmp::AssertParamIsEq<usize>;
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::default::Default for Loc {
        #[inline]
        fn default() -> Loc {
            Loc {
                begin: ::core::default::Default::default(),
                end: ::core::default::Default::default(),
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::marker::Copy for Loc {}
    impl Loc {
        /// Converts location to a range
        pub fn to_range(&self) -> std::ops::Range<usize> {
            self.begin..self.end
        }
    }
    impl std::fmt::Debug for Loc {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            f.write_str(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["", "..."],
                    &match (&self.begin, &self.end) {
                        _args => [
                            ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Display::fmt),
                        ],
                    },
                ));
                res
            })
        }
    }
}
mod error {
    use crate::{
        lexer::Lexer,
        loc::Loc,
        parser::{token_name, Parser, SymbolKind, YYStack},
    };
    impl Parser {
        pub fn report_syntax_error(&self, stack: &YYStack, yytoken: &SymbolKind, loc: Loc) {
            {
                ::std::io::_print(::core::fmt::Arguments::new_v1(
                    &["An error has occured during parsing.\n"],
                    &match () {
                        _args => [],
                    },
                ));
            };
            for i in 0..stack.len() {
                {
                    ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                        &["stack[", "] = ", "\n"],
                        &match (&i, &stack.stack.get(i).unwrap()) {
                            _args => [
                                ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Debug::fmt),
                            ],
                        },
                        &[
                            ::core::fmt::rt::v1::Argument {
                                position: 0usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 0u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Implied,
                                },
                            },
                            ::core::fmt::rt::v1::Argument {
                                position: 1usize,
                                format: ::core::fmt::rt::v1::FormatSpec {
                                    fill: ' ',
                                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                                    flags: 4u32,
                                    precision: ::core::fmt::rt::v1::Count::Implied,
                                    width: ::core::fmt::rt::v1::Count::Implied,
                                },
                            },
                        ],
                        unsafe { ::core::fmt::UnsafeArg::new() },
                    ));
                };
            }
            if stack.stack.get(1).unwrap().value.to_token().token_type == Lexer::tFN {
                if stack.stack.get(2).is_some()
                    && stack.stack.get(2).unwrap().value.to_token().token_type == Lexer::tIDENTIFIER
                {
                    {
                        match stack.stack.get(3).unwrap().value.to_token().token_type {
                            Lexer::tLPAREN => {
                                if stack.stack.get(4).is_some()
                                    && stack.stack.get(4).unwrap().value.to_token().token_type
                                        == Lexer::tRPAREN
                                {
                                    {
                                        if stack.stack.get(5).is_some()
                                            && stack
                                                .stack
                                                .get(5)
                                                .unwrap()
                                                .value
                                                .to_token()
                                                .token_type
                                                == Lexer::tLBRACK
                                        {
                                            {
                                                if stack.stack.get(6).is_some()
                                                    && stack
                                                        .stack
                                                        .get(6)
                                                        .unwrap()
                                                        .value
                                                        .to_token()
                                                        .token_type
                                                        == Lexer::tRBRACK
                                                {
                                                    {}
                                                } else {
                                                    {
                                                        :: std :: io :: _print (:: core :: fmt :: Arguments :: new_v1_formatted (& ["Expected " , ", found " , "\n"] , & match (& token_name (Lexer :: tRBRACK) , & yytoken . name ()) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Debug :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , } , & [:: core :: fmt :: rt :: v1 :: Argument { position : 0usize , format : :: core :: fmt :: rt :: v1 :: FormatSpec { fill : ' ' , align : :: core :: fmt :: rt :: v1 :: Alignment :: Unknown , flags : 4u32 , precision : :: core :: fmt :: rt :: v1 :: Count :: Implied , width : :: core :: fmt :: rt :: v1 :: Count :: Implied , } , } , :: core :: fmt :: rt :: v1 :: Argument { position : 1usize , format : :: core :: fmt :: rt :: v1 :: FormatSpec { fill : ' ' , align : :: core :: fmt :: rt :: v1 :: Alignment :: Unknown , flags : 0u32 , precision : :: core :: fmt :: rt :: v1 :: Count :: Implied , width : :: core :: fmt :: rt :: v1 :: Count :: Implied , } , }] , unsafe { :: core :: fmt :: UnsafeArg :: new () })) ;
                                                    };
                                                }
                                            }
                                        } else {
                                            {
                                                :: std :: io :: _print (:: core :: fmt :: Arguments :: new_v1_formatted (& ["Expected " , ", found " , "\n"] , & match (& token_name (Lexer :: tLBRACK) , & yytoken . name ()) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Debug :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , } , & [:: core :: fmt :: rt :: v1 :: Argument { position : 0usize , format : :: core :: fmt :: rt :: v1 :: FormatSpec { fill : ' ' , align : :: core :: fmt :: rt :: v1 :: Alignment :: Unknown , flags : 4u32 , precision : :: core :: fmt :: rt :: v1 :: Count :: Implied , width : :: core :: fmt :: rt :: v1 :: Count :: Implied , } , } , :: core :: fmt :: rt :: v1 :: Argument { position : 1usize , format : :: core :: fmt :: rt :: v1 :: FormatSpec { fill : ' ' , align : :: core :: fmt :: rt :: v1 :: Alignment :: Unknown , flags : 0u32 , precision : :: core :: fmt :: rt :: v1 :: Count :: Implied , width : :: core :: fmt :: rt :: v1 :: Count :: Implied , } , }] , unsafe { :: core :: fmt :: UnsafeArg :: new () })) ;
                                            };
                                        }
                                    }
                                } else {
                                    {
                                        :: std :: io :: _print (:: core :: fmt :: Arguments :: new_v1_formatted (& ["Expected " , ", found " , "\n"] , & match (& token_name (Lexer :: tRPAREN) , & yytoken . name ()) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Debug :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , } , & [:: core :: fmt :: rt :: v1 :: Argument { position : 0usize , format : :: core :: fmt :: rt :: v1 :: FormatSpec { fill : ' ' , align : :: core :: fmt :: rt :: v1 :: Alignment :: Unknown , flags : 4u32 , precision : :: core :: fmt :: rt :: v1 :: Count :: Implied , width : :: core :: fmt :: rt :: v1 :: Count :: Implied , } , } , :: core :: fmt :: rt :: v1 :: Argument { position : 1usize , format : :: core :: fmt :: rt :: v1 :: FormatSpec { fill : ' ' , align : :: core :: fmt :: rt :: v1 :: Alignment :: Unknown , flags : 0u32 , precision : :: core :: fmt :: rt :: v1 :: Count :: Implied , width : :: core :: fmt :: rt :: v1 :: Count :: Implied , } , }] , unsafe { :: core :: fmt :: UnsafeArg :: new () })) ;
                                    };
                                };
                            }
                            Lexer::tLBRACK => {
                                if stack.stack.get(4).is_some()
                                    && stack.stack.get(4).unwrap().value.to_token().token_type
                                        == Lexer::tRBRACK
                                {
                                    {}
                                } else {
                                    {
                                        :: std :: io :: _print (:: core :: fmt :: Arguments :: new_v1_formatted (& ["Expected " , ", found " , "\n"] , & match (& token_name (Lexer :: tRBRACK) , & yytoken . name ()) { _args => [:: core :: fmt :: ArgumentV1 :: new (_args . 0 , :: core :: fmt :: Debug :: fmt) , :: core :: fmt :: ArgumentV1 :: new (_args . 1 , :: core :: fmt :: Display :: fmt)] , } , & [:: core :: fmt :: rt :: v1 :: Argument { position : 0usize , format : :: core :: fmt :: rt :: v1 :: FormatSpec { fill : ' ' , align : :: core :: fmt :: rt :: v1 :: Alignment :: Unknown , flags : 4u32 , precision : :: core :: fmt :: rt :: v1 :: Count :: Implied , width : :: core :: fmt :: rt :: v1 :: Count :: Implied , } , } , :: core :: fmt :: rt :: v1 :: Argument { position : 1usize , format : :: core :: fmt :: rt :: v1 :: FormatSpec { fill : ' ' , align : :: core :: fmt :: rt :: v1 :: Alignment :: Unknown , flags : 0u32 , precision : :: core :: fmt :: rt :: v1 :: Count :: Implied , width : :: core :: fmt :: rt :: v1 :: Count :: Implied , } , }] , unsafe { :: core :: fmt :: UnsafeArg :: new () })) ;
                                    };
                                };
                            }
                            _ => ::core::panicking::panic("explicit panic"),
                        }
                    }
                } else {
                    {
                        ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
                            &["Expected ", ", found ", "\n"],
                            &match (&token_name(Lexer::tIDENTIFIER), &yytoken.name()) {
                                _args => [
                                    ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Debug::fmt),
                                    ::core::fmt::ArgumentV1::new(
                                        _args.1,
                                        ::core::fmt::Display::fmt,
                                    ),
                                ],
                            },
                            &[
                                ::core::fmt::rt::v1::Argument {
                                    position: 0usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 4u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                                ::core::fmt::rt::v1::Argument {
                                    position: 1usize,
                                    format: ::core::fmt::rt::v1::FormatSpec {
                                        fill: ' ',
                                        align: ::core::fmt::rt::v1::Alignment::Unknown,
                                        flags: 0u32,
                                        precision: ::core::fmt::rt::v1::Count::Implied,
                                        width: ::core::fmt::rt::v1::Count::Implied,
                                    },
                                },
                            ],
                            unsafe { ::core::fmt::UnsafeArg::new() },
                        ));
                    };
                };
            }
        }
    }
}
mod parser {
    use std::convert::TryInto;
    use crate::lexer::Lexer;
    use crate::lexer::Token;
    use crate::loc::Loc;
    use crate::value::*;
    use std::path::PathBuf;
    /// A Bison parser, automatically generated from src/parser.y.
    pub struct Parser {
        /// Lexer that is used to get tokens
        pub yylexer: Lexer,
        #[allow(dead_code)]
        yy_error_verbose: bool,
        yynerrs: i32,
        yyerrstatus_: i32,
        result: Option<i32>,
        /// Just an extra field for demonstration
        pub name: String,
        /// Enables debug printing
        pub debug: bool,
        pub program: Value,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Parser {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Parser {
                    yylexer: ref __self_0_0,
                    yy_error_verbose: ref __self_0_1,
                    yynerrs: ref __self_0_2,
                    yyerrstatus_: ref __self_0_3,
                    result: ref __self_0_4,
                    name: ref __self_0_5,
                    debug: ref __self_0_6,
                    program: ref __self_0_7,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Parser");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "yylexer",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "yy_error_verbose",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "yynerrs",
                        &&(*__self_0_2),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "yyerrstatus_",
                        &&(*__self_0_3),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "result",
                        &&(*__self_0_4),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "name",
                        &&(*__self_0_5),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "debug",
                        &&(*__self_0_6),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "program",
                        &&(*__self_0_7),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[inline]
    fn i32_to_usize(v: i32) -> usize {
        v as usize
    }
    /// Maps token ID into human-readable name
    pub fn token_name(id: i32) -> &'static str {
        let first_token = Lexer::YYerror;
        if id > first_token + 1 {
            let pos: usize = (id - first_token + 1)
                .try_into()
                .expect("failed to cast token id into usize, is it negative?");
            Lexer::TOKEN_NAMES[pos]
        } else if id == 0 {
            "EOF"
        } else {
            ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                &["token_name fails, ", " (first token = ", ")"],
                &match (&id, &first_token) {
                    _args => [
                        ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Display::fmt),
                    ],
                },
            ))
        }
    }
    /// Local alias
    type YYLoc = Loc;
    impl Parser {
        #[allow(dead_code)]
        const BISON_VERSION: &'static str = "30802";
    }
    fn make_yylloc(rhs: &YYStack, n: usize) -> YYLoc {
        if 0 < n {
            YYLoc {
                begin: rhs.location_at(n - 1).begin,
                end: rhs.location_at(0).end,
            }
        } else {
            YYLoc {
                begin: rhs.location_at(0).end,
                end: rhs.location_at(0).end,
            }
        }
    }
    pub struct SymbolKind {
        value: i32,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for SymbolKind {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                SymbolKind {
                    value: ref __self_0_0,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "SymbolKind");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "value",
                        &&(*__self_0_0),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for SymbolKind {
        #[inline]
        fn clone(&self) -> SymbolKind {
            match *self {
                SymbolKind {
                    value: ref __self_0_0,
                } => SymbolKind {
                    value: ::core::clone::Clone::clone(&(*__self_0_0)),
                },
            }
        }
    }
    impl ::core::marker::StructuralPartialEq for SymbolKind {}
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::cmp::PartialEq for SymbolKind {
        #[inline]
        fn eq(&self, other: &SymbolKind) -> bool {
            match *other {
                SymbolKind {
                    value: ref __self_1_0,
                } => match *self {
                    SymbolKind {
                        value: ref __self_0_0,
                    } => (*__self_0_0) == (*__self_1_0),
                },
            }
        }
        #[inline]
        fn ne(&self, other: &SymbolKind) -> bool {
            match *other {
                SymbolKind {
                    value: ref __self_1_0,
                } => match *self {
                    SymbolKind {
                        value: ref __self_0_0,
                    } => (*__self_0_0) != (*__self_1_0),
                },
            }
        }
    }
    impl SymbolKind {
        #[allow(non_upper_case_globals)]
        const S_YYEOF: i32 = 0;
        #[allow(non_upper_case_globals)]
        const S_YYerror: i32 = 1;
        #[allow(non_upper_case_globals)]
        const S_YYUNDEF: i32 = 2;
        #[allow(non_upper_case_globals)]
        const S_tPLUS: i32 = 3;
        #[allow(non_upper_case_globals)]
        const S_tMINUS: i32 = 4;
        #[allow(non_upper_case_globals)]
        const S_tMUL: i32 = 5;
        #[allow(non_upper_case_globals)]
        const S_tDIV: i32 = 6;
        #[allow(non_upper_case_globals)]
        const S_tLPAREN: i32 = 7;
        #[allow(non_upper_case_globals)]
        const S_tRPAREN: i32 = 8;
        #[allow(non_upper_case_globals)]
        const S_tLBRACK: i32 = 9;
        #[allow(non_upper_case_globals)]
        const S_tRBRACK: i32 = 10;
        #[allow(non_upper_case_globals)]
        const S_tCOLON: i32 = 11;
        #[allow(non_upper_case_globals)]
        const S_tPERIOD: i32 = 12;
        #[allow(non_upper_case_globals)]
        const S_tAMPERSAND: i32 = 13;
        #[allow(non_upper_case_globals)]
        const S_tSEMICOLON: i32 = 14;
        #[allow(non_upper_case_globals)]
        const S_tPATHSEP: i32 = 15;
        #[allow(non_upper_case_globals)]
        const S_tCOMMA: i32 = 16;
        #[allow(non_upper_case_globals)]
        const S_tASSIGN: i32 = 17;
        #[allow(non_upper_case_globals)]
        const S_tGT: i32 = 18;
        #[allow(non_upper_case_globals)]
        const S_tLT: i32 = 19;
        #[allow(non_upper_case_globals)]
        const S_tLET: i32 = 20;
        #[allow(non_upper_case_globals)]
        const S_tSTR: i32 = 21;
        #[allow(non_upper_case_globals)]
        const S_tI32: i32 = 22;
        #[allow(non_upper_case_globals)]
        const S_tI64: i32 = 23;
        #[allow(non_upper_case_globals)]
        const S_tBOOL: i32 = 24;
        #[allow(non_upper_case_globals)]
        const S_kwBOOL: i32 = 25;
        #[allow(non_upper_case_globals)]
        const S_tU32: i32 = 26;
        #[allow(non_upper_case_globals)]
        const S_tU64: i32 = 27;
        #[allow(non_upper_case_globals)]
        const S_tI8: i32 = 28;
        #[allow(non_upper_case_globals)]
        const S_tIF: i32 = 29;
        #[allow(non_upper_case_globals)]
        const S_tELSE: i32 = 30;
        #[allow(non_upper_case_globals)]
        const S_tSTRING: i32 = 31;
        #[allow(non_upper_case_globals)]
        const S_kwRETURN: i32 = 32;
        #[allow(non_upper_case_globals)]
        const S_kwSTRUCT: i32 = 33;
        #[allow(non_upper_case_globals)]
        const S_kwIMPORT: i32 = 34;
        #[allow(non_upper_case_globals)]
        const S_kwNEW: i32 = 35;
        #[allow(non_upper_case_globals)]
        const S_kwEXTERN: i32 = 36;
        #[allow(non_upper_case_globals)]
        const S_tINFER: i32 = 37;
        #[allow(non_upper_case_globals)]
        const S_tIDENTIFIER: i32 = 38;
        #[allow(non_upper_case_globals)]
        const S_tNUM: i32 = 39;
        #[allow(non_upper_case_globals)]
        const S_tFN: i32 = 40;
        #[allow(non_upper_case_globals)]
        const S_tERROR: i32 = 41;
        #[allow(non_upper_case_globals)]
        const S_tABORT: i32 = 42;
        #[allow(non_upper_case_globals)]
        const S_tACCEPT: i32 = 43;
        #[allow(non_upper_case_globals)]
        const S_YYACCEPT: i32 = 44;
        #[allow(non_upper_case_globals)]
        const S_program: i32 = 45;
        #[allow(non_upper_case_globals)]
        const S_item: i32 = 46;
        #[allow(non_upper_case_globals)]
        const S_items: i32 = 47;
        #[allow(non_upper_case_globals)]
        const S_function: i32 = 48;
        #[allow(non_upper_case_globals)]
        const S_ty_idents: i32 = 49;
        #[allow(non_upper_case_globals)]
        const S_ty_ident: i32 = 50;
        #[allow(non_upper_case_globals)]
        const S_ty: i32 = 51;
        #[allow(non_upper_case_globals)]
        const S_ident: i32 = 52;
        const VALUES_: &'static [SymbolKind] = &[
            SymbolKind {
                value: SymbolKind::S_YYEOF,
            },
            SymbolKind {
                value: SymbolKind::S_YYerror,
            },
            SymbolKind {
                value: SymbolKind::S_YYUNDEF,
            },
            SymbolKind {
                value: SymbolKind::S_tPLUS,
            },
            SymbolKind {
                value: SymbolKind::S_tMINUS,
            },
            SymbolKind {
                value: SymbolKind::S_tMUL,
            },
            SymbolKind {
                value: SymbolKind::S_tDIV,
            },
            SymbolKind {
                value: SymbolKind::S_tLPAREN,
            },
            SymbolKind {
                value: SymbolKind::S_tRPAREN,
            },
            SymbolKind {
                value: SymbolKind::S_tLBRACK,
            },
            SymbolKind {
                value: SymbolKind::S_tRBRACK,
            },
            SymbolKind {
                value: SymbolKind::S_tCOLON,
            },
            SymbolKind {
                value: SymbolKind::S_tPERIOD,
            },
            SymbolKind {
                value: SymbolKind::S_tAMPERSAND,
            },
            SymbolKind {
                value: SymbolKind::S_tSEMICOLON,
            },
            SymbolKind {
                value: SymbolKind::S_tPATHSEP,
            },
            SymbolKind {
                value: SymbolKind::S_tCOMMA,
            },
            SymbolKind {
                value: SymbolKind::S_tASSIGN,
            },
            SymbolKind {
                value: SymbolKind::S_tGT,
            },
            SymbolKind {
                value: SymbolKind::S_tLT,
            },
            SymbolKind {
                value: SymbolKind::S_tLET,
            },
            SymbolKind {
                value: SymbolKind::S_tSTR,
            },
            SymbolKind {
                value: SymbolKind::S_tI32,
            },
            SymbolKind {
                value: SymbolKind::S_tI64,
            },
            SymbolKind {
                value: SymbolKind::S_tBOOL,
            },
            SymbolKind {
                value: SymbolKind::S_kwBOOL,
            },
            SymbolKind {
                value: SymbolKind::S_tU32,
            },
            SymbolKind {
                value: SymbolKind::S_tU64,
            },
            SymbolKind {
                value: SymbolKind::S_tI8,
            },
            SymbolKind {
                value: SymbolKind::S_tIF,
            },
            SymbolKind {
                value: SymbolKind::S_tELSE,
            },
            SymbolKind {
                value: SymbolKind::S_tSTRING,
            },
            SymbolKind {
                value: SymbolKind::S_kwRETURN,
            },
            SymbolKind {
                value: SymbolKind::S_kwSTRUCT,
            },
            SymbolKind {
                value: SymbolKind::S_kwIMPORT,
            },
            SymbolKind {
                value: SymbolKind::S_kwNEW,
            },
            SymbolKind {
                value: SymbolKind::S_kwEXTERN,
            },
            SymbolKind {
                value: SymbolKind::S_tINFER,
            },
            SymbolKind {
                value: SymbolKind::S_tIDENTIFIER,
            },
            SymbolKind {
                value: SymbolKind::S_tNUM,
            },
            SymbolKind {
                value: SymbolKind::S_tFN,
            },
            SymbolKind {
                value: SymbolKind::S_tERROR,
            },
            SymbolKind {
                value: SymbolKind::S_tABORT,
            },
            SymbolKind {
                value: SymbolKind::S_tACCEPT,
            },
            SymbolKind {
                value: SymbolKind::S_YYACCEPT,
            },
            SymbolKind {
                value: SymbolKind::S_program,
            },
            SymbolKind {
                value: SymbolKind::S_item,
            },
            SymbolKind {
                value: SymbolKind::S_items,
            },
            SymbolKind {
                value: SymbolKind::S_function,
            },
            SymbolKind {
                value: SymbolKind::S_ty_idents,
            },
            SymbolKind {
                value: SymbolKind::S_ty_ident,
            },
            SymbolKind {
                value: SymbolKind::S_ty,
            },
            SymbolKind {
                value: SymbolKind::S_ident,
            },
        ];
        pub(crate) fn get(n: i32) -> &'static SymbolKind {
            &Self::VALUES_[i32_to_usize(n)]
        }
        pub(crate) fn code(&self) -> i32 {
            self.value
        }
        #[allow(non_upper_case_globals)]
        const yynames_: &'static [&'static str] = &[
            "end of file",
            "error",
            "invalid token",
            "+",
            "-",
            "*",
            "/",
            "(",
            ")",
            "{",
            "}",
            ":",
            ".",
            "&",
            ";",
            "::",
            ",",
            "=",
            ">",
            "<",
            "let",
            "str",
            "i32",
            "i64",
            "true or fals",
            "bool",
            "u32",
            "u64",
            "i8",
            "if",
            "else",
            "Text wrapped in quotes",
            "return",
            "struct",
            "import",
            "new",
            "extern",
            "_",
            "local variable or method",
            "number",
            "fn",
            "controlled YYERROR",
            "controlled YYABORT",
            "controlled YYACCEPT",
            "$accept",
            "program",
            "item",
            "items",
            "function",
            "ty_idents",
            "ty_ident",
            "ty",
            "ident",
            "<<NULL>>",
        ];
        pub(crate) fn name(&self) -> String {
            let code: usize = self.code().try_into().unwrap();
            Self::yynames_[code].to_owned()
        }
    }
    const DYMMY_SYMBOL_KIND: SymbolKind = SymbolKind { value: 0 };
    impl Lexer {
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
        #[allow(dead_code)]
        const EOF: i32 = Self::YYEOF;
        #[allow(dead_code)]
        pub(crate) const TOKEN_NAMES: &'static [&'static str] = &[
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
        ];
    }
    impl Parser {
        fn yycdebug(&self, s: &str) {
            if self.debug {
                {
                    ::std::io::_eprint(::core::fmt::Arguments::new_v1(
                        &["", "\n"],
                        &match (&s,) {
                            _args => [::core::fmt::ArgumentV1::new(
                                _args.0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ));
                };
            }
        }
    }
    /// Local alias
    type YYValue = Value;
    pub struct YYStackItem {
        pub state: i32,
        pub value: YYValue,
        pub loc: YYLoc,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for YYStackItem {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                YYStackItem {
                    state: ref __self_0_0,
                    value: ref __self_0_1,
                    loc: ref __self_0_2,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "YYStackItem");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "state",
                        &&(*__self_0_0),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "value",
                        &&(*__self_0_1),
                    );
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "loc",
                        &&(*__self_0_2),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    pub struct YYStack {
        pub stack: Vec<YYStackItem>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for YYStack {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                YYStack {
                    stack: ref __self_0_0,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "YYStack");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "stack",
                        &&(*__self_0_0),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
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
            let states = self
                .stack
                .iter()
                .map(|e| e.state.to_string())
                .collect::<Vec<String>>()
                .join(" ");
            let values = self
                .stack
                .iter()
                .map(|e| {
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &[""],
                        &match (&e.value,) {
                            _args => [::core::fmt::ArgumentV1::new(
                                _args.0,
                                ::core::fmt::Debug::fmt,
                            )],
                        },
                    ));
                    res
                })
                .collect::<Vec<String>>()
                .join(" ");
            f.write_fmt(::core::fmt::Arguments::new_v1(
                &["Stack now states = ", " / values = ", " "],
                &match (&states, &values) {
                    _args => [
                        ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Display::fmt),
                        ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Debug::fmt),
                    ],
                },
            ))
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
        fn yyaction(
            &mut self,
            yyn: i32,
            yystack: &mut YYStack,
            yylen: &mut usize,
        ) -> Result<i32, ()> {
            #[allow(unused_assignments)]
            let mut yyval: YYValue = YYValue::Uninitialized;
            let yyloc: YYLoc = make_yylloc(yystack, *yylen);
            self.yy_reduce_print(yyn, yystack);
            match yyn {
                2 => {
                    self.result = Some(1);
                    let m = ValueList::from(yystack.owned_value_at(0))
                        .iter()
                        .map(|v| Item::from(v.to_owned()))
                        .collect::<Vec<_>>();
                    let prog = Value::Program(m);
                    self.program = prog.clone();
                    yyval = prog;
                }
                3 => yyval = Value::Item(Item::Function(Function::from(yystack.owned_value_at(0)))),
                4 => yyval = Value::ValueList(<[_]>::into_vec(box [yystack.owned_value_at(0)])),
                5 => {
                    let mut v = ValueList::from(yystack.owned_value_at(1));
                    v.push(yystack.owned_value_at(0));
                    yyval = Value::ValueList(v)
                }
                6 => {
                    yyval = Value::Function(Function {
                        ty: Ty::from(yystack.owned_value_at(6)),
                        name: Identifier::from(yystack.owned_value_at(5)),
                        params: ValueList::from(yystack.owned_value_at(3))
                            .iter()
                            .map(|v| Value::TypedIdent, ::from(v.to_owned()))
                            .collect::<Vec<Value::TypedIdent>>(),
                    })
                }
                7 => yyval = Value::ValueList(<[_]>::into_vec(box [yystack.owned_value_at(0)])),
                8 => {
                    let mut v = ValueList::from(yystack.owned_value_at(2));
                    v.push(yystack.owned_value_at(0));
                    yyval = Value::ValueList(v)
                }
                9 => {
                    yyval = Value::TypedIdent(TypedIdent(
                        Ty::from(yystack.owned_value_at(1)),
                        Identifier::from(yystack.owned_value_at(0)),
                    ))
                }
                10 => yyval = Value::Ty(Ty::I32),
                11 => yyval = Value::Ty(Ty::I64),
                12 => {
                    yyval = Value::Ty(Ty::Identifier(Identifier::from(yystack.owned_value_at(0))))
                }
                13 => {
                    yyval = Value::Identifier(Identifier(
                        Token::from(yystack.owned_value_at(0)).token_value,
                    ))
                }
                _ => {}
            }
            if let YYValue::Uninitialized = yyval {
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["yyval is Uninitialized in rule at line "],
                    &match (&Self::yyrline_[i32_to_usize(yyn)],) {
                        _args => [::core::fmt::ArgumentV1::new(
                            _args.0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ));
            }
            self.yy_symbol_print(
                "-> $$ =",
                SymbolKind::get(Self::yyr1_[i32_to_usize(yyn)]),
                &yyval,
                &yyloc,
            );
            yystack.pop_n(*yylen);
            *yylen = 0;
            let yystate =
                self.yy_lr_goto_state(yystack.state_at(0), Self::yyr1_[i32_to_usize(yyn)]);
            yystack.push(yystate, yyval, yyloc);
            Ok(Self::YYNEWSTATE)
        }
        fn yy_symbol_print(
            &self,
            s: &str,
            yykind: &SymbolKind,
            yyvalue: &YYValue,
            yylocation: &YYLoc,
        ) {
            if self.debug {
                self.yycdebug(&{
                    let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                        &["", "", " ", " ( ", ": ", " )"],
                        &match (
                            &s,
                            &if yykind.code() < Self::YYNTOKENS_ {
                                " token "
                            } else {
                                " nterm "
                            },
                            &yykind.name(),
                            &yylocation,
                            &yyvalue,
                        ) {
                            _args => [
                                ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Display::fmt),
                                ::core::fmt::ArgumentV1::new(_args.2, ::core::fmt::Debug::fmt),
                                ::core::fmt::ArgumentV1::new(_args.3, ::core::fmt::Debug::fmt),
                                ::core::fmt::ArgumentV1::new(_args.4, ::core::fmt::Debug::fmt),
                            ],
                        },
                    ));
                    res
                })
            }
        }
        /// Parses given input. Returns true if the parsing was successful.
        pub fn parse(&mut self) -> bool {
            let mut yyloc: YYLoc;
            let mut yychar: i32 = Self::YYEMPTY_;
            let mut yytoken = &DYMMY_SYMBOL_KIND;
            let mut yyn: i32 = 0;
            let mut yylen: usize = 0;
            let mut yystate: i32 = 0;
            let mut yystack = YYStack::new();
            let mut label: i32 = Self::YYNEWSTATE;
            let mut yyerrloc: YYLoc = YYLoc { begin: 0, end: 0 };
            let mut yylloc: YYLoc = YYLoc { begin: 0, end: 0 };
            let mut yylval: YYValue = YYValue::Uninitialized;
            self.yycdebug("Starting parse");
            self.yyerrstatus_ = 0;
            self.yynerrs = 0;
            yystack.push(yystate, yylval.clone(), yylloc);
            loop {
                match label {
                    Self::YYNEWSTATE => {
                        if self.debug {
                            self.yycdebug(&{
                                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                                    &["Entering state "],
                                    &match (&yystate,) {
                                        _args => [::core::fmt::ArgumentV1::new(
                                            _args.0,
                                            ::core::fmt::Display::fmt,
                                        )],
                                    },
                                ));
                                res
                            });
                            {
                                ::std::io::_eprint(::core::fmt::Arguments::new_v1(
                                    &["", "\n"],
                                    &match (&yystack,) {
                                        _args => [::core::fmt::ArgumentV1::new(
                                            _args.0,
                                            ::core::fmt::Display::fmt,
                                        )],
                                    },
                                ));
                            };
                        }
                        if yystate == Self::YYFINAL_ {
                            return true;
                        }
                        yyn = Self::yypact_[i32_to_usize(yystate)];
                        if yy_pact_value_is_default(yyn) {
                            label = Self::YYDEFAULT;
                            continue;
                        }
                        if yychar == Self::YYEMPTY_ {
                            self.yycdebug("Reading a token");
                            let token = self.next_token();
                            yychar = token.token_type;
                            yylloc = token.loc;
                            yylval = YYValue::from_token(token);
                        }
                        yytoken = Self::yytranslate_(yychar);
                        self.yy_symbol_print("Next token is", yytoken, &yylval, &yylloc);
                        if yytoken == SymbolKind::get(1) {
                            yychar = Lexer::YYUNDEF;
                            yytoken = SymbolKind::get(2);
                            yyerrloc = yylloc;
                            label = Self::YYERRLAB1;
                        } else {
                            yyn += yytoken.code();
                            if yyn < 0
                                || Self::YYLAST_ < yyn
                                || Self::yycheck_[i32_to_usize(yyn)] != yytoken.code()
                            {
                                label = Self::YYDEFAULT;
                            } else {
                                yyn = Self::yytable_[i32_to_usize(yyn)];
                                if yyn <= 0 {
                                    if yy_table_value_is_error(yyn) {
                                        label = Self::YYERRLAB;
                                    } else {
                                        yyn = -yyn;
                                        label = Self::YYREDUCE;
                                    }
                                } else {
                                    self.yy_symbol_print("Shifting", yytoken, &yylval, &yylloc);
                                    yychar = Self::YYEMPTY_;
                                    if self.yyerrstatus_ > 0 {
                                        self.yyerrstatus_ -= 1;
                                    }
                                    yystate = yyn;
                                    yystack.push(
                                        yystate,
                                        std::mem::take(&mut yylval),
                                        std::mem::take(&mut yylloc),
                                    );
                                    label = Self::YYNEWSTATE;
                                }
                            }
                        }
                        continue;
                    }
                    Self::YYDEFAULT => {
                        yyn = Self::yydefact_[i32_to_usize(yystate)];
                        if yyn == 0 {
                            label = Self::YYERRLAB;
                        } else {
                            label = Self::YYREDUCE;
                        }
                        continue;
                    }
                    Self::YYREDUCE => {
                        yylen = i32_to_usize(Self::yyr2_[i32_to_usize(yyn)]);
                        label = match self.yyaction(yyn, &mut yystack, &mut yylen) {
                            Ok(label) => label,
                            Err(_) => Self::YYERROR,
                        };
                        yystate = yystack.state_at(0);
                        continue;
                    }
                    Self::YYERRLAB => {
                        if self.yyerrstatus_ == 0 {
                            self.yynerrs += 1;
                            if yychar == Self::YYEMPTY_ {
                                yytoken = &DYMMY_SYMBOL_KIND;
                            }
                            self.report_syntax_error(&yystack, yytoken, yylloc);
                        }
                        yyerrloc = yylloc;
                        if self.yyerrstatus_ == 3 {
                            if yychar <= Lexer::YYEOF {
                                if yychar == Lexer::YYEOF {
                                    return false;
                                }
                            } else {
                                yychar = Self::YYEMPTY_;
                            }
                        }
                        label = Self::YYERRLAB1;
                        continue;
                    }
                    Self::YYERROR => {
                        yystack.pop_n(yylen);
                        yylen = 0;
                        yystate = yystack.state_at(0);
                        label = Self::YYERRLAB1;
                        continue;
                    }
                    Self::YYERRLAB1 => {
                        self.yyerrstatus_ = 3;
                        loop {
                            yyn = Self::yypact_[i32_to_usize(yystate)];
                            if !yy_pact_value_is_default(yyn) {
                                yyn += SymbolKind {
                                    value: SymbolKind::S_YYerror,
                                }
                                .code();
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
                            if yystack.len() == 1 {
                                return false;
                            }
                            yyerrloc = *yystack.location_at(0);
                            yystack.pop();
                            yystate = yystack.state_at(0);
                            if self.debug {
                                {
                                    ::std::io::_eprint(::core::fmt::Arguments::new_v1(
                                        &["", "\n"],
                                        &match (&yystack,) {
                                            _args => [::core::fmt::ArgumentV1::new(
                                                _args.0,
                                                ::core::fmt::Display::fmt,
                                            )],
                                        },
                                    ));
                                };
                            }
                        }
                        if label == Self::YYABORT {
                            continue;
                        }
                        yystack.push(0, YYValue::Uninitialized, yylloc);
                        yystack.push(0, YYValue::Uninitialized, yyerrloc);
                        yyloc = make_yylloc(&yystack, 2);
                        yystack.pop_n(2);
                        self.yy_symbol_print(
                            "Shifting",
                            SymbolKind::get(Self::yystos_[i32_to_usize(yyn)]),
                            &yylval,
                            &yyloc,
                        );
                        yystate = yyn;
                        yystack.push(yyn, yylval.clone(), yyloc);
                        label = Self::YYNEWSTATE;
                        continue;
                    }
                    Self::YYACCEPT => {
                        return true;
                    }
                    Self::YYABORT => {
                        return false;
                    }
                    _ => {
                        ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                            &["internal bison error: unknown label "],
                            &match (&label,) {
                                _args => [::core::fmt::ArgumentV1::new(
                                    _args.0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                    }
                }
            }
        }
    }
    fn yy_pact_value_is_default(yyvalue: i32) -> bool {
        yyvalue == YYPACT_NINF_
    }
    fn yy_table_value_is_error(yyvalue: i32) -> bool {
        yyvalue == YYTABLE_NINF_
    }
    const YYPACT_NINF_: i32 = -33;
    const YYTABLE_NINF_: i32 = -1;
    impl Parser {
        #[allow(non_upper_case_globals)]
        const yypact_: &'static [i32] = &[
            -22, -33, -33, -33, 5, -33, -22, -33, -32, -33, -33, -33, 0, -22, -6, -33, -32, -1,
            -22, -33, 2, -33, -33,
        ];
        #[allow(non_upper_case_globals)]
        const yydefact_: &'static [i32] = &[
            0, 10, 11, 13, 0, 4, 2, 3, 0, 12, 1, 5, 0, 0, 0, 7, 0, 0, 0, 9, 0, 8, 6,
        ];
        #[allow(non_upper_case_globals)]
        const yypgoto_: &'static [i32] = &[-33, -33, 7, -33, -33, -33, -4, -9, -5];
        #[allow(non_upper_case_globals)]
        const yydefgoto_: &'static [i32] = &[0, 4, 5, 6, 7, 14, 15, 8, 9];
        #[allow(non_upper_case_globals)]
        const yytable_: &'static [i32] = &[
            1, 2, 17, 12, 16, 10, 3, 13, 20, 16, 18, 19, 22, 11, 21, 0, 3,
        ];
        #[allow(non_upper_case_globals)]
        const yycheck_: &'static [i32] =
            &[22, 23, 8, 8, 13, 0, 38, 7, 9, 18, 16, 16, 10, 6, 18, -1, 38];
        #[allow(non_upper_case_globals)]
        const yystos_: &'static [i32] = &[
            0, 22, 23, 38, 45, 46, 47, 48, 51, 52, 0, 46, 52, 7, 49, 50, 51, 8, 16, 52, 9, 50, 10,
        ];
        #[allow(non_upper_case_globals)]
        const yyr1_: &'static [i32] = &[0, 44, 45, 46, 47, 47, 48, 49, 49, 50, 51, 51, 51, 52];
        #[allow(non_upper_case_globals)]
        const yyr2_: &'static [i32] = &[0, 2, 1, 1, 1, 2, 7, 1, 3, 2, 1, 1, 1, 1];
        #[allow(non_upper_case_globals)]
        const yyrline_: &'static [i32] = &[
            0, 82, 82, 89, 93, 95, 101, 108, 110, 116, 119, 121, 123, 126,
        ];
        fn yy_reduce_print(&self, yyrule: i32, yystack: &YYStack) {
            if !(self.debug) {
                return;
            }
            let yylno = Self::yyrline_[i32_to_usize(yyrule)];
            let yynrhs = Self::yyr2_[i32_to_usize(yyrule)];
            self.yycdebug(&{
                let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                    &["Reducing stack by rule ", " (line ", "):"],
                    &match (&(yyrule - 1), &yylno) {
                        _args => [
                            ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Display::fmt),
                            ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Display::fmt),
                        ],
                    },
                ));
                res
            });
            for yyi in 0..yynrhs {
                let state: usize = i32_to_usize(yystack.state_at(i32_to_usize(yynrhs - (yyi + 1))));
                self.yy_symbol_print(
                    &{
                        let res = ::alloc::fmt::format(::core::fmt::Arguments::new_v1(
                            &["   $", " ="],
                            &match (&(yyi + 1),) {
                                _args => [::core::fmt::ArgumentV1::new(
                                    _args.0,
                                    ::core::fmt::Display::fmt,
                                )],
                            },
                        ));
                        res
                    },
                    SymbolKind::get(Self::yystos_[state]),
                    yystack.borrow_value_at(i32_to_usize(yynrhs - (yyi + 1))),
                    yystack.location_at(i32_to_usize(yynrhs - (yyi + 1))),
                );
            }
        }
        fn yytranslate_(t: i32) -> &'static SymbolKind {
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
        const yytranslate_table_: &'static [i32] = &[
            0, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 1, 2, 3, 4, 5,
            6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28,
            29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43,
        ];
        const YYLAST_: i32 = 16;
        const YYEMPTY_: i32 = -2;
        const YYFINAL_: i32 = 10;
        const YYNTOKENS_: i32 = 44;
    }
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
                program: Value::Uninitialized,
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
}
mod value {
    pub(crate) use crate::ast::*;
    use crate::lexer::Token;
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
        Program(Vec<Item>),
        ValueList(Vec<Value>),
        TypedIdent(TypedIdent),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Value {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Value::Stolen,) => ::core::fmt::Formatter::write_str(f, "Stolen"),
                (&Value::Uninitialized,) => ::core::fmt::Formatter::write_str(f, "Uninitialized"),
                (&Value::None,) => ::core::fmt::Formatter::write_str(f, "None"),
                (&Value::Token(ref __self_0),) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Token");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Value::Result(ref __self_0),) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Result");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Value::Ty(ref __self_0),) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Ty");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Value::Item(ref __self_0),) => {
                    let debug_trait_builder = &mut ::core::fmt::Formatter::debug_tuple(f, "Item");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Value::Function(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Function");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Value::Identifier(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Identifier");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Value::Program(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Program");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Value::ValueList(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "ValueList");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
                (&Value::TypedIdent(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "TypedIdent");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Value {
        #[inline]
        fn clone(&self) -> Value {
            match (&*self,) {
                (&Value::Stolen,) => Value::Stolen,
                (&Value::Uninitialized,) => Value::Uninitialized,
                (&Value::None,) => Value::None,
                (&Value::Token(ref __self_0),) => {
                    Value::Token(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&Value::Result(ref __self_0),) => {
                    Value::Result(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&Value::Ty(ref __self_0),) => Value::Ty(::core::clone::Clone::clone(&(*__self_0))),
                (&Value::Item(ref __self_0),) => {
                    Value::Item(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&Value::Function(ref __self_0),) => {
                    Value::Function(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&Value::Identifier(ref __self_0),) => {
                    Value::Identifier(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&Value::Program(ref __self_0),) => {
                    Value::Program(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&Value::ValueList(ref __self_0),) => {
                    Value::ValueList(::core::clone::Clone::clone(&(*__self_0)))
                }
                (&Value::TypedIdent(ref __self_0),) => {
                    Value::TypedIdent(::core::clone::Clone::clone(&(*__self_0)))
                }
            }
        }
    }
    #[allow(non_snake_case)]
    impl Identifier {
        pub(crate) fn from(value: Value) -> Identifier {
            let vc = value.clone();
            match value {
                Value::Identifier(s) => (|s: Identifier| s)(s),
                other => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["wrong type, expected ", ", got "],
                    &match (&vc, &other) {
                        _args => [
                            ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Debug::fmt),
                            ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Debug::fmt),
                        ],
                    },
                )),
            }
        }
    }
    #[allow(non_snake_case)]
    impl Ty {
        pub(crate) fn from(value: Value) -> Ty {
            let vc = value.clone();
            match value {
                Value::Ty(s) => (|s: Ty| s)(s),
                other => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["wrong type, expected ", ", got "],
                    &match (&vc, &other) {
                        _args => [
                            ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Debug::fmt),
                            ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Debug::fmt),
                        ],
                    },
                )),
            }
        }
    }
    #[allow(non_snake_case)]
    impl Item {
        pub(crate) fn from(value: Value) -> Item {
            let vc = value.clone();
            match value {
                Value::Item(s) => (|s: Item| s)(s),
                other => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["wrong type, expected ", ", got "],
                    &match (&vc, &other) {
                        _args => [
                            ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Debug::fmt),
                            ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Debug::fmt),
                        ],
                    },
                )),
            }
        }
    }
    #[allow(non_snake_case)]
    impl Function {
        pub(crate) fn from(value: Value) -> Function {
            let vc = value.clone();
            match value {
                Value::Function(s) => (|s: Function| s)(s),
                other => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["wrong type, expected ", ", got "],
                    &match (&vc, &other) {
                        _args => [
                            ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Debug::fmt),
                            ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Debug::fmt),
                        ],
                    },
                )),
            }
        }
    }
    #[allow(non_snake_case)]
    impl TypedIdent {
        pub(crate) fn from(value: Value) -> TypedIdent {
            let vc = value.clone();
            match value {
                Value::TypedIdent(s) => (|s: TypedIdent| s)(s),
                other => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["wrong type, expected ", ", got "],
                    &match (&vc, &other) {
                        _args => [
                            ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Debug::fmt),
                            ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Debug::fmt),
                        ],
                    },
                )),
            }
        }
    }
    #[allow(non_snake_case)]
    pub mod ValueList {
        use super::Value;
        pub(crate) fn from(value: Value) -> Vec<Value> {
            let vc = value.clone();
            match value {
                Value::ValueList(a) => (|s: Vec<Value>| s)(a),
                other => ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(
                    &["wrong type, expected ", ", got "],
                    &match (&vc, &other) {
                        _args => [
                            ::core::fmt::ArgumentV1::new(_args.0, ::core::fmt::Debug::fmt),
                            ::core::fmt::ArgumentV1::new(_args.1, ::core::fmt::Debug::fmt),
                        ],
                    },
                )),
            }
        }
    }
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
                ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1_formatted(
                    &["called `to_token` on a non-token value. "],
                    &match (&self,) {
                        _args => [::core::fmt::ArgumentV1::new(
                            _args.0,
                            ::core::fmt::Debug::fmt,
                        )],
                    },
                    &[::core::fmt::rt::v1::Argument {
                        position: 0usize,
                        format: ::core::fmt::rt::v1::FormatSpec {
                            fill: ' ',
                            align: ::core::fmt::rt::v1::Alignment::Unknown,
                            flags: 4u32,
                            precision: ::core::fmt::rt::v1::Count::Implied,
                            width: ::core::fmt::rt::v1::Count::Implied,
                        },
                    }],
                    unsafe { ::core::fmt::UnsafeArg::new() },
                ));
            }
        }
    }
}
mod ast {
    pub enum Ty {
        I32,
        I64,
        Identifier(Identifier),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Ty {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Ty::I32,) => ::core::fmt::Formatter::write_str(f, "I32"),
                (&Ty::I64,) => ::core::fmt::Formatter::write_str(f, "I64"),
                (&Ty::Identifier(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Identifier");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Ty {
        #[inline]
        fn clone(&self) -> Ty {
            match (&*self,) {
                (&Ty::I32,) => Ty::I32,
                (&Ty::I64,) => Ty::I64,
                (&Ty::Identifier(ref __self_0),) => {
                    Ty::Identifier(::core::clone::Clone::clone(&(*__self_0)))
                }
            }
        }
    }
    pub enum Item {
        Function(Function),
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Item {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match (&*self,) {
                (&Item::Function(ref __self_0),) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Function");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Item {
        #[inline]
        fn clone(&self) -> Item {
            match (&*self,) {
                (&Item::Function(ref __self_0),) => {
                    Item::Function(::core::clone::Clone::clone(&(*__self_0)))
                }
            }
        }
    }
    pub struct Function {
        pub name: Identifier,
        pub ty: Ty,
        pub params: Vec<TypedIdent>,
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Function {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Function {
                    name: ref __self_0_0,
                    ty: ref __self_0_1,
                    params: ref __self_0_2,
                } => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_struct(f, "Function");
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "name",
                        &&(*__self_0_0),
                    );
                    let _ =
                        ::core::fmt::DebugStruct::field(debug_trait_builder, "ty", &&(*__self_0_1));
                    let _ = ::core::fmt::DebugStruct::field(
                        debug_trait_builder,
                        "params",
                        &&(*__self_0_2),
                    );
                    ::core::fmt::DebugStruct::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Function {
        #[inline]
        fn clone(&self) -> Function {
            match *self {
                Function {
                    name: ref __self_0_0,
                    ty: ref __self_0_1,
                    params: ref __self_0_2,
                } => Function {
                    name: ::core::clone::Clone::clone(&(*__self_0_0)),
                    ty: ::core::clone::Clone::clone(&(*__self_0_1)),
                    params: ::core::clone::Clone::clone(&(*__self_0_2)),
                },
            }
        }
    }
    pub struct Identifier(pub String);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for Identifier {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                Identifier(ref __self_0_0) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "Identifier");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for Identifier {
        #[inline]
        fn clone(&self) -> Identifier {
            match *self {
                Identifier(ref __self_0_0) => {
                    Identifier(::core::clone::Clone::clone(&(*__self_0_0)))
                }
            }
        }
    }
    pub struct TypedIdent(pub Ty, pub Identifier);
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::fmt::Debug for TypedIdent {
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            match *self {
                TypedIdent(ref __self_0_0, ref __self_0_1) => {
                    let debug_trait_builder =
                        &mut ::core::fmt::Formatter::debug_tuple(f, "TypedIdent");
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_0));
                    let _ = ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0_1));
                    ::core::fmt::DebugTuple::finish(debug_trait_builder)
                }
            }
        }
    }
    #[automatically_derived]
    #[allow(unused_qualifications)]
    impl ::core::clone::Clone for TypedIdent {
        #[inline]
        fn clone(&self) -> TypedIdent {
            match *self {
                TypedIdent(ref __self_0_0, ref __self_0_1) => TypedIdent(
                    ::core::clone::Clone::clone(&(*__self_0_0)),
                    ::core::clone::Clone::clone(&(*__self_0_1)),
                ),
            }
        }
    }
}
fn main() {
    let lexer = Lexer::new("i32 main(){} i32 main(){}i32 main(){}");
    let mut parser = parser::Parser::new(lexer, ".parser");
    let parsed = parser.parse();
    {
        ::std::io::_print(::core::fmt::Arguments::new_v1_formatted(
            &["", "\n"],
            &match (&parser.program,) {
                _args => [::core::fmt::ArgumentV1::new(
                    _args.0,
                    ::core::fmt::Debug::fmt,
                )],
            },
            &[::core::fmt::rt::v1::Argument {
                position: 0usize,
                format: ::core::fmt::rt::v1::FormatSpec {
                    fill: ' ',
                    align: ::core::fmt::rt::v1::Alignment::Unknown,
                    flags: 4u32,
                    precision: ::core::fmt::rt::v1::Count::Implied,
                    width: ::core::fmt::rt::v1::Count::Implied,
                },
            }],
            unsafe { ::core::fmt::UnsafeArg::new() },
        ));
    };
}
