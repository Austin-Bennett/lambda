





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
    
    pub fn unwrap_finished(self) -> F {
        match self { 
            Progress::Finished(f) => f,
            _ => panic!("Unwrap finished on a non-finished value"),
        }
    }

    pub fn unwrap_partially_finished(self) -> P {
        match self {
            Progress::Partial(p) => p,
            _ => panic!("Unwrap partial on a non-partial value"),
        }
    }

    pub fn unwrap_unfinished(self) -> U {
        match self {
            Progress::Todo(u) => u,
            _ => panic!("Unwrap todo on a non-todo value"),
        }
    }


    pub fn unwrap_finished_ref(&self) -> &F {
        match self {
            Progress::Finished(f) => f,
            _ => panic!("Unwrap finished on a non-finished value"),
        }
    }

    pub fn unwrap_partially_finished_ref(&self) -> &P {
        match self {
            Progress::Partial(p) => p,
            _ => panic!("Unwrap partial on a non-partial value"),
        }
    }

    pub fn unwrap_unfinished_ref(&self) -> &U {
        match self {
            Progress::Todo(u) => u,
            _ => panic!("Unwrap todo on a non-todo value"),
        }
    }
}



