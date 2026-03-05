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
    fn compile_expression(&mut self, expr: &ExprNode, context: &Option<ExpressionContext>, label: &mut u64) {
        match expr {
            ExprNode::CallOperation { caller, args } => {
                //push arguments to the stack
                for a in args {
                    self.compile_expression(a, context, label);
                    self.builder.add(Instruction::Push(Register::Ret));
                }
                //call
                self.builder.pseudo(PseudoInstruction::Call(caller.clone()));
                self.builder.add(Instruction::PopN(args.len() as u64));
            }
            ExprNode::BinaryOperation { op, operands } => {
                self.compile_expression(&operands.1, context, label);
                self.builder.add(Instruction::Push(Register::Ret));
                self.compile_expression(&operands.0, context, label);
                self.builder
                    .add(Instruction::Pop(Register::Aux));

                match *op {
                    "+" => {
                        self.builder
                            .add(Instruction::Add);
                    }
                    "-" => {
                        self.builder
                            .add(Instruction::Sub);
                    }
                    "*" => {

                        self.builder
                            .add(Instruction::Mul);
                    }
                    "/" => {
                        self.builder
                            .add(Instruction::Div);
                    }
                    "<=" => {
                        let l1 = label.to_string();
                        *label += 1;
                        let l2 = label.to_string();
                        *label += 1;
                        self.builder
                            .add(Instruction::Cmp)
                            .pseudo(PseudoInstruction::JumpLessEq(l1.clone()))
                            .add(Instruction::Mov(Register::Ret, 0))
                            .pseudo(PseudoInstruction::Jump(l2.clone()))
                            .label(l1)
                            .add(Instruction::Mov(Register::Ret, 1))
                            .label(l2);


                    }
                    _ => panic!("Unknown operator: {}", op)
                }
            }
            ExprNode::UnaryOperation { op, operand } => {
                self.compile_expression(&*operand, context, label);
                if *op == "-" {
                    self.builder.add(Instruction::Negate);
                }
            }
            ExprNode::Ident(ident) => {
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
            ExprNode::Num(n) => {
                //treat all numbers like u64's
                self.builder.add(Instruction::Mov(Register::Ret, n.literal.parse().unwrap()));
            }
            ExprNode::Void => {}
            ExprNode::Error(e) => {
                panic!("{}", e)
            }
        }
    }
    
    fn compile_statement(&mut self, s: &Statement, ctxt: &Option<ExpressionContext>, label: &mut u64) {
        match s {
            Statement::Expression(e) => {
                self.compile_expression(e, ctxt, label)
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

                let l3 = label.to_string();
                *label += 1;


                //compile the predicate
                self.compile_expression(pred, ctxt, label);
                self.builder
                    .add(Instruction::Mov(Register::Aux, 0))
                    .add(Instruction::Cmp)
                    .pseudo(PseudoInstruction::JumpNEq(l1.clone()))
                    .pseudo(PseudoInstruction::Jump(l2.clone()))
                    .label(l1);

                self.compile_block_builder(code, ctxt, label);

                self.builder
                    .pseudo(PseudoInstruction::Jump(l3.clone()))
                    .label(l2);
                self.compile_block_builder(else_block, ctxt, label);
                self.builder.label(l3);
            }
        }
    }
}