use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use crate::ast::block::BlockSyntax;
use crate::ast::{GenericSyntax, Syntax};
use crate::ast::statements::expressions::ExprSyntax;
use crate::common::sourcemap::SourceMap;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::lexer::token::{StatementToken, Token, TokenType};
use crate::unpack_opt_tk;

pub struct WhileStatement {
    pub condition: ExprSyntax,
    pub code: BlockSyntax,
}

impl Debug for WhileStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "while {:?} {{", self.condition.data)?;

        if !self.code.data.is_empty() {
            f.write_str("\n")?;

            for s in &self.code.data {
                write!(f, "\t{:?}", s)?;
                f.write_str("\n")?;
            }

        }
        f.write_str("}")
    }
}

pub type WhileSyntax = GenericSyntax<WhileStatement>;

impl Syntax for WhileSyntax {
    fn parse(tokens: &mut VecDeque<Token>, context: &mut Compiler) -> Option<Self>
    where
        Self: Sized
    {
        let unpack_opt_tk!(TokenType::Statement(StatementToken::WhileKW), _) = tokens.get(0) else {
            return None;
        };

        let unpack_opt_tk!(_, mut smap) = tokens.pop_front() else { unreachable!() };

        let Some(condition) = ExprSyntax::parse(tokens, context) else {
            context.emit_compile_message(
                CompileMessage::new(
                    smap,
                    "expected boolean expression after \"while\"".to_string(),
                    CompileMessageType::Error
                )
            );
            return None;
        };

        smap.extend(&condition.smap);

        let Some(blk) = BlockSyntax::parse(tokens, context) else {
            context.emit_compile_message(
                CompileMessage::new(
                    smap,
                    "expected code block after \"while\" keyword".to_string(),
                    CompileMessageType::Error
                )
            );
            return None;
        };
        
        smap.extend(&blk.smap);

        Some(Self{
            data: WhileStatement{
                condition,
                code: blk
            },
            smap
        })
    }

    fn get_sourcemap(&self) -> &SourceMap {
        &self.smap
    }
}