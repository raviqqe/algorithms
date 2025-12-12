//! The B-tree data structure.

/// A B-tree.
#[derive(Clone, Debug, Default)]
pub struct BTree<T: Ord, const N: usize = 32> {
    root: Option<Node<T, N>>,
}

impl<T: Ord, const N: usize> BTree<T, N> {
    /// Creates a B-tree.
    pub const fn new() -> Self {
        Self { root: None }
    }

    /// Gets an element.
    pub fn get(&self, value: &T) -> Option<&T> {
        self.root.as_ref().and_then(|node| node.get(value))
    }

    /// Inserts an element.
    pub fn insert(&mut self, value: T) {
        if let Some(node) = self.root {
            node.insert(value);
        } else {
            self.root = Some(Node::new(value));
        }
    }

    /// Returns `true` if a tree is empty, or `false` otherwise.
    pub const fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

#[derive(Clone, Debug, Default)]
struct Node<T: Ord, const N: usize> {
    nodes: Vec<Self>,
    values: Vec<T>,
}

impl<T: Ord, const N: usize> Node<T, N> {
    fn new(value: T) -> Self {
        Self {
            nodes: vec![],
            values: vec![value],
        }
    }

    pub fn get(&self, value: &T) -> Option<&T> {
        let index = match self.values.binary_search(value) {
            Ok(index) => return self.values.get(index),
            Err(index) => index,
        };

        self.nodes.get(index).and_then(|node| node.get(value))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        BTree::<(), 0>::new();
    }
}
