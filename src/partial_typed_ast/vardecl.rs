use crate::ast;
use crate::common::type_context::{PartialType, TypeName};
use crate::common::utils::modulepath::ModulePath;

pub struct PTVarDecl {
    pub name: ModulePath,
    pub ty: PartialType, //a proper type
    pub value: Option<ast::statements::expressions::Expr>
}