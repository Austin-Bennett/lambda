use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use crate::ast::block::BlockSyntax;
use crate::ast::{GenericSyntax, Syntax};
use crate::ast::statements::expressions::ExprSyntax;
use crate::common::sourcemap::SourceMap;
use crate::compiler::{CompileMessage, CompileMessageType, Compiler};
use crate::lexer::token::{ExpressionToken, FeatureToken, StatementToken, Token, TokenType};
use crate::{token_match, unpack_opt_tk};

pub enum ElseStatement {
    If(IfSyntax),
    Else(BlockSyntax),
}

impl Debug for ElseStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "else ")?;

        match self {
            Self::If(i4) => {
                write!(f, "{:?}", i4.data)?;
            },
            Self::Else(block) => {
                write!(f, "{:?}", block)?;
            }
        }

        Ok(())
    }
}

pub struct IfStatement {
    pub predicate: ExprSyntax,
    pub code: BlockSyntax,
    pub otherwise: Option<Box<ElseStatement>>,
}

impl Debug for IfStatement {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "if {:?} {:?}", self.predicate.data, self.code)?;
        if let Some(e1se) = &self.otherwise {
            write!(f, "\t{:?}", e1se)?;
        }

        Ok(())
    }
}

pub type IfSyntax = GenericSyntax<IfStatement>;

impl Syntax for IfSyntax {
    fn parse(tokens: &mut VecDeque<Token>, context: &mut Compiler) -> Option<Self>
    where
        Self: Sized,
    {
        if !token_match!(tokens, TokenType::Statement(StatementToken::IfKW)) { return None; }

        let unpack_opt_tk!(_, mut smap) = tokens.pop_front() else { unreachable!() };


        //must eval to a boolean
        let Some(predicate) = ExprSyntax::parse(tokens, context) else {
            context.emit_compile_message(CompileMessage::new(
                smap.clone(),
                "Expected boolean expression after if expression!".to_string(),
                CompileMessageType::Error,
            ));
            return None;
        };

        smap.extend(&predicate.smap);

        //parse in the block
        let Some(block) = BlockSyntax::parse(tokens, context) else {
            context.emit_compile_message(CompileMessage::new(
                smap.clone(),
                "Expected block after if expression!".to_string(),
                CompileMessageType::Error
            ));
            return None;
        };

        smap.extend(&block.smap);

        //possibly parse in the else expression
        let else_ = if let unpack_opt_tk!(TokenType::Statement(StatementToken::ElseKW), _) = tokens.get(0) {
            let unpack_opt_tk!(TokenType::Statement(StatementToken::ElseKW), emap) = tokens.pop_front() else { unreachable!() };

            smap.extend(&emap);
            let statement = if let unpack_opt_tk!(TokenType::Statement(StatementToken::IfKW), _) = tokens.get(0) {
                let syntax = IfSyntax::parse(tokens, context)?;
                smap.extend(&syntax.smap);
                ElseStatement::If(syntax)
            } else if let unpack_opt_tk!(TokenType::Feature(FeatureToken::OpenBrace), _) = tokens.get(0) {
                let block = BlockSyntax::parse(tokens, context)?;
                smap.extend(&block.smap);
                ElseStatement::Else(block)
            } else {
                context.emit_compile_message(
                    CompileMessage::new(
                        smap.clone(),
                        "expected \"if\" or { after \"else\"".to_string(),
                        CompileMessageType::Error
                    )
                );

                return None;
            };
            
            
            Some(statement)

        } else {
            None
        };


        Some(Self{
            data: IfStatement{
                predicate,
                code: block,
                otherwise: else_.map(|e| Box::new(e)),
            },
            smap,
        })
    }

    fn get_sourcemap(&self) -> &SourceMap {
        &self.smap
    }
}