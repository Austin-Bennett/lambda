use std::alloc::{alloc, Layout};
use std::ops::{Deref, DerefMut};
use std::ptr;

pub struct RuntimeStatic<T>(*mut T);



impl<T> RuntimeStatic<T> {
    pub fn new(val: T) -> Self {
        let ptr = unsafe{ alloc(Layout::new::<T>()) } as *mut T;
        unsafe{ ptr::write(ptr, val) }
        Self( ptr )
    }
    
    pub fn static_ref(this: &Self) -> &'static T {
        unsafe{ &* this.0 }
    }
    
    pub fn static_mut(this: &mut Self) -> &'static mut T {
        unsafe{ &mut * this.0 }
    }
}

impl<T> Deref for RuntimeStatic<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe{ &*self.0 }
    }
}

impl<T> DerefMut for RuntimeStatic<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe{ &mut *self.0 }
    }
}