





pub enum Progress<U, P, F> {
    Todo(U),
    Partial(P),
    Finished(F),
}

impl<U, P, F> Progress<U, P, F> {
    pub fn is_finished(&self) -> bool {
        match self {
            Progress::Finished(_) => true,
            _ => false,
        }
    }

    pub fn is_partially_finished(&self) -> bool {
        match self {
            Progress::Partial(_) => true,
            _ => false,
        }
    }

    pub fn is_unfinished(&self) -> bool {
        match self {
            Progress::Todo(_) => true,
            _ => true
        }
    }
}

