use crate::ast;
use crate::common::type_context::Type;
use crate::common::utils::modulepath::ModulePath;

pub struct VarDecl {
    pub name: ModulePath,
    pub ty: Type, //a proper type
    pub value: Option<ast::statements::expressions::Expr>
}