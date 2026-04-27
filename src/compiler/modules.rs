use std::collections::VecDeque;
use crate::ast;
use crate::ast::Syntax;
use crate::common::sourcemap::SourceMap;
use crate::common::utils::modulepath::ModulePath;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::lexer::token::{FeatureToken, Token, TokenType};
use crate::ast::ty::Type;
use crate::typed_ast::ast::items::function::Function;
use crate::typed_ast::typing::scope::AvailableContext;
use crate::typed_ast::typing::ty::TypeId;

pub struct LModule {
    pub smap: SourceMap,
    pub ast: Vec<ast::Item>,
    pub dependencies: Vec<ModulePath>, //module dependencies
}


impl LModule {
    pub fn parse_untyped(_path: ModulePath, mut tokens: VecDeque<Token>, smap: SourceMap, dependencies: Vec<ModulePath>, compiler: &mut Compiler) -> Self {
        let mut items = Vec::new();

        while !tokens.is_empty() {

            let Some(item) = ast::ItemSyntax::parse(&mut tokens, compiler) else {
                if let Some(Token{ typ: TokenType::Feature(FeatureToken::StatementEnd), smap: _ }) = tokens.get(0) {
                    tokens.pop_front();
                    continue;
                } else if let Some(next) = tokens.pop_front() {
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
    pub fn from_ast(module: &LModule, compiler: &mut Compiler, context: &mut AvailableContext<TypeId>) -> Self {


        let mut functions = Vec::new();

        //structures get managed by the compiler as we need them, so we only care about functions
        for item in &module.ast {
            match item {
                ast::Item::Func(function) => {
                    // generic functions are handled on-demand; skip them here
                    if !function.data.type_parameters.is_empty() { continue; }
                    let Some(func) = Function::from_ast(function, compiler, context) else { continue; };
                    functions.push(func);
                }
                ast::Item::Modify(modify) => {
                    // generic modify blocks are handled on-demand; skip them here
                    if !modify.data.type_parameters.is_empty() { continue; }

                    let type_name = modify.data.ty.mangle_name();
                    let Some(type_id) = compiler.resolve_type(&modify.data.ty) else { continue; };
                    for method in &modify.data.methods {
                        let mangled = format!("{}_{}", type_name, method.name);
                        let Some(func) = Function::from_method(method, type_id, &mangled, compiler, context) else { continue; };
                        functions.push(func);
                    }
                    for op in &modify.data.operators {
                        let op_mangled = format!("{}__op_{}", type_name, op.op_name);
                        let Some(func) = Function::from_operator(op, type_id, &op_mangled, compiler, context) else { continue; };
                        functions.push(func);
                    }
                }
                _ => {}
            }
        }



        LTypedModule{
            smap: module.smap.clone(),
            functions
        }
    }
}