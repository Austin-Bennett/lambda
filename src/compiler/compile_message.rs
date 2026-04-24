use std::fmt::{Debug, Formatter};
use crate::common::sourcemap::SourceMap;
use crate::compiler::CompileMessageType;
use crate::lexer::token::Token;

pub struct CompileMessage {
    pub ty: CompileMessageType,
    pub source: SourceMap,
    pub message: String,
    pub notes: Vec<CompileMessage>,
}



impl Debug for CompileMessage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Compile message at {:?}:\n", self.source)?;
        write!(f, "{}\n", self.message)?;

        for note in &self.notes {
            write!(f, "note: {:?}\n", note)?;
        }

        Ok(())
    }
}

impl CompileMessage {
    pub fn new(src: SourceMap, msg: String, ty: CompileMessageType) -> Self {
        Self{
            source: src,
            message: msg,
            notes: Vec::new(),
            ty
        }
    }

    pub fn expected_token_error(src: SourceMap, what: impl AsRef<str>, after: impl AsRef<str>, next_token: Option<Token>) -> Self {
        match next_token {
            Some(tk) => {
                CompileMessage::new(
                    src,
                    format!("expected {} after {}, got token: {:?}", what.as_ref(), after.as_ref(), tk.typ),
                    CompileMessageType::Error
                )
            },
            None => CompileMessage::new(
                src,
                format!("expected {} after {}", what.as_ref(), after.as_ref()),
                CompileMessageType::Error
            )
        }
    }

    pub fn expected_token_in_error(src: SourceMap, what: impl AsRef<str>, in_: impl AsRef<str>, next_token: Option<Token>) -> Self {
        match next_token {
            Some(tk) => {
                CompileMessage::new(
                    src,
                    format!("expected {} in {}, got token: {:?}", what.as_ref(), in_.as_ref(), tk.typ),
                    CompileMessageType::Error
                )
            },
            None => CompileMessage::new(
                src,
                format!("expected {} in {}", what.as_ref(), in_.as_ref()),
                CompileMessageType::Error
            )
        }
    }

    pub fn note(src: SourceMap, msg: String) -> Self {
        Self{
            source: src,
            message: msg,
            notes: Vec::new(),
            ty: CompileMessageType::Info
        }
    }

    pub fn add_note(mut self, msg: CompileMessage) -> Self {
        self.notes.push(msg);

        self
    }
}