use std::collections::BTreeMap;

struct Node<T: Ord + Clone> {
    children: BTreeMap<T, Self>,
    is_end: bool,
}

impl<T: Ord + Clone> Node<T> {
    pub fn new(end: bool) -> Self {
        Self{
            children: BTreeMap::new(),
            is_end: end,
        }
    }

    pub fn get_or_insert(&mut self, val: T, end: bool) -> &mut Self {
        let res = self.children.entry(val).or_insert(Node::new(end));
        if end {  // Only set to true, never overwrite true with false
            res.is_end = true;
        }
        res
    }
}

pub struct Trie<T: Ord + Clone> {
    roots: BTreeMap<T, Node<T>>
}

impl<T: Ord + Clone> Trie<T> {
    pub fn new() -> Self {
        Self{ roots: BTreeMap::new() }
    }

    pub fn insert(&mut self, slice: &[T]) {
        if slice.is_empty() {
            return;
        }

        let mut start = self.roots.entry(slice[0].clone()).or_insert(
            Node::new(slice.len() == 1)
        );

        for i in 1..slice.len() {
            start = start.get_or_insert(slice[i].clone(), i+1 == slice.len());
        }
    }

    pub fn contains(&self, slice: &[T]) -> bool {
        let Some(mut start) = self.roots.get(&slice[0]) else {
            return false;
        };

        for i in 1..slice.len() {
            if let Some(n) = start.children.get(&slice[i]) {
                start = n;
            } else {
                return false;
            }
        }

        start.is_end
    }

    pub fn match_prefix_greedy(&self, slice: &T) {

    }
}