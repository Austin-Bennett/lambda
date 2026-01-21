use crate::lambda_parser::ExprNode::{BinaryOperation, Num, UnaryOperation, Void};
use crate::lambda_parser::{Env, ExprNode, FloatHelpers, Variable};
use std::collections::HashMap;


impl ExprNode {

    pub fn to_number(&self, varmap: &Env) -> Option<f64> {
        match self {
            ExprNode::Ident(s) => {
                if let Some(Variable::Number(n)) = varmap.get(s) {
                    Some(*n)
                } else {
                    None
                }
            },
            ExprNode::Num(n) => Some(*n),
            _ => None
        }
    }

    #[allow(dead_code)]
    pub fn is_ident(&self) -> bool {
        if let ExprNode::Ident(_) = self {
            true
        } else {
            false
        }
    }

    pub fn binary_operation(self, op: &'static str, other: ExprNode, varmap: &mut Env) -> Result<ExprNode, String> {
        if op == "=" {
            //special case: variable declaration
            return if let ExprNode::Ident(ident) = self {
                if let Some(n) = other.to_number(varmap) {
                    varmap.set(ident, Variable::Number(n));
                    Ok(Num(n))
                } else {
                    Ok(BinaryOperation { op, operands: Box::new((ExprNode::Ident(ident), other)) })
                }
            } else if let ExprNode::CallOperation { caller, args } = self {
                let mut inps = Vec::new();

                let ExprNode::Ident(caller) = *caller else { return Ok(ExprNode::CallOperation { caller, args }); };

                for arg in &args {
                    if let ExprNode::Ident(s) = arg {
                        inps.push(s.clone());
                    } else {
                        return Ok(ExprNode::CallOperation {caller: Box::new(ExprNode::Ident(caller)), args});
                    }
                }

                varmap.set(caller, Variable::Function { inputs: inps, body: other });

                Ok(Void)
            } else {
                Ok(BinaryOperation { op, operands: Box::new((self, other)) })
            }
        }

        let Some(lhs) = self.to_number(varmap) else { return Ok(BinaryOperation { op, operands: Box::new((self, other)) }); };
        let Some(rhs) = other.to_number(varmap) else { return Ok(BinaryOperation { op, operands: Box::new((self, other)) }); };

        match op {

            "+" => Ok(Num(lhs + rhs)),
            "-" => Ok(Num(lhs - rhs)),
            "*" => Ok(Num(lhs * rhs)),
            "/" => Ok(Num(lhs / rhs)),
            "**" => Ok(Num(lhs.powf(rhs))),
            
            "<" => Ok(if lhs < rhs && !lhs.equates(rhs) { Num(1.0) } else { Num(0.0) }),
            ">" => Ok(if lhs > rhs && !lhs.equates(rhs) { Num(1.0) } else { Num(0.0) }),
            "<=" => Ok(if lhs < rhs || lhs.equates(rhs) { Num(1.0) } else { Num(0.0) }),
            ">=" => Ok(if lhs > rhs || lhs.equates(rhs) { Num(1.0) } else { Num(0.0) }),
            "!=" => Ok(if !lhs.equates(rhs) { Num(1.0) } else { Num(0.0) }),
            "==" => Ok(if lhs.equates(rhs) { Num(1.0) } else { Num(0.0) }),

            _ => Err(format!("Unknown binary operator: {}", op))
        }
    }

    pub fn unary_operation(self, op: & str, varmap: &mut Env) -> Result<ExprNode, String> {
        match op {
            "+" => Ok(self),
            "-" => {
                if let Some(n) = self.to_number(varmap) {
                    Ok(Num(n))
                } else {
                    Ok(UnaryOperation { op: "-", operand: Box::new(self) })
                }
            }
            _ => Err(format!("Unknown unary operator: {}", op))
        }
    }

    pub fn solve_ref(&mut self, varmap: &mut Env) -> Result<ExprNode, String> {
        let old = std::mem::replace(self, Num(0.0));
        old.solve(varmap)?;
        
    }

    //simplifies the expression as much as possible, which may end up in a single value that can be printed
    pub fn solve(mut self, varmap: &mut Env) -> Result<ExprNode, String> {

        self = match self {
            ExprNode::CallOperation { caller, mut args } => {
                if let ExprNode::Ident(ident) = &*caller {
                    if let Some(Variable::NativeFunction(func)) = varmap.get(&ident) {
                        #[allow(suspicious_double_ref_op)]
                        let func = func.clone();
                        return func(varmap, args);
                    }
                    
                    let func = match varmap.get(&ident) {
                        Some(Variable::Function { inputs, body }) => (inputs.clone(), body.clone()),
                        _ => return Ok(ExprNode::CallOperation { caller, args }),
                    };

                    let mut map = HashMap::new();

                    for (i, j) in func.0.iter().zip(&mut args) {
                        let old = std::mem::replace(j, ExprNode::Num(0.0));
                        *j = old.solve(varmap)?;
                        if let Some(n) = j.to_number(varmap) {
                            map.insert(i.clone(), Variable::Number(n));
                        } else if let ExprNode::Ident(s) = j {
                            if let Some(v) = varmap.get(s) {
                                map.insert(i.clone(), v.clone());
                            } else {
                                return Ok(ExprNode::CallOperation { caller, args });
                            }
                        }
                    }

                    // Now borrow mutably only for evaluation
                    varmap.push_scope(map);
                    let res = func.1.solve(varmap)?;
                    varmap.pop_scope();

                    res

                } else {
                    ExprNode::CallOperation { caller, args }
                }
            }
            ExprNode::BinaryOperation { op, operands } => {
                let (mut lhs, mut rhs) = *operands;
                lhs = lhs.solve(varmap)?;
                rhs = rhs.solve(varmap)?;

                lhs.binary_operation(op, rhs, varmap)?
            }
            ExprNode::UnaryOperation { op, operand } => {
                let operand = operand.solve(varmap)?;
                operand.unary_operation(op, varmap)?
            }
            ExprNode::Ident(ident) => if let Some(Variable::Number(n)) = varmap.get(&ident) {
                ExprNode::Num(*n)
            } else {
                ExprNode::Ident(ident)
            }
            ExprNode::Num(n) => ExprNode::Num(n),
            ExprNode::Error(s) => return Err(s),
            ExprNode::Void => self,
        };

        Ok(self)
    }
}