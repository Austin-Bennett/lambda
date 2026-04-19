use std::collections::{HashMap, VecDeque};
use crate::ast;
use crate::ast::Syntax;
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::common::utils::outcome::Outcome;
use crate::common::utils::progress::Progress;
use crate::common::utils::Todo;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::lexer::token::Token;
use crate::typed_ast::ast::items::function::{Function, FunctionSignature};
use crate::typed_ast::typing::scope::AvailableContext;

pub struct LModule {
    pub smap: SourceMap,
    pub ast: Vec<ast::Item>,
    pub dependencies: Vec<ModulePath>, //module dependencies
}


impl LModule {
    pub fn parse_untyped(path: ModulePath, mut tokens: VecDeque<Token>, smap: SourceMap, dependencies: Vec<ModulePath>, compiler: &mut Compiler) -> Self {
        let mut items = Vec::new();

        while !tokens.is_empty() {

            let Some(mut item) = ast::ItemSyntax::parse(&mut tokens, compiler) else {
                if let Some(next) = tokens.pop_front() {
                    compiler.emit_compile_message(
                        CompileMessage::new(
                            next.smap,
                            format!("Unexpected token {:?}", next.typ),
                            CompileMessageType::Error,
                        )
                    );
                    continue;
                } else {
                    break;
                }
            };
            items.push(item.data);
        }

        


        Self{
            smap,
            dependencies,
            ast: items
        }

    }
}


pub struct LTypedModule {
    pub smap: SourceMap,
    pub functions: Vec<Function>,
}

impl LTypedModule {
    pub fn from_ast(module: &LModule, compiler: &mut Compiler, context: &mut AvailableContext) -> Self {


        let mut functions = Vec::new();

        //structures get managed by the compiler as we need them, so we only care about functions
        for item in &module.ast {
            if let ast::Item::Func(function) = item {
                let Some(func) = Function::from_ast(function, compiler, context) else { continue; };
                functions.push(func);
            }
        }



        LTypedModule{
            smap: module.smap.clone(),
            functions
        }
    }
}