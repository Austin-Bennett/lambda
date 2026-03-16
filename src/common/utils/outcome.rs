use std::fmt::Debug;
use std::ops::{ControlFlow, Deref, DerefMut, DerefPure, FromResidual, Try};
use std::ops::ControlFlow::{Break, Continue};

//an error type that can represent Some/Ok, None and Err
//like a combination of Option and Result
//the idea is that sometimes you want to return an error, and sometimes you just
//want to return nothing to indicate something else entirely besides an error
pub enum Outcome<T, E> {
    Ok(T),
    None,
    Err(E),
}

//this structure is meant to act as a helper
//for cases where you want to return the None value if it returns None
pub struct OnlyOkOutcome<T, E> {
    oc: Outcome<T, E>
}

impl<T, E> Deref for OnlyOkOutcome<T, E> {
    type Target = Outcome<T, E>;

    fn deref(&self) -> &Self::Target {
        &self.oc
    }
}

impl<T, E> DerefMut for OnlyOkOutcome<T, E> {

    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.oc
    }
}

unsafe impl<T, E> DerefPure for OnlyOkOutcome<T, E> {}

impl<T, E> Outcome<T, E> {
    pub fn unwrap_some(self) -> T where E: Debug {
        match self {
            Outcome::Ok(v) => v,
            Outcome::None => panic!("unwrap called on a None value"),
            Outcome::Err(e) => panic!("unwrap called on a Err value: {:?}", e)
        }
    }

    pub fn unwrap_some_or(self, or: T) -> T {
        match self {
            Outcome::Ok(v) => v,
            Outcome::None => or,
            Outcome::Err(e) => or
        }
    }

    pub fn unwrap_err(self) -> E where T: Debug {
        match self {
            Outcome::Ok(v) => panic!("unwrap_err called on Ok value: {:?}", v),
            Outcome::None => panic!("unwrap_err called on a None value"),
            Outcome::Err(e) => e,
        }
    }

    pub fn unwrap_err_or(self, or: E) -> E {
        match self {
            Outcome::Ok(v) => or,
            Outcome::None => or,
            Outcome::Err(e) => e,
        }
    }



    //discards the err value
    pub fn ok(self) -> Option<T> {
        match self {
            Outcome::Ok(v) => Some(v),
            Outcome::None => None,
            Outcome::Err(_) => None,
        }
    }


    pub fn ok_or(self, or: E) -> Result<T, E> {
        match self {
            Outcome::Ok(v) => Ok(v),
            Outcome::Err(e) => Err(e),
            Outcome::None => Err(or),
        }
    }

    pub fn only_ok(self) -> OnlyOkOutcome<T, E> {
        OnlyOkOutcome{
            oc: self
        }
    }
}

impl<T, E> From<Option<T>> for Outcome<T, E> {
    fn from(value: Option<T>) -> Self {
        match value {
            Some(v) => Self::Ok(v),
            None => Self::None
        }
    }
}

impl<T, E> From<Result<T, E>> for Outcome<T, E> {
    fn from(value: Result<T, E>) -> Self {
        match value {
            Ok(v) => Self::Ok(v),
            Err(e) => Self::Err(e)
        }
    }
}



impl<T, E> FromResidual for Outcome<T, E> {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        Self::Err(residual)
    }
}

impl<T, E> Try for Outcome<T, E> {
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
            Outcome::Ok(v) => {
                ControlFlow::Continue(Some(v))
            }
            Outcome::None => {
                ControlFlow::Continue(None)
            }
            Outcome::Err(e) => {
                Break(e)
            }
        }
    }
}

impl<T, E> FromResidual for OnlyOkOutcome<T, E> {
    fn from_residual(residual: <Self as Try>::Residual) -> Self {
        Self{
            oc: match residual {
            None => Outcome::None,
            Some(v) => Outcome::Err(v)
        }
    }
    }
}

impl<T, E> Try for OnlyOkOutcome<T, E> {
    type Output = T;
    type Residual = Option<E>;

    fn from_output(output: Self::Output) -> Self {
        Self{
            oc: Outcome::Ok(output)
        }
    }

    fn branch(self) -> ControlFlow<Self::Residual, Self::Output> {
        match self.oc {
            Outcome::Ok(v) => {
                ControlFlow::Continue(v)
            }
            Outcome::None => {
                ControlFlow::Break(None)
            }
            Outcome::Err(e) => {
                ControlFlow::Break(Some(e))
            }
        }
    }
}