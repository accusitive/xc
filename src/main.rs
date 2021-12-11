use std::{collections::HashMap, time::Duration};

use ast::Function;
use inkwell::{
    builder::Builder,
    context::Context,
    execution_engine::{ExecutionEngine, JitFunction},
    module::{Linkage, Module},
    targets::{InitializationConfig, Target},
    types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum},
    values::{BasicMetadataValueEnum, BasicValueEnum, CallableValue, FunctionValue, IntValue},
};
use lexer::Lexer;
use value::{Expr, Identifier, Item, Program, Statement, Ty, Value};

mod ast;
mod error;
mod lexer;
mod loc;
mod parser;
mod value;
fn parse() -> Option<Value> {
    let s = std::fs::read_to_string("./x.txt").unwrap();
    let lexer = Lexer::new(&s);
    let mut parser = parser::Parser::new(lexer, ".parser");
    let parsed = parser.parse();
    return if parsed { Some(parser.program) } else { None };
    // let program = parser.program;
    // return program;
}
struct Compiler<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
    builder: Builder<'ctx>,
    engine: ExecutionEngine<'ctx>,
    functions: HashMap<String, FunctionValue<'ctx>>,
    kek_input: Option<String>,
    kek_result: Option<FunctionValue<'ctx>>,
}
impl<'ctx> Compiler<'ctx> {
    fn compile_expression(&mut self, e: &Expr) -> Option<BasicValueEnum<'ctx>> {
        match e {
            Expr::Block(stmts) => {
                let values = stmts
                    .into_iter()
                    .map(|stmt| self.compile_statement(stmt))
                    .collect::<Vec<_>>();
                *values.last().unwrap_or(&None)
            }
            Expr::Literal(i) => Some(BasicValueEnum::IntValue(
                self.context.i64_type().const_int(*i, false),
            )),
            Expr::FunctionCall(i) => {
                let f = match self.functions.get(&i.0) {
                    Some(f) => *f,
                    None => self.module.add_function(
                        &i.0,
                        self.context.i32_type().fn_type(&[], false),
                        None,
                    ),
                };
                let compiler_address = self
                    .builder
                    .build_call(self.module.get_function("gca").unwrap(), &[], "gcacall")
                    .try_as_basic_value()
                    .unwrap_left();
                self.kek_input = Some(i.0.clone());
                let kek_result = self
                    .builder
                    .build_call(
                        self.module.get_function("kek").unwrap(),
                        &[BasicMetadataValueEnum::from(compiler_address)],
                        "called_keke",
                    )
                    .try_as_basic_value()
                    .unwrap_left();
                let ptr = self.builder.build_int_to_ptr(
                    kek_result.into_int_value(),
                    self.context
                        .i64_type()
                        .fn_type(&[], false)
                        .ptr_type(inkwell::AddressSpace::Generic),
                    "int-to-ptr",
                );
                // ;
                Some(
                    self.builder
                        .build_call(CallableValue::try_from(ptr).unwrap(), &[], "functioncall")
                        .try_as_basic_value()
                        .unwrap_left(),
                )
            } // change kek to return pointer
              // figure it out
        }
    }
    fn compile_statement(&mut self, s: &Statement) -> Option<BasicValueEnum<'ctx>> {
        match s {
            Statement::Return(e) => {
                let compiled_expression = self.compile_expression(e);
                self.builder.build_return(Some(
                    &compiled_expression.expect("Cannot return an expression of that type."),
                ));
                return compiled_expression;
            }
        }
    }
    fn compile_ty(&mut self, t: &Ty) -> BasicTypeEnum<'ctx> {
        match t {
            Ty::I32 => inkwell::types::BasicTypeEnum::IntType(self.context.i32_type()),
            Ty::I64 => inkwell::types::BasicTypeEnum::IntType(self.context.i64_type()),
            Ty::Identifier(_) => todo!(),
        }
    }
    fn compile_function(&mut self, f: &Function) {
        println!("Compiling function `{}`", f.name.0);
        let ty = self.compile_ty(&f.ty);
        let fv = self
            .module
            .add_function(&f.name.0, ty.fn_type(&[], false), None);

        match self.functions.insert(f.name.0.clone(), fv) {
            Some(f) => {
                f.replace_all_uses_with(fv);
                unsafe { f.delete() };
            }
            None => println!("all good. first occurence"),
        }
        let entry = self.context.append_basic_block(fv, "entry");
        self.builder.position_at_end(entry);
        self.compile_expression(&f.expr);
    }
    fn compile_item(&mut self, i: &Item) {
        match i {
            Item::Function(f) => self.compile_function(f),
        }
    }
    fn compile_program(&mut self, p: &Program) {
        println!("Recompiling program");
        for item in &p.0 {
            self.compile_item(item);
        }
        if self.module.get_function("_main_proxy").is_none() {
            let z = self.module.add_function(
                "_main_proxy",
                self.context.i64_type().fn_type(&[], false),
                None,
            );
            let entry = self.context.append_basic_block(z, "entry");
            self.builder.position_at_end(entry);
            let val = self
                .builder
                .build_call(self.module.get_function("main").unwrap(), &[], "ctm")
                .try_as_basic_value()
                .unwrap_left();
            self.builder.build_return(Some(&val));
        }
    }
    fn init(&mut self) {
        let kekfv = self.module.add_function(
            "kek",
            self.context.i64_type().fn_type(
                &[BasicMetadataTypeEnum::IntType(self.context.i64_type())],
                false,
            ),
            None,
        );
        self.engine.add_global_mapping(&kekfv, kek as usize);
        let f = self
            .module
            .add_function("gca", self.context.i64_type().fn_type(&[], false), None);
        let entry = self.context.append_basic_block(f, "entry");
        self.builder.position_at_end(entry);
        self.builder.build_return(Some(
            &self
                .context
                .i64_type()
                .const_int(self as *const Compiler as u64, false),
        ));
        println!("Built return");
    }
}
#[no_mangle]
/// Intercept all function calls and get an updated poiinter
extern "C" fn kek(compiler: &mut Compiler) -> u64 {
    let i = compiler.kek_input.as_ref().unwrap();
    let f = compiler.functions.get(i).unwrap();
    println!("Passing through kek!");
    unsafe {
        compiler
            .engine
            .get_function::<unsafe extern "C" fn() -> i64>(f.get_name().to_str().unwrap())
            .unwrap()
            .inner as u64
    }
}
fn main() {
    Target::initialize_native(&InitializationConfig::default()).unwrap();
    let mut program = Program::from(parse().unwrap());
    let context = Context::create();
    let module = context.create_module(".module");
    let engine = module
        .create_jit_execution_engine(inkwell::OptimizationLevel::None)
        .unwrap();
    let builder = context.create_builder();
    let mut compiler = Compiler {
        builder,
        context: &context,
        module,
        engine,
        functions: HashMap::new(),
        kek_input: None,
        kek_result: None,
    };

    compiler.init();
    compiler.compile_program(&program);
    compiler.module.print_to_stderr();
    // compiler
    // .engine
    // .add_global_mapping(&compiler.module.get_function("kek").unwrap(), Compiler::kek as usize);

    loop {
        let parsed = parse();
        match parsed {
            Some(new_prog) => {
                let new_prog = Program::from(new_prog);
                if program != new_prog {
                    println!("AST Updated.");
                    compiler.compile_program(&new_prog);
                    compiler.engine.remove_module(&compiler.module).unwrap();
                    compiler.engine.add_module(&compiler.module).unwrap();
                    compiler.module.print_to_stderr();
                }
                program = new_prog;
            }
            None => {
                println!("Failed to parse new thing!");
            }
        }

        unsafe {
            // let v = compiler
            //     .engine
            //     .run_function(*compiler.functions.get("main").unwrap(), &[]);
            // compiler.engine.get_function_value(fn_name)

            let v: JitFunction<unsafe extern "C" fn() -> i64> =
                compiler.engine.get_function("_main_proxy").unwrap();
            println!("Result: {}", v.call());
        };
        std::thread::sleep(Duration::from_millis(500));
    }
}
