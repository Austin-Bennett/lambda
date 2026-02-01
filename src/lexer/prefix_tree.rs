use std::collections::BTreeMap;




struct Node<K: Ord + Clone, V> {
    children: BTreeMap<K, Self>,
    val: Option<V>
}

impl<K: Ord + Clone, V> Node<K, V> {
    pub fn new(val: Option<V>) -> Self {
        Self{
            children: BTreeMap::new(),
            val,
        }
    }

    pub fn empty() -> Self {
        Self::new(None)
    }

    pub fn get_or_insert(&mut self, key: K, val: Option<V>) -> &mut Self {
        if let Some(k) = self.children.get_mut(&key) {
            let k: *mut Node<K, V> = k as *mut _;
            if let Some(v) = val {
                unsafe{ (*k).val = Some(v); }
            }
            return unsafe{ &mut *k };
        }


        self.children.entry(key).or_insert(Node::new(val))
    }
}

pub struct Trie<K: Ord + Clone, V> {
    roots: BTreeMap<K, Node<K, V>>
}

impl<K: Ord + Clone, V> Trie<K, V> {
    pub fn new() -> Self {
        Self{ roots: BTreeMap::new() }
    }

    pub fn insert(&mut self, slice: &[K], val: V) {
        if slice.is_empty() {
            return;
        }

        

    }

    pub fn contains(&self, slice: &[K]) -> bool {
        if slice.is_empty() { return false; }
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

    pub fn match_prefix_greedy(&self, slice: &[K]) -> usize {
        if slice.is_empty() {
            return 0;
        }

        let Some(mut node) = self.roots.get(&slice[0]) else {
            return 0;
        };

        let mut last_match = if node.is_end { 1 } else { 0 };

        for i in 1..slice.len() {
            if let Some(next) = node.children.get(&slice[i]) {
                node = next;
                if node.is_end {
                    last_match = i + 1;
                }
            } else {
                break;
            }
        }

        last_match
    }
}