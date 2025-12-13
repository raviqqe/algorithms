//! The B-tree data structure.

mod node;

use self::node::Node;
use core::{fmt::Debug, mem::take};

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
                self.root = Some(Node::new(vec![take(node), split_node], vec![value]));
            }
        } else {
            self.root = Some(Node::new(vec![], vec![value]));
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
    use proptest::prelude::*;

    const MAX_ITERATIONS: usize = 1 << 10;

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
    fn insert_before_degree_reversely() {
        const DEGREE: usize = 8;

        let mut tree = BTree::<usize, DEGREE>::new();
        let xs = (0..DEGREE - 1).rev().collect::<Vec<_>>();

        for (index, x) in xs.iter().copied().enumerate() {
            assert_eq!(tree.get(&x), None);

            tree.insert(x);

            for &y in &xs[..index + 1] {
                assert_eq!(tree.get(&y), Some(&y));
            }
        }
    }

    #[test]
    fn insert_after_degree() {
        const DEGREE: usize = 8;

        let mut tree = BTree::<usize, DEGREE>::new();

        for x in 0..MAX_ITERATIONS {
            assert_eq!(tree.get(&x), None);

            tree.insert(x);

            for y in 0..x + 1 {
                assert_eq!(tree.get(&y), Some(&y), "x = {x}, y = {y}, tree = {tree:#?}");
            }
        }
    }

    proptest! {
        #[test]
        fn insert_random_with_even_degree(xs: Vec<usize>) {
            const DEGREE: usize = 4;

            let mut tree = BTree::<usize, DEGREE>::new();

            for (index, x) in xs.iter().copied().enumerate() {
                prop_assert_eq!(tree.get(&x), None);

                tree.insert(x);

                for y in xs.iter().copied().take(index + 1) {
                    prop_assert_eq!(tree.get(&y), Some(&y));
                }
            }
        }

        #[test]
        fn insert_random_with_odd_degree(xs: Vec<usize>) {
            const DEGREE: usize = 5;

            let mut tree = BTree::<usize, DEGREE>::new();

            for (index, x) in xs.iter().copied().enumerate() {
                prop_assert_eq!(tree.get(&x), None);

                tree.insert(x);

                for y in xs.iter().copied().take(index + 1) {
                    prop_assert_eq!(tree.get(&y), Some(&y));
                }
            }
        }
    }
}
