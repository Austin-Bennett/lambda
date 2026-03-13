use std::fmt::Debug;
use std::hash::Hash;
use crate::common::utils::hashtable::ArrayHashTable;

pub struct TrieNode<E: Hash + PartialEq + Debug, const N: usize> {
    elem: E,
    is_end: bool,
    children: ArrayHashTable<E, TrieNode<E, N>, N>,
}

pub struct Trie<E: Hash + PartialEq + Debug, const N: usize = 100> {
    root: ArrayHashTable<E, TrieNode<E, N>, N>
}

impl<E: Hash + PartialEq + Debug, const N: usize> Trie<E, N> {

    pub fn new() -> Self {

    }

}