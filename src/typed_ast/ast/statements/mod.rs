use crate::ast::statements::Statement;
use crate::compiler::Compiler;
use crate::typed_ast::ast::statements::expression::TypedExpr;
use crate::typed_ast::ast::statements::ret::TypedReturn;
use crate::typed_ast::ast::statements::vardecl::TypedVarDecl;
use crate::typed_ast::typing::scope::AvailableContext;
use crate::typed_ast::typing::tcontext::TypeContext;

pub mod expression;
pub mod vardecl;
pub mod ret;

pub enum TypedStatement {
    VarDecl(TypedVarDecl),
    Return(TypedReturn),
    Expr(TypedExpr),
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
        }
    }
}

impl TypedStatement {
    pub fn from_ast(statement: &Statement, compiler: &mut Compiler, context: &mut AvailableContext) -> Option<Self> {
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
                Some(Self::Return(TypedReturn::from_ast(ret, compiler, context)?))
            }
        }
    }
}