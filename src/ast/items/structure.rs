use std::collections::VecDeque;
use std::fmt::{Debug, Formatter};
use crate::ast::{GenericSyntax, Syntax};
use crate::ast::statements::vardecl::{VarDecl, VarDeclSyntax};
use crate::common::sourcemap::SourceMap;
use crate::compiler::{CompileMessage, Compiler};
use crate::lexer::token::{ExpressionToken, FeatureToken, StatementToken, Token, TokenType};
use crate::unpack_opt_tk;

pub mod lstruct {
    use super::*;
    
    #[derive(Clone, Hash)]
    pub struct Structure {
        pub name: String,
        pub type_parameters: Vec<String>,
        pub members: Vec<VarDecl>,
    }
}

impl Debug for lstruct::Structure {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "struct {} {{\n", self.name)?;
        for m in &self.members {
            write!(f, "\t{:?},\n", m)?;
        }
        write!(f, "}}")
    }
}

pub type StructureSyntax = GenericSyntax<lstruct::Structure>;

impl Syntax for StructureSyntax {
    fn parse(tokens: &mut VecDeque<Token>, compiler: &mut Compiler) -> Option<Self>
    where
        Self: Sized
    {
        let unpack_opt_tk!( TokenType::Statement(StatementToken::StructKW), _ ) = tokens.get(0) else { return None };
        let unpack_opt_tk!( TokenType::Statement(StatementToken::StructKW), mut smap ) = tokens.pop_front() else { unreachable!() };



        // optional type parameters: struct<T, U> Name { ... }
        let type_parameters = if let unpack_opt_tk!(
            TokenType::Expression(ExpressionToken::Operator(crate::common::operator::Operator { tk: "<", .. })), _
        ) = tokens.get(0) {
            tokens.pop_front(); // consume '<'
            let mut tps = Vec::new();
            loop {
                if let unpack_opt_tk!(
                    TokenType::Expression(ExpressionToken::Operator(crate::common::operator::Operator { tk: ">", .. })), _
                ) = tokens.get(0) {
                    tokens.pop_front();
                    break;
                }
                let next = tokens.pop_front();
                if let unpack_opt_tk!(TokenType::Expression(ExpressionToken::Identifier(tp)), _) = next {
                    tps.push(tp);
                } else {
                    compiler.emit_compile_message(CompileMessage::expected_token_error(
                        smap.clone(), "type parameter name", "generic struct declaration", next,
                    ));
                    break;
                }
                if let unpack_opt_tk!(
                    TokenType::Expression(ExpressionToken::Operator(crate::common::operator::Operator { tk: ">", .. })), _
                ) = tokens.get(0) {
                    tokens.pop_front();
                    break;
                }
                if let unpack_opt_tk!(TokenType::Feature(FeatureToken::Comma), _) = tokens.get(0) {
                    tokens.pop_front();
                } else {
                    break;
                }
            }
            tps
        } else {
            Vec::new()
        };

        //next token should be the struct name
        let next = tokens.pop_front();
        let unpack_opt_tk!( TokenType::Expression(ExpressionToken::Identifier(s)), imap )
            = next else {
            compiler.emit_compile_message(
                CompileMessage::expected_token_error(
                    smap,
                    "identifier",
                    "'struct' keyword",
                    next
                )
            );
            return None;
        };
        smap.extend(imap);

        let mut res = StructureSyntax{
            data: lstruct::Structure{
                name: s,
                type_parameters,
                members: Vec::new(),
            },
            smap
        };

        //check for the '{'
        
        let next = tokens.pop_front();
        let unpack_opt_tk!( TokenType::Feature(FeatureToken::OpenBrace), smap ) = next else {
            compiler.emit_compile_message(CompileMessage::expected_token_error(
                res.smap,
                "{",
                format!("struct \'{}\' declaration", res.data.name),
                next
            ));
            return None;
        };

        res.smap.extend(smap);
        
        //time to parse members
        while let Some(vdecl) = VarDeclSyntax::parse(tokens, compiler) {
            //parse away any commas, expect at least 1
            let mut count = 0;
            while let unpack_opt_tk!(TokenType::Feature(FeatureToken::Comma), _) = tokens.get(0) { tokens.pop_front(); count += 1; }
            if count == 0 {
                let next = tokens.pop_front();
                compiler.emit_compile_message(
                    CompileMessage::expected_token_error(
                        vdecl.smap.clone(),
                        ",",
                        format!("struct member declaration {}: {:?}", vdecl.data.name, vdecl.data.ty),
                        next
                    )
                );
            }

            //add this member
            res.smap.extend(vdecl.smap);
            res.data.members.push(
                vdecl.data
            )
        }

        //expect the '}'
        let next = tokens.pop_front();
        let unpack_opt_tk!( TokenType::Feature(FeatureToken::CloseBrace), cbsmap ) = next else {
            compiler.emit_compile_message(CompileMessage::expected_token_error(
                res.smap,
                "}",
                format!("struct members declaration in struct: {}", res.data.name),
                next
            ));
            return None;
        };
        res.smap.extend(cbsmap);

        Some(res)
    }

    fn get_sourcemap(&self) -> &SourceMap {
        & self.smap
    }
}