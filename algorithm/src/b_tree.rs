//! The B-tree data structure.

mod node;

use self::node::Node;
use core::fmt::Debug;
use core::mem::take;

/// A B-tree.
#[derive(Clone, Debug, Default)]
pub struct BTree<T, const N: usize = 32> {
    root: Option<Node<T, N>>,
}

impl<T: Debug + Ord, const N: usize> BTree<T, N> {
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

        for x in 0..196 {
            assert_eq!(tree.get(&x), None);

            tree.insert(x);

            for y in 0..x + 1 {
                assert_eq!(tree.get(&y), Some(&y), "x = {x}, y = {y}, tree = {tree:?}");
            }
        }
    }
}
