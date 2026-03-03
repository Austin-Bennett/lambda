use crate::lambda_parser::ExprNode;
use crate::lambda_parser::statement::{Block, Statement};
use crate::lvm::lexecutable::{LExecutableBuilder, LPseudoExecutable, PseudoInstruction};

pub struct Compiler {
    components: Vec<LPseudoExecutable>,
    builder: LExecutableBuilder,
}

impl Compiler {
    pub fn new() -> Self {
        Self{
            components: Vec::new(),
            builder: LExecutableBuilder::new()
        }
    }


    pub fn compile_block(&mut self, b: &Block) {
        
        self.compile_block_builder(b);
        
        self.components.push(self.builder.build_pseudo())
    }
    
    fn compile_block_builder(&mut self, blk: &Block) {
        for s in blk {
            self.compile_statement(s)
        }
    }
    fn compile_expression(&mut self, expr: &ExprNode) {
        match expr {
            ExprNode::CallOperation { caller, args } => {}
            ExprNode::BinaryOperation { op, operands } => {}
            ExprNode::UnaryOperation { op, operand } => {}
            ExprNode::Ident(ident) => {}
            ExprNode::Num(n) => {}
            ExprNode::Argument(arg) => {}
            ExprNode::Void => {}
            ExprNode::Error(e) => {}
        }
    }
    
    fn compile_statement(&mut self, s: &Statement) {
        match s {
            Statement::Expression(e) => {
                
            }
            Statement::FunctionDeclaration { name, args, body } => {
                
            }
            Statement::IfStatement { pred, code, else_block } => {}
        }
    }
}