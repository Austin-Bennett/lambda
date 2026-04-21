use crate::ast::GenericSyntax;
use crate::ast::statements::while_stmt::WhileSyntax;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::typed_ast::ast::block::TypedBlockSyntax;
use crate::typed_ast::ast::statements::expression::TypedExpr;
use crate::typed_ast::ast::statements::TypedStatement;
use crate::typed_ast::typing::scope::AvailableContext;
use crate::typed_ast::typing::tcontext::TypeContext;
use crate::typed_ast::typing::ty::TypeId;

pub struct TypedWhileStatement {
    pub condition: TypedExpr,
    pub code: TypedBlockSyntax,
}

pub type TypedWhileSyntax = GenericSyntax<TypedWhileStatement>;


impl TypedWhileSyntax {
    
    pub fn to_string(&self, context: &TypeContext) -> String {
        
        let mut s = format!("while {:?} {{", self.data.condition);
        
        if !self.data.code.data.is_empty() {
            s.push('\n');
        }
        
        for st in &self.data.code.data {
            s += &*st.to_string(context);
            s.push('\n');
        }
        s.push('}');
        
        
        s
        
    }
    
    pub fn from_ast(stmt: &WhileSyntax, function_return: TypeId, compiler: &mut Compiler, context: &mut AvailableContext<TypeId>) -> Option<Self> {
        let condition = TypedExpr::from_ast(&stmt.data.condition, compiler, context)?;
        
        if condition.ty != compiler.type_context.bool {
            compiler.emit_compile_message(
                CompileMessage::new(
                    stmt.smap.clone(),
                    format!("expected boolean expression, got expression of type {:?}", compiler.type_context.name_of(condition.ty)),
                    CompileMessageType::Error
                )
            );
            
            return None;
        }
        
        let code = TypedBlockSyntax::from_ast(&stmt.data.code, function_return, compiler, context).unwrap();
        
        
        Some(
            Self{
                data: TypedWhileStatement{
                    condition,
                    code,
                },
                smap: stmt.smap.clone(),
            }
        )
    }
}