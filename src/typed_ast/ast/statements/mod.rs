use crate::ast::statements::Statement;
use crate::common::sourcemap::SourceMap;
use crate::compiler::Compiler;
use crate::typed_ast::ast::statements::expression::TypedExpr;
use crate::typed_ast::ast::statements::if_stmt::{TypedIfStatement, TypedIfSyntax};
use crate::typed_ast::ast::statements::ret::TypedReturn;
use crate::typed_ast::ast::statements::vardecl::TypedVarDecl;
use crate::typed_ast::ast::statements::while_stmt::TypedWhileSyntax;
use crate::typed_ast::typing::scope::AvailableContext;
use crate::typed_ast::typing::tcontext::TypeContext;
use crate::typed_ast::typing::ty::TypeId;

pub mod expression;
pub mod vardecl;
pub mod ret;
pub mod if_stmt;
pub mod while_stmt;

pub enum TypedStatement {
    VarDecl(TypedVarDecl),
    Return(TypedReturn),
    Expr(TypedExpr),
    If(TypedIfSyntax),
    While(TypedWhileSyntax),
}

impl TypedStatement {
    pub fn to_string(&self, context: &TypeContext) -> String {
        match self {
            TypedStatement::VarDecl(vd) => { format!("{}: {} = {}", vd.name, context.name_of(vd.ty).unwrap(),
                    if let Some(e) = &vd.val {
                        format!("{:?}", e.value)
                    } else {
                        "".to_string()
                    }
            ) }
            TypedStatement::Return(ret) => { format!("return {:?}", ret.0.value) }
            TypedStatement::Expr(expr) => { format!("{:?}", expr.value) }
            TypedStatement::If(if_statement) => {
                if_statement.to_string(context)
            },
            TypedStatement::While(while_stmt) => {
                while_stmt.to_string(context)
            }
        }
    }
}

impl TypedStatement {
    pub fn from_ast(statement: &Statement, function_return: TypeId, compiler: &mut Compiler, context: &mut AvailableContext<TypeId>) -> Option<Self> {
        match statement {
            Statement::VariableDeclaration(vd) => {
                Some(Self::VarDecl(TypedVarDecl::from_ast(vd, compiler, context)?))
            }
            Statement::Expression(expr) => {
                let mut expr = TypedExpr::from_ast(expr, compiler, context)?;
                if expr.ty == compiler.type_context.int_literal {
                    expr.ty = compiler.type_context.int32;
                }
                Some(Self::Expr(expr))
            }
            Statement::Return(ret) => {
                Some(Self::Return(TypedReturn::from_ast(ret, function_return, compiler, context)?))
            }
            Statement::If(i4) => {
                Some(Self::If(TypedIfSyntax::from_ast(i4, function_return, compiler, context)?))
            }
            Statement::While(while_stmt) => {
                Some(Self::While(TypedWhileSyntax::from_ast(while_stmt, function_return, compiler, context)?))
            }
        }
    }
}