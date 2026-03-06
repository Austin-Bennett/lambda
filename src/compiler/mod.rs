use std::cmp::PartialEq;
use std::collections::HashMap;
use crate::lambda_parser::{ExprNode, NumericalLiteral};
use crate::lambda_parser::statement::{Block, Statement};
use crate::lvm::lbc::{Instruction, Register};
use crate::lvm::lexecutable::{link, LExecutable, LExecutableBuilder, LPseudoExecutable, PseudoInstruction};
use crate::map;

pub struct Compiler {
    builder: LExecutableBuilder,
}

pub struct ExpressionContext(HashMap<String, u64>, u64);//offset from bottom, current offset, current label


impl Compiler {
    pub fn new() -> Self {
        Self{
            builder: LExecutableBuilder::new()
        }
    }

    pub fn compile_block_executable(&mut self, b: &Block) -> LExecutable {
        let mut label = 0;

        self.compile_block_builder(b, &None, &mut label);

        self.builder.build()
    }

    pub fn compile_block(&mut self, b: &Block) -> LPseudoExecutable {

        let mut label = 0;
        
        self.compile_block_builder(b, &None, &mut label);
        
        self.builder.build_pseudo()
    }
    
    fn compile_block_builder(&mut self, blk: &Block, ctxt: &Option<ExpressionContext>, label: &mut u64) {
        for s in blk {
            self.compile_statement(s, ctxt, label);
        }
    }

    fn compile_call_expression(&mut self, caller: &String, args: &Vec<ExprNode>, context: &Option<ExpressionContext>, label: &mut u64) {
        //push arguments to the stack
        for a in args {
            self.compile_expression(a, context, label);
            self.builder.add(Instruction::Push(Register::Ret));
        }
        //call
        self.builder.pseudo(PseudoInstruction::Call(caller.clone()));
        self.builder.add(Instruction::PopN(args.len() as u64));
    }

    fn compile_identifier(&mut self, context: &Option<ExpressionContext>, ident: &String) {
        if let Some(ctxt) = context {
            if let Some(ident) = ctxt.0.get(ident) {
                self.builder
                    .add(Instruction::MovBottom(Register::Ret, *ident as i64))
                ;
            } else {
                panic!("Unknown identifier: {}", ident)
            }
        }
    }

    fn compile_expression(&mut self, expr: &ExprNode, context: &Option<ExpressionContext>, label: &mut u64) -> Register {
        match expr {
            ExprNode::CallOperation { caller, args } => {
                self.compile_call_expression(caller, args, context, label);
                Register::Ret
            }
            ExprNode::BinaryOperation { op, operands } => {
                let r1 = self.compile_expression(&operands.1, context, label);
                self.builder.add(Instruction::Push(r1));
                let r1 = self.compile_expression(&operands.0, context, label);
                if r1 != Register::Ret {
                    self.builder.add(Instruction::MovR(Register::Ret, r1));
                }
                self.builder
                    .add(Instruction::Pop(Register::Aux));

                match *op {
                    "+" => {
                        self.builder
                            .add(Instruction::Add);
                        Register::Ret
                    }
                    "-" => {
                        self.builder
                            .add(Instruction::Sub);
                        Register::Ret
                    }
                    "*" => {

                        self.builder
                            .add(Instruction::Mul);
                        Register::Ret
                    }
                    "/" => {
                        self.builder
                            .add(Instruction::Div);
                        Register::Ret
                    }
                    "<=" => {
                        self.builder
                            .add(Instruction::CmpLTE);
                        Register::Cmp
                    }
                    _ => panic!("Unknown operator: {}", op)
                }
            }
            ExprNode::UnaryOperation { op, operand } => {
                self.compile_expression(&*operand, context, label);
                if *op == "-" {
                    self.builder.add(Instruction::Negate);
                }
                Register::Ret
            }
            ExprNode::Ident(ident) => {
                self.compile_identifier(context, ident);
                Register::Ret
            }
            ExprNode::Num(n) => {
                //treat all numbers like i64's
                self.builder.add(Instruction::Mov(Register::Ret, n.literal.parse::<i64>().unwrap().cast_unsigned()));
                Register::Ret
            }
            ExprNode::Void => { Register::Ret }
            ExprNode::Error(e) => {
                panic!("{}", e)
            }
        }
    }

    fn compile_floating_expression(&mut self, expr: &ExprNode, context: &Option<ExpressionContext>, label: &mut u64) -> Register {
        match expr {
            ExprNode::CallOperation { caller, args } => {
                self.compile_call_expression(caller, args, context, label);
                Register::Ret
            }
            ExprNode::BinaryOperation { op, operands } => {
                let r1 = self.compile_expression(&operands.1, context, label);
                self.builder.add(Instruction::Push(r1));
                let r1 = self.compile_expression(&operands.0, context, label);
                if r1 != Register::Ret {
                    self.builder.add(Instruction::MovR(Register::Ret, r1));
                }
                self.builder
                    .add(Instruction::Pop(Register::Aux));

                match *op {
                    "+" => {
                        self.builder
                            .add(Instruction::FAdd);
                        Register::Ret
                    }
                    "-" => {
                        self.builder
                            .add(Instruction::FSub);
                        Register::Ret
                    }
                    "*" => {

                        self.builder
                            .add(Instruction::FMul);
                        Register::Ret
                    }
                    "/" => {
                        self.builder
                            .add(Instruction::FDiv);
                        Register::Ret
                    }
                    "<=" => {
                        self.builder
                            .add(Instruction::FCmpLTE);
                        Register::Cmp
                    }
                    _ => panic!("Unknown operator: {}", op)
                }
            }
            ExprNode::UnaryOperation { op, operand } => {
                self.compile_expression(&*operand, context, label);
                if *op == "-" {
                    self.builder.add(Instruction::FNegate);
                }
                Register::Ret
            }
            ExprNode::Ident(ident) => {
                self.compile_identifier(context, ident);
                Register::Ret
            }
            ExprNode::Num(n) => {
                //treat all numbers like f64's
                self.builder.add(Instruction::Mov(Register::Ret, n.literal.parse::<f64>().unwrap().to_bits()));
                Register::Ret
            }
            ExprNode::Void => { Register::Ret }
            ExprNode::Error(e) => {
                panic!("{}", e)
            }
        }
    }

    fn compile_statement(&mut self, s: &Statement, ctxt: &Option<ExpressionContext>, label: &mut u64) {
        match s {
            Statement::Expression(e) => {
                //todo: differentiate between floating expressions and integer expressions at the parsing level
                self.compile_floating_expression(e, ctxt, label);
            }
            Statement::FunctionDeclaration { name, args, body } => {
                let is_main = name == "main";
                if is_main {
                    self.builder.entry_label(name.clone());
                } else {
                    self.builder.label(name.clone());
                }



                self.builder.add(Instruction::Push(Register::Bottom));
                self.builder.add(Instruction::MovR(Register::Bottom, Register::Stack));
                //create a context
                let mut ctx = ExpressionContext(HashMap::new(), args.len() as u64 * 8);
                for (i, a) in args.iter().enumerate() {
                    ctx.0.insert(a.clone(), 16 + i as u64 * 8);
                }
                self.compile_block_builder(body, &Some(ctx), label);

                self.builder.add(Instruction::Pop(Register::Bottom));
                if is_main {
                    self.builder.add(Instruction::Exit);
                } else {
                    self.builder.add(Instruction::Return);
                }

            }
            Statement::IfStatement { pred, code, else_block } => {
                let l1 = label.to_string();
                *label += 1;
                let l2 = label.to_string();
                *label += 1;



                //compile the predicate
                if self.compile_expression(pred, ctxt, label) != Register::Cmp {
                    self.builder.add(Instruction::MovR(Register::Cmp, Register::Ret));
                }


                //use the comparison register to determine jumps
                self.builder
                    .pseudo(PseudoInstruction::JumpNEq(l1.clone()));

                //if Eq, then run this code
                self.compile_block_builder(code, ctxt, label);

                self.builder
                    .pseudo(PseudoInstruction::Jump(l2.clone()))
                    .label(l1);
                self.compile_block_builder(else_block, ctxt, label);

                self.builder.label(l2);

            }
        }
    }
}