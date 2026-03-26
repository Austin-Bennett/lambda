use crate::ast::statements::expressions::{Expr, ExprSyntax};
use crate::common::sourcemap::SourceMap;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::typed_ast::typing::ty::TypeInfo;





//only numbers are allowed in these const expressions
pub fn eval_array_len_expr_uint(expr: &ExprSyntax, compiler: &mut Compiler) -> Option<u128> {
    match &expr.data {
        Expr::Identifier(ident) => {
            compiler.emit_compile_message(
                CompileMessage::new(
                    expr.smap.clone(),
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
                            expr.smap.clone(),
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
                        expr.smap.clone(),
                        format!("Expected single expression, got tuple"),
                        CompileMessageType::Error
                    )
                );
                None
            } else {
                eval_array_len_expr_uint(&values[0], compiler)
            }
        }
        Expr::BinaryOp(op) => {
            let lhs = eval_array_len_expr_uint(&op.lhs, compiler)?;
            let rhs = eval_array_len_expr_uint(&op.rhs, compiler)?;
            
            match op.op.tk { 
                
                "+" => Some(lhs + rhs),
                "-" => Some(lhs - rhs),
                "*" => Some(lhs * rhs),
                "/" => Some(lhs / rhs),
                
                _ => {
                    compiler.emit_compile_message(CompileMessage::new(
                        expr.smap.clone(),
                        format!("Unknown binary operator: {}", op.op.tk),
                        CompileMessageType::Error,
                    ));
                    None
                }
            }
            
        }
        Expr::UnaryOp(op) => {
            let v = eval_array_len_expr_uint(&op.operand, compiler)?;

            match op.op.tk {
                "-" => {
                    compiler.emit_compile_message(
                        CompileMessage::new(
                            expr.smap.clone(),
                            format!("Cannot negate unsigned expression"),
                            CompileMessageType::Error
                        )
                    );
                    
                    None
                }

                _ => {
                    compiler.emit_compile_message(CompileMessage::new(
                        expr.smap.clone(),
                        format!("Unknown unary operator: {}", op.op.tk),
                        CompileMessageType::Error,
                    ));
                    None
                }
            }
        }
        Expr::CallOp(op) => {
            let caller = eval_array_len_expr_uint(&op.caller, compiler)?;
            if op.arguments.len() != 1 {
                compiler.emit_compile_message(CompileMessage::new(
                    expr.smap.clone(),
                    format!("Expected 1 argument, got {}", op.arguments.len()),
                    CompileMessageType::Error,
                ));
                
                None
            } else {
                let v = eval_array_len_expr_uint(&op.arguments[0], compiler)?;
                Some(caller * v)
            }
        }
    }
}