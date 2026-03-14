use std::array::from_fn;
use std::fmt::Debug;
use std::hash::{DefaultHasher, Hash, Hasher};
use std::ops::{Index, IndexMut};

#[derive(Clone)]
struct HTNode<K: Clone, E: Clone> {
    key: K,
    elem: E,
    next: Option<Box<HTNode<K, E>>>
}

impl<K: Clone, E: Clone> HTNode<K, E> {
    pub fn new(key: K, elem: E) -> Self {
        Self{
            key,
            elem,
            next: None,
        }
    }
}

//a simple hashtable that uses a fixed-size array
#[derive(Clone)]
pub struct ArrayHashTable<K: Hash + PartialEq + Debug + Clone, E: Clone, const N: usize = 100> {
    elems: [Option<Box<HTNode<K, E>>>; N]
}

impl<K: Hash + PartialEq + Debug + Clone, E: Clone, const N: usize> ArrayHashTable<K, E, N> {
    pub fn new() -> Self {
        Self{
            elems: from_fn(|_| None),
        }
    }

    pub fn insert(&mut self, key: K, v: E) {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let idx = hasher.finish() as usize % N;

        let mut node = &mut self.elems[idx];

        while let Some(n) = node {
            if n.key == key {
                n.elem = v;
                return;
            }
            node = &mut n.next;
        }

        *node = Some(Box::new(HTNode::new(key, v)));
    }

    pub fn contains(&mut self, key: impl AsRef<K>) -> bool {
        let mut hasher = DefaultHasher::new();
        let r = key.as_ref();
        r.hash(&mut hasher);
        let idx = hasher.finish() as usize % N;

        let mut node = &self.elems[idx];

        while let Some(n) = node {
            if n.key == *r {
                return true;
            }
            node = &n.next;
        }

        false
    }

    pub fn get(&self, key: impl AsRef<K>) -> Option<&E> {
        let mut hasher = DefaultHasher::new();
        let r = key.as_ref();
        r.hash(&mut hasher);
        let idx = hasher.finish() as usize % N;

        let mut node = &self.elems[idx];

        while let Some(n) = node {
            if n.key == *r {
                return Some(&n.elem);
            }
            node = &n.next;
        }

        None
    }



    pub fn get_mut(&mut self, key: impl AsRef<K>) -> Option<&mut E> {
        let mut hasher = DefaultHasher::new();
        let r = key.as_ref();
        r.hash(&mut hasher);
        let idx = hasher.finish() as usize % N;

        let mut node = &mut self.elems[idx];

        while let Some(n) = node {
            if n.key == *r {
                return Some(&mut n.elem);
            }
            node = &mut n.next;
        }

        None
    }


}

impl<K: Hash + PartialEq + Debug + Clone, KRef: AsRef<K>, E: Clone, const N: usize> Index<KRef> for ArrayHashTable<K, E, N> {
    type Output = E;

    fn index(&self, index: KRef) -> &Self::Output {
        if let Some(v) = self.get(&index) {
            v
        } else {
            panic!("No key \"{:?}\" found in hash table", index.as_ref())
        }
    }
}

impl<K: Hash + PartialEq + Debug + Clone, KRef: AsRef<K>, E: Clone, const N: usize> IndexMut<KRef> for ArrayHashTable<K, E, N> {
    fn index_mut(&mut self, index: KRef) -> &mut Self::Output {

        if let Some(v) = self.get_mut(&index) {
            v
        } else {
            panic!("No key \"{:?}\" found in hash table", index.as_ref());
        }
    }
}