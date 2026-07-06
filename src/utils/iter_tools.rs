

pub struct LambdaIterator<I: Iterator> {
    iterator: I,
    peek: Option<I::Item>,
}

impl<I: Iterator> Iterator for LambdaIterator<I> {
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(val) = self.peek.take() {
            Some(val)
        } else {
            self.iterator.next()
        }
    }
}

impl<I: Iterator> LambdaIterator<I> {

    fn update_peek_val(&mut self) {
        if self.peek.is_none() {
            self.peek = self.iterator.next();
        }
    }
    pub fn peek(&mut self) -> Option<&I::Item> {
        self.update_peek_val();

        self.peek.as_ref()
    }

    pub fn peek_mut(&mut self) -> Option<&mut I::Item> {
        self.update_peek_val();
        self.peek.as_mut()
    }

    pub fn next_if<F>(&mut self, mut predicate: F) -> Option<I::Item> where F: FnMut(&I::Item) -> bool {
        let res = self.next()?;
        
        if predicate(&res) {
            Some(res)
        } else {
            self.peek = Some(res);
            None
        }
    }
    
    pub fn peek_if<F>(&mut self, mut predicate: F) -> Option<&I::Item> where F: FnMut(&I::Item) -> bool {
        self.update_peek_val();
        let res = self.peek.as_ref()?;
        
        if predicate(res) {
            Some(res)
        } else {
            None
        }
    }
}

impl<I: Iterator<Item: PartialEq>> LambdaIterator<I> {
    pub fn next_if_eq(&mut self, other: &I::Item) -> Option<I::Item> {
        let res = self.next()?;

        if res.eq(other) {
            Some(res)
        } else {
            self.peek = Some(res);
            None
        }
    }

    pub fn peek_if_eq(&mut self, other: &I::Item) -> Option<&I::Item> {
        self.update_peek_val();
        let res = self.peek.as_ref()?;

        if res.eq(other) {
            Some(res)
        } else {
            None
        }
    }
}



pub trait ToLambdaIterator {
    fn lmb_iter(self) -> LambdaIterator<Self> where Self: Sized + Iterator;
}

impl<I: Iterator> ToLambdaIterator for I {
    fn lmb_iter(self) -> LambdaIterator<Self>
    where
        Self: Sized + Iterator,
    {
        LambdaIterator{
            iterator: self,
            peek: None
        }
    }
}

impl<I: Iterator> From<I> for LambdaIterator<I> {
    fn from(value: I) -> Self {
        LambdaIterator{
            iterator: value,
            peek: None
        }
    }
}