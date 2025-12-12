//! The B-tree data structure.

/// A B-tree.
#[derive(Clone, Debug)]
pub struct BTree<T: Ord, const N: usize = 32> {
    root: Option<Node<T, N>>,
}

impl<T: Ord, const N: usize> BTree<T, N> {
    pub fn new() -> Self {
        Self { root: None }
    }

    pub fn get(&self, value: &T) -> Option<&T> {
        if let Some(node) = &self.root {
            node.get(value)
        } else {
            None
        }
    }

    // pub fn insert(&mut self, key: K, value: V) {
    //     if self.root.is_none() {
    //         let mut root = Node::new(true);
    //         root.keys.push(key);
    //         root.cells.push(value);
    //         self.root = Some(root);
    //         return;
    //     }
    //
    //     if self.root.as_ref().unwrap().is_full(self.t) {
    //         let mut new_root = Node::new(false);
    //         let old_root = self.root.take().unwrap();
    //         new_root.children.push(old_root);
    //         new_root.split_child(0, self.t);
    //         new_root.insert_non_full(key, value, self.t);
    //         self.root = Some(new_root);
    //     } else {
    //         self.root
    //             .as_mut()
    //             .unwrap()
    //             .insert_non_full(key, value, self.t);
    //     }
    // }

    pub fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

#[derive(Clone, Debug)]
enum Cell<T: Ord, const N: usize> {
    Node(Node<T, N>),
    Value(T),
}

#[derive(Clone, Debug)]
struct Node<T: Ord, const N: usize> {
    cells: Box<[Option<Cell<T, N>>; N]>,
}

impl<T: Ord, const N: usize> Node<T, N> {
    fn new() -> Self {
        Node {
            cells: [const { None }; N].into(),
        }
    }

    pub fn get(&self, value: &T) -> Option<&T> {
        let mut i = 0;

        while i < self.cells.len() && key > &self.cells[i] {
            i += 1;
        }

        if i < self.keys.len() && key == &self.keys[i] {
            Some(&self.cells[i])
        } else if self.is_leaf {
            None
        } else {
            self.cells[i].get(value)
        }
    }

    // fn insert_non_full(&mut self, key: K, value: V, t: usize) {
    //     let mut i = self.keys.len();
    //
    //     if self.is_leaf {
    //         self.keys.push(key.clone());
    //         self.cells.push(value.clone());
    //         while i > 0 && &key < &self.keys[i - 1] {
    //             self.keys.swap(i, i - 1);
    //             self.cells.swap(i, i - 1);
    //             i -= 1;
    //         }
    //     } else {
    //         while i > 0 && &key < &self.keys[i - 1] {
    //             i -= 1;
    //         }
    //
    //         if self.children[i].is_full(t) {
    //             self.split_child(i, t);
    //             if &key > &self.keys[i] {
    //                 i += 1;
    //             }
    //         }
    //         self.children[i].insert_non_full(key, value, t);
    //     }
    // }
    //
    // fn split_child(&mut self, index: usize, t: usize) {
    //     let mut new_node = Node::new(self.children[index].is_leaf);
    //     let mid = t - 1;
    //
    //     new_node.keys = self.children[index].keys.split_off(mid + 1);
    //     new_node.cells = self.children[index].values.split_off(mid + 1);
    //
    //     if !self.children[index].is_leaf {
    //         new_node.children = self.children[index].children.split_off(mid + 1);
    //     }
    //
    //     let mid_key = self.children[index].keys.pop().unwrap();
    //     let mid_value = self.children[index].values.pop().unwrap();
    //
    //     self.keys.insert(index, mid_key);
    //     self.cells.insert(index, mid_value);
    //     self.children.insert(index + 1, new_node);
    // }
}
