use crate::ast::GenericSyntax;
use crate::ast::statements::{Statement};
use crate::ast::statements::vardecl::VarDeclSyntax;
use crate::compiler::Compiler;
use crate::typed_ast::ast::statements::expression::TypedExpr;
use crate::typed_ast::ast::statements::ret::{TypedReturn};
use crate::typed_ast::ast::statements::vardecl::TypedVarDecl;
use crate::typed_ast::typing::scope::AvailableContext;

pub mod expression;
pub mod vardecl;
pub mod ret;

pub enum TypedStatement {
    VarDecl(TypedVarDecl),
    Return(TypedReturn),
    Expr(TypedExpr),
}



impl TypedStatement {
    pub fn from_ast(statement: &Statement, compiler: &mut Compiler, context: &mut AvailableContext) -> Option<Self> {
        match statement {
            Statement::VariableDeclaration(vd) => {
                Some(Self::VarDecl(TypedVarDecl::from_ast(vd, compiler, context)?))
            }
            Statement::Expression(expr) => {
                Some(Self::Expr(TypedExpr::from_ast(expr, compiler, context)?))
            }
            Statement::Return(ret) => {
                Some(Self::Return(TypedReturn::from_ast(ret, compiler, context)?))
            }
        }
    }
}