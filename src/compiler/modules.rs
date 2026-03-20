use std::collections::VecDeque;
use crate::ast;
use crate::ast::Syntax;
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::common::utils::outcome::Outcome;
use crate::common::utils::progress::Progress;
use crate::common::utils::Todo;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::lexer::token::Token;

pub struct LModule {
    pub smap: SourceMap,
    pub ast: Vec<Result<Todo, ast::Item>>,
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
            item.append_namespace(&path);
            items.push(Err(item.data));
        }

        


        Self{
            smap,
            dependencies,
            ast: items
        }

    }
}
