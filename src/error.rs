use crate::{
    lexer::Lexer,
    loc::Loc,
    parser::{token_name, Parser, SymbolKind, YYStack},
};
impl Parser {
    pub fn report_syntax_error(&self, stack: &YYStack, yytoken: &SymbolKind, loc: Loc) {
        // for i in &stack.stack {
            println!("An error has occured during parsing.");
        for i in 0..stack.len() {
            println!("stack[{}] = {:#?}", i, stack.stack.get(i).unwrap());
        }
        macro_rules! expect {
            ($index: expr, $token: expr, $b: block) => {
                if stack.stack.get($index).is_some()
                    && stack.stack.get($index).unwrap().value.to_token().token_type == $token
                {
                    $b
                } else {
                    println!(
                        "Expected {:#?}, found {}",
                        token_name($token),
                        yytoken.name()
                    );
                }
            };
        }
        // if stack.stack.get(1).unwrap().value.to_token().token_type == Lexer::tFN {
        //     expect!(2, Lexer::tIDENTIFIER, {
        //         match stack.stack.get(3).unwrap().value.to_token().token_type {
        //             Lexer::tLPAREN => {
        //                 expect!(4, Lexer::tRPAREN, {
        //                     expect!(5, Lexer::tLBRACK, { expect!(6, Lexer::tRBRACK, {}) })
        //                 });
        //             }
        //             Lexer::tLBRACK => {
        //                 expect!(4, Lexer::tRBRACK, {});
        //             }
        //             _ => panic!(),
        //         }
        //     });
        // }
        // if stack.stack.get(1).unwrap().value.to_token().token_type == Lexer::tFN {
        //     expect!(2, Lexer::tIDENTIFIER, {
        //         expect!(3, Lexer::tLPAREN, {
        //             expect!(4, Lexer::tRPAREN, {
        //                 expect!(5, Lexer::tLBRACK, {
        //                     expect!(6, Lexer::tRBRACK, {
        //                         println!("This case is valid.");
        //                     });
        //                 });
        //             });
        //         });
        //     });
        // }
    }
}
