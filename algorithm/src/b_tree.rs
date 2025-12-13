//! The B-tree data structure.

use core::mem::take;

/// A B-tree.
#[derive(Clone, Debug, Default)]
pub struct BTree<T, const N: usize = 32> {
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
        if let Some(node) = &mut self.root {
            if let Some((value, split_node)) = node.insert(value) {
                self.root = Some(Node::new_split(take(node), value, split_node));
            }
        } else {
            self.root = Some(Node::new(value));
        }
    }

    /// Returns `true` if a tree is empty, or `false` otherwise.
    pub const fn is_empty(&self) -> bool {
        self.root.is_none()
    }
}

#[derive(Clone, Debug)]
struct Node<T, const N: usize> {
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

    fn new_split(left: Node<T, N>, value: T, right: Node<T, N>) -> Self {
        Self {
            nodes: vec![left, right],
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

    fn insert(&mut self, value: T) -> Option<(T, Self)> {
        let index = match self.values.binary_search(&value) {
            Ok(index) => {
                self.values[index] = value;
                return None;
            }
            Err(index) => index,
        };

        if self.values.len() < N - 1 {
            self.values.insert(index, value);
            return None;
        }

        self.values.insert(index, value);

        let values = self.values.split_off(N / 2);

        Some((
            self.values.pop().unwrap(),
            Self {
                nodes: vec![],
                values,
            },
        ))
    }

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

impl<T, const N: usize> Default for Node<T, N> {
    fn default() -> Self {
        Self {
            nodes: vec![],
            values: vec![],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn new() {
        BTree::<(), 0>::new();
    }

    #[test]
    fn insert_before_degree() {
        const DEGREE: usize = 8;
        let mut tree = BTree::<usize, DEGREE>::new();

        for x in 0..DEGREE - 1 {
            assert_eq!(tree.get(&x), None);
            tree.insert(x);

            for y in 0..x + 1 {
                assert_eq!(tree.get(&y), Some(&y));
            }
        }
    }

    #[test]
    fn insert_after_degree() {
        const DEGREE: usize = 8;
        let mut tree = BTree::<usize, DEGREE>::new();

        for x in 0..DEGREE - 1 {
            assert_eq!(tree.get(&x), None);
            tree.insert(x);

            for y in 0..x + 1 {
                assert_eq!(tree.get(&y), Some(&y));
            }
        }
    }
}
