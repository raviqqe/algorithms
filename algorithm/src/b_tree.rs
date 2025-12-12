//! The B-tree data structure.

/// A B-tree.
#[derive(Clone, Debug)]
pub struct BTree<T: Ord, const N: usize = 32> {
    root: Option<Node<V, N>>,
}

#[derive(Clone, Debug)]
enum Cell<T: Ord, const N: usize> {
    Value(T),
    Node(Node<T, N>),
}

#[derive(Clone, Debug)]
struct Node<T: Ord, const N: usize> {
    keys: Vec<T>,
    cells: Vec<Cell<T, N>>,
}

impl<K: Ord + Clone, V: Clone> Node<K, V> {
    fn new() -> Self {
        Node {
            keys: vec![],
            cells: vec![],
        }
    }

    fn is_full(&self, t: usize) -> bool {
        self.keys.len() >= 2 * t - 1
    }

    fn search(&self, key: &K) -> Option<&V> {
        let mut i = 0;
        while i < self.keys.len() && key > &self.keys[i] {
            i += 1;
        }

        if i < self.keys.len() && key == &self.keys[i] {
            Some(&self.cells[i])
        } else if self.is_leaf {
            None
        } else {
            self.children[i].search(key)
        }
    }

    fn insert_non_full(&mut self, key: K, value: V, t: usize) {
        let mut i = self.keys.len();

        if self.is_leaf {
            self.keys.push(key.clone());
            self.cells.push(value.clone());
            while i > 0 && &key < &self.keys[i - 1] {
                self.keys.swap(i, i - 1);
                self.cells.swap(i, i - 1);
                i -= 1;
            }
        } else {
            while i > 0 && &key < &self.keys[i - 1] {
                i -= 1;
            }

            if self.children[i].is_full(t) {
                self.split_child(i, t);
                if &key > &self.keys[i] {
                    i += 1;
                }
            }
            self.children[i].insert_non_full(key, value, t);
        }
    }

    fn split_child(&mut self, index: usize, t: usize) {
        let mut new_node = Node::new(self.children[index].is_leaf);
        let mid = t - 1;

        new_node.keys = self.children[index].keys.split_off(mid + 1);
        new_node.cells = self.children[index].values.split_off(mid + 1);

        if !self.children[index].is_leaf {
            new_node.children = self.children[index].children.split_off(mid + 1);
        }

        let mid_key = self.children[index].keys.pop().unwrap();
        let mid_value = self.children[index].values.pop().unwrap();

        self.keys.insert(index, mid_key);
        self.cells.insert(index, mid_value);
        self.children.insert(index + 1, new_node);
    }
}

impl<K: Ord + Clone, V: Clone> BTree<K, V> {
    /// Creates a new B-tree with minimum degree t.
    /// Each node will have at least t-1 keys (except root) and at most 2t-1 keys.
    pub fn new(t: usize) -> Self {
        assert!(t >= 2, "Minimum degree must be at least 2");
        BTree { root: None, t }
    }

    /// Searches for a key in the B-tree and returns a reference to its value if found.
    pub fn search(&self, key: &K) -> Option<&V> {
        self.root.as_ref().and_then(|root| root.search(key))
    }

    /// Inserts a key-value pair into the B-tree.
    pub fn insert(&mut self, key: K, value: V) {
        if self.root.is_none() {
            let mut root = Node::new(true);
            root.keys.push(key);
            root.cells.push(value);
            self.root = Some(root);
            return;
        }

        if self.root.as_ref().unwrap().is_full(self.t) {
            let mut new_root = Node::new(false);
            let old_root = self.root.take().unwrap();
            new_root.children.push(old_root);
            new_root.split_child(0, self.t);
            new_root.insert_non_full(key, value, self.t);
            self.root = Some(new_root);
        } else {
            self.root
                .as_mut()
                .unwrap()
                .insert_non_full(key, value, self.t);
        }
    }

    /// Returns true if the B-tree is empty.
    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}
