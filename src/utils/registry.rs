use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};

pub type RegistryID = u64;
pub struct Registry<I> {
    items: Vec<I>
}



impl<I> Registry<I> {
    pub fn new() -> Self {
        Self{
            items: Vec::new()
        }
    }

    pub fn register(&mut self, item: I) -> RegistryID {
        let id = self.items.len() as u64;

        self.items.push(item);

        id
    }

}

pub trait RegistryFunctions {
    type Item;

    fn get_by_id(&self, id: RegistryID) -> Option<&Self::Item>;
    fn get_by_id_mut(&mut self, id: RegistryID) -> Option<&mut Self::Item>;
}

impl<I> RegistryFunctions for Registry<I> {
    type Item = I;

    fn get_by_id(&self, id: RegistryID) -> Option<&Self::Item> {
        if id >= self.items.len() as u64 {
            None
        } else {
            Some(&self.items[id as usize])
        }
    }

    fn get_by_id_mut(&mut self, id: RegistryID) -> Option<&mut Self::Item> {
        if id >= self.items.len() as u64 {
            None
        } else {
            Some(&mut self.items[id as usize])
        }
    }
}

pub struct LookupRegistry<I: Hash + Clone + Eq> {
    registry: Registry<I>,
    lookup: HashMap<I, u64>
}

impl<I: Hash + Clone + Eq> LookupRegistry<I> {
    pub fn new() -> Self {
        Self{
            registry: Registry::new(),
            lookup: HashMap::new(),
        }
    }

    pub fn register(&mut self, item: I) -> RegistryID {

        let hashed_copy = item.clone();
        let id = self.registry.register(item);

        self.lookup.insert(hashed_copy, id);

        id
    }
    
    pub fn lookup_id(&self, item: &I) -> Option<u64> {
        self.lookup.get(item).copied()
    }
}

impl<I: Hash + Clone + Eq + 'static> Deref for LookupRegistry<I> {
    type Target = dyn RegistryFunctions<Item=I>;

    fn deref(&self) -> &Self::Target {
        & self.registry
    }
}

impl<I: Hash + Clone + Eq + 'static> DerefMut for LookupRegistry<I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.registry
    }
}