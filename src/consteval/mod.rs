use crate::ast::statements::expressions::Expr;
use crate::common::sourcemap::SourceMap;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::typed_ast::typing::ty::TypeInfo;

//stores information about known constants- there currently are none so this is just empty
pub struct ConstContext {
    
}




//evaluates e as a unsigned integer expression,
//in a constant context, an identifier is not allowed, currently
//smap is required for compile errors
//this is good enough for now, but eventually we will need a full interpreter to 
//run these expressions at compile time, most likely some kind of jit interpreter
//rather than a full bytecode interpreter
pub fn eval_expr_const_uint(e: &Expr, smap: &SourceMap, context: &ConstContext, compiler: &mut Compiler) -> Option<u128> {
    match e {
        Expr::Identifier(ident) => {
            compiler.emit_compile_message(
                CompileMessage::new(
                    smap.clone(),
                    format!("non-const identifier found in constant expression: {}", ident),
                    CompileMessageType::Error,
                )
            );
            
            None
        },
        Expr::IntLiteral(i) => {
            match i.as_u128() { 
                Ok(v) => Some(v),
                Err(e) => {
                    compiler.emit_compile_message(
                        CompileMessage::new(
                            smap.clone(),
                            e.to_string(),
                            CompileMessageType::Error,
                        )
                    );
                    None
                }
            }
        }
        Expr::Tuple(values) => {
            if values.len() != 1 {
                compiler.emit_compile_message(
                    CompileMessage::new(
                        smap.clone(),
                        format!("Expected single expression, got tuple"),
                        CompileMessageType::Error
                    )
                );
                None
            } else {
                eval_expr_const_uint(&values[0], smap, context, compiler)
            }
        }
        Expr::BinaryOp(op) => {
            let lhs = eval_expr_const_uint(&op.lhs, smap, context, compiler)?;
            let rhs = eval_expr_const_uint(&op.rhs, smap, context, compiler)?;
            
            match op.op.tk { 
                
                "+" => Some(lhs + rhs),
                "-" => Some(lhs - rhs),
                "*" => Some(lhs * rhs),
                "/" => Some(lhs / rhs),
                
                _ => {
                    compiler.emit_compile_message(CompileMessage::new(
                        smap.clone(),
                        format!("Unknown binary operator: {}", op.op.tk),
                        CompileMessageType::Error,
                    ));
                    None
                }
            }
            
        }
        Expr::UnaryOp(op) => {
            let v = eval_expr_const_uint(&op.operand, smap, context, compiler)?;

            match op.op.tk {
                "-" => {
                    compiler.emit_compile_message(
                        CompileMessage::new(
                            smap.clone(),
                            format!("Cannot negate unsigned expression"),
                            CompileMessageType::Error
                        )
                    );
                    
                    None
                }

                _ => {
                    compiler.emit_compile_message(CompileMessage::new(
                        smap.clone(),
                        format!("Unknown unary operator: {}", op.op.tk),
                        CompileMessageType::Error,
                    ));
                    None
                }
            }
        }
        Expr::CallOp(op) => {
            let caller = eval_expr_const_uint(&op.caller, smap, context, compiler)?;
            if op.arguments.len() != 1 {
                compiler.emit_compile_message(CompileMessage::new(
                    smap.clone(),
                    format!("Expected 1 argument, got {}", op.arguments.len()),
                    CompileMessageType::Error,
                ));
                
                None
            } else {
                let v = eval_expr_const_uint(&op.arguments[0], smap, context, compiler)?;
                Some(caller * v)
            }
        }
    }
}