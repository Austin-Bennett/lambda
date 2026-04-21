use std::fmt::{Debug, Formatter};
use std::ops::Deref;
use crate::ast::GenericSyntax;
use crate::ast::statements::expressions::ExprSyntax;
use crate::ast::statements::if_stmt::{ElseStatement, IfSyntax};
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::typed_ast::ast::block::TypedBlockSyntax;
use crate::typed_ast::ast::statements::expression::TypedExpr;
use crate::typed_ast::typing::scope::AvailableContext;
use crate::typed_ast::typing::tcontext::TypeContext;
use crate::typed_ast::typing::ty::TypeId;

pub enum TypedElseStatement {
    If(TypedIfSyntax),
    Else(TypedBlockSyntax)
}

pub struct TypedIfStatement {
    pub predicate: TypedExpr,
    pub code: TypedBlockSyntax,
    pub otherwise: Option<Box<TypedElseStatement>>
}


pub type TypedIfSyntax = GenericSyntax<TypedIfStatement>;

impl TypedIfSyntax {
    pub fn from_ast(expr: &IfSyntax, function_return: TypeId, compiler: &mut Compiler, context: &mut AvailableContext<TypeId>) -> Option<Self> {
        let pred = TypedExpr::from_ast(&expr.data.predicate, compiler, context)?;
        if pred.ty != compiler.type_context.bool {
            compiler.emit_compile_message(
                CompileMessage::new(
                    pred.smap,
                    format!("Expected boolean expression, got {}", compiler.type_context.name_of(pred.ty).unwrap()),
                    CompileMessageType::Error
                )
            );

            return None;
        }

        let code = TypedBlockSyntax::from_ast(&expr.data.code, function_return, compiler, context)?;

        let otherwise = if let Some(e1se) = &expr.data.otherwise {

            Some(match e1se.deref() {
                ElseStatement::If(i4) => {
                    TypedElseStatement::If(TypedIfSyntax::from_ast(i4, function_return, compiler, context)?)
                },
                ElseStatement::Else(block) => {
                    TypedElseStatement::Else(TypedBlockSyntax::from_ast(block, function_return, compiler, context)?)
                }
            })

        } else {
            None
        };

        Some(Self{
            data: TypedIfStatement{
                predicate: pred,
                code,
                otherwise: otherwise.map(|o| Box::new(o))
            },
            smap: expr.smap.clone(),
        })
    }

    pub fn to_string(&self, context: &TypeContext) -> String {
        let mut s = format!("if {:?} {{", self.data.predicate);

        if self.data.code.data.is_empty() {
            s.push('}');
        } else {
            s.push('\n');
            for st in &self.data.code.data {
                s.push('\t');
                s += &st.to_string(context);
                s.push('\n');
            }
            s.push('}');
        }

        if let Some(e1se) = &self.data.otherwise {
            match e1se.deref() {
                TypedElseStatement::If(i4) => {
                    s += &i4.to_string(context);
                }
                TypedElseStatement::Else(e) => {
                    s += "else {";

                    if e.data.is_empty() {
                        s.push('}');
                    } else {
                        s.push('\n');
                        for st in &e.data {
                            s += &st.to_string(context);
                            s.push('\n');
                        }
                        s.push('}');
                    }
                }
            }
        }

        s
    }
}