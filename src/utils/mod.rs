use std::cmp::Ordering;

pub mod macros;
pub mod iter_tools;
pub mod registry;

pub trait CtrlFlowHelpers<V> {
    fn visit<F>(&self, visiter: F) where F: FnMut(&V);
    fn visit_mut<F>(&mut self, visiter: F) where F: FnMut(&mut V);
}

impl<V> CtrlFlowHelpers<V> for Option<V> {
    fn visit<F>(&self, mut visiter: F)
    where
        F: FnMut(&V)
    {
        if let Some(v) = self {
            visiter(v);
        }
    }

    fn visit_mut<F>(&mut self, mut visiter: F)
    where
        F: FnMut(&mut V)
    {
        if let Some(v) = self {
            visiter(v);
        }
    }
}

impl<V, E> CtrlFlowHelpers<V> for Result<V, E> {
    fn visit<F>(&self, mut visiter: F)
    where
        F: FnMut(&V)
    {
        if let Ok(v) = self {
            visiter(v);
        }
    }

    fn visit_mut<F>(&mut self, mut visiter: F)
    where
        F: FnMut(&mut V)
    {
        if let Ok(v) = self {
            visiter(v);
        }
    }
}

