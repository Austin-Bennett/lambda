use std::fmt::Debug;
use std::ops::{ControlFlow, FromResidual, Try};
use std::ops::ControlFlow::{Break, Continue};

//an error type that can represent Some/Ok, None and Err
//like a combination of Option and Result
//the idea is that sometimes you want to return an error, and sometimes you just
//want to return nothing to indicate something else entirely besides an error
pub enum TryError<T, E> {
    Ok(T),
    None,
    Err(E),
}

impl<T, E> TryError<T, E> {
    pub fn unwrap_some(self) -> T where E: Debug {
        match self {
            TryError::Ok(v) => v,
            TryError::None => panic!("Unwrap called on a None value"),
            TryError::Err(e) => panic!("Unwrap called on a Err value: {:?}", e)
        }
    }
}

impl<T, E> FromResidual for TryError<T, E> {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        Self::Err(residual)
    }
}

impl<T, E> Try for TryError<T, E> {
    type Output = Option<T>;
    type Residual = E;

    fn from_output(output: Self::Output) -> Self {
        match output {
            Some(v) => Self::Ok(v),
            None => Self::None,
        }
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self {
            TryError::Ok(v) => {
                ControlFlow::Continue(Some(v))
            }
            TryError::None => {
                ControlFlow::Continue(None)
            }
            TryError::Err(e) => {
                Break(e)
            }
        }
    }
}