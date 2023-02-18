use std::collections::HashMap;

pub enum SearchMode {
    Exact,
    Prefix,
}

#[derive(Clone, Debug)]
struct TrieNode<K, V>
where
    K: Clone + Eq + std::hash::Hash + std::fmt::Debug,
    V: Clone + std::fmt::Debug,
{
    children: HashMap<K, TrieNode<K, V>>,
    value: Option<V>,
}

impl<K, V> TrieNode<K, V>
where
    K: Clone + Eq + std::hash::Hash + std::fmt::Debug,
    V: Clone + std::fmt::Debug,
{
    fn new() -> Self {
        Self {
            children: HashMap::new(),
            value: None,
        }
    }

    fn insert(&mut self, key: K, child: Self) -> &mut Self {
        self.children.entry(key).or_insert(child)
    }

    fn search_child(&self, path: &[K]) -> Option<Self> {
        let mut curr = self;

        for p in path {
            match curr.children.get(p) {
                None => {
                    return None;
                }
                Some(next) => {
                    curr = next;
                }
            }
        }

        Some(curr.to_owned())
    }

    fn search(&self, mode: SearchMode, path: &[K]) -> bool {
        let child = self.search_child(path);

        match mode {
            SearchMode::Prefix => child.is_some(),
            SearchMode::Exact => match child {
                None => false,
                Some(node) => node.value.is_some(),
            },
        }
    }

    fn collect_children(node: &Self, results: &mut Vec<Self>) {
        results.push(node.clone());
        for (_, child) in node.children.iter() {
            Self::collect_children(child, results);
        }
    }

    fn predict(&self) -> Vec<V> {
        let children = &mut Vec::new();
        Self::collect_children(self, children);

        children
            .iter()
            .filter_map(|child| child.value.clone())
            .collect()
    }
}

#[derive(Clone, Debug)]
pub struct Trie<K, V>
where
    K: Clone + Eq + std::hash::Hash + std::fmt::Debug,
    V: Clone + std::fmt::Debug,
{
    root: TrieNode<K, V>,
}

impl<K, V> Trie<K, V>
where
    K: Clone + Eq + std::hash::Hash + std::fmt::Debug,
    V: Clone + std::fmt::Debug,
{
    pub fn new() -> Self {
        Self {
            root: TrieNode::new(),
        }
    }

    pub fn insert(&mut self, path: &[K], value: V) {
        let mut curr = &mut self.root;

        for p in path {
            let next = curr.insert(p.clone(), TrieNode::new());
            curr = next;
        }

        curr.value = Some(value);
    }

    pub fn search(&self, mode: SearchMode, path: &[K]) -> bool {
        self.root.search(mode, path)
    }

    pub fn predict(&self, path: &[K]) -> Option<Vec<V>> {
        match self.root.search_child(path) {
            Some(node) => Some(node.predict()),
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_trie() {
        use super::*;
        let mut trie: Trie<u8, &str> = Trie::new();

        trie.insert(b"foo", "foo");
        trie.insert(b"foobar", "foobar");
        trie.insert(b"foobar2000", "foobar2000");

        assert!(trie.search(SearchMode::Prefix, b"f"));
        assert!(trie.search(SearchMode::Prefix, b"fo"));
        assert!(trie.search(SearchMode::Prefix, b"foo"));
        assert!(trie.search(SearchMode::Prefix, b"foob"));
        assert!(trie.search(SearchMode::Prefix, b"fooba"));
        assert!(trie.search(SearchMode::Prefix, b"foobar"));

        assert_eq!(trie.search(SearchMode::Prefix, b"f"), true);
        assert_eq!(trie.search(SearchMode::Prefix, b"fo"), true);
        assert_eq!(trie.search(SearchMode::Prefix, b"fa"), false);
        assert_eq!(trie.search(SearchMode::Prefix, b"bar"), false);
        assert_eq!(trie.search(SearchMode::Prefix, b"ob"), false);
        assert_eq!(trie.search(SearchMode::Prefix, b"foooba"), false);

        assert_eq!(trie.search(SearchMode::Exact, b"f"), false);
        assert_eq!(trie.search(SearchMode::Exact, b"fo"), false);
        assert_eq!(trie.search(SearchMode::Exact, b"foo"), true);
        assert_eq!(trie.search(SearchMode::Exact, b"foob"), false);
        assert_eq!(trie.search(SearchMode::Exact, b"fooba"), false);
        assert_eq!(trie.search(SearchMode::Exact, b"foobar"), true);

        // Not working yet
        let kids = trie.root.predict();
        println!("kids {:?}, len {}, kids_long: {kids:?}", kids, kids.len());

        let f_node = trie.root.search_child(b"f");
        assert_eq!(f_node.expect("foob node is None").predict().len(), 3);

        let foob_node = trie.root.search_child(b"foob");
        assert_eq!(foob_node.expect("foob node is None").predict().len(), 2);
    }
}
