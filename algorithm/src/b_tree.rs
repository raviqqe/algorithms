//! The B-tree data structure.

mod node;

use self::node::Node;
use core::fmt::Debug;

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

    /// Returns an element.
    pub fn get(&self, value: &T) -> Option<&T> {
        self.root.as_ref().and_then(|node| node.get(value))
    }

    /// Inserts an element.
    pub fn insert(&mut self, value: T) {
        self.root = Some(if let Some(mut node) = self.root.take() {
            if let Some((value, split_node)) = node.insert(value) {
                Node::new(vec![node, split_node], vec![value])
            } else {
                node
            }
        } else {
            Node::new(vec![], vec![value])
        });

        #[cfg(test)]
        self.validate();
    }

    /// Removes an element.
    pub fn remove(&mut self, value: &T) {
        if let Some(node) = &mut self.root {
            node.remove(value);
            node.flatten();
        }

        #[cfg(test)]
        self.validate();
    }

    /// Returns `true` if a tree is empty, or `false` otherwise.
    pub const fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    #[cfg(test)]
    fn validate(&self) {
        if let Some(node) = &self.root {
            node.validate();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    const MAX_ITERATIONS: usize = 1 << 10;

    #[test]
    fn new() {
        BTree::<(), 0>::new();
    }

    macro_rules! test_insert_and_delete {
        ($name:ident, $degree:expr) => {
            proptest! {
                #[test]
                fn $name(xs: Vec<(bool, usize)>) {
                    let mut tree = BTree::<usize, $degree>::new();

                    for (remove, x) in xs {
                        assert_eq!(tree.get(&x), None);

                        if remove {
                            tree.remove(&x);

                            assert_eq!(tree.get(&x), None);
                        } else{
                            tree.insert(x);

                            assert_eq!(tree.get(&x), Some(&x));
                        }
                    }
                }
            }
        };
    }

    test_insert_and_delete!(insert_and_delete_random_with_even_degree, 4);
    test_insert_and_delete!(insert_and_delete_random_with_odd_degree, 5);

    mod insert {
        use super::*;
        use pretty_assertions::assert_eq;

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
                    assert_eq!(tree.get(&y), Some(&y));
                }
            }
        }

        #[test]
        fn insert_after_degree_reversely() {
            const DEGREE: usize = 8;

            let mut tree = BTree::<usize, DEGREE>::new();

            for x in (0..MAX_ITERATIONS).rev() {
                assert_eq!(tree.get(&x), None);

                tree.insert(x);

                for y in x..MAX_ITERATIONS {
                    assert_eq!(tree.get(&y), Some(&y));
                }
            }
        }

        macro_rules! test_insert {
            ($name:ident, $degree:expr) => {
                proptest! {
                    #[test]
                    fn $name(xs: Vec<usize>) {
                        let mut tree = BTree::<usize, $degree>::new();

                        for (index, x) in xs.iter().copied().enumerate() {
                            assert_eq!(tree.get(&x), None);

                            tree.insert(x);

                            for y in xs.iter().copied().take(index + 1) {
                                assert_eq!(tree.get(&y), Some(&y));
                            }
                        }
                    }
                }
            };
        }

        test_insert!(insert_random_with_even_degree, 4);
        test_insert!(insert_random_with_odd_degree, 5);
    }

    mod remove {
        use super::*;
        use pretty_assertions::assert_eq;

        const DEGREE: usize = 8;

        #[test]
        fn remove_before_degree() {
            let mut tree = BTree::<usize, DEGREE>::new();

            for x in 0..DEGREE - 1 {
                tree.insert(x);
            }

            for x in 0..DEGREE - 1 {
                assert_eq!(tree.get(&x), Some(&x));

                tree.remove(&x);

                for y in 0..x + 1 {
                    assert_eq!(tree.get(&y), None);
                }

                for y in x + 1..DEGREE - 1 {
                    assert_eq!(tree.get(&y), Some(&y));
                }
            }
        }

        #[test]
        fn remove_before_degree_reversely() {
            let mut tree = BTree::<usize, DEGREE>::new();

            for x in 0..DEGREE - 1 {
                tree.insert(x);
            }

            for x in (0..DEGREE - 1).rev() {
                assert_eq!(tree.get(&x), Some(&x));

                tree.remove(&x);

                for y in 0..x {
                    assert_eq!(tree.get(&y), Some(&y));
                }

                for y in x..DEGREE - 1 {
                    assert_eq!(tree.get(&y), None);
                }
            }
        }

        #[test]
        fn remove_after_degree() {
            let mut tree = BTree::<usize, DEGREE>::new();

            for x in 0..MAX_ITERATIONS {
                tree.insert(x);
            }

            for x in 0..MAX_ITERATIONS {
                assert_eq!(tree.get(&x), Some(&x));

                tree.remove(&x);

                for y in 0..x + 1 {
                    assert_eq!(tree.get(&y), None);
                }

                for y in x + 1..MAX_ITERATIONS {
                    assert_eq!(tree.get(&y), Some(&y));
                }
            }
        }

        #[test]
        fn remove_after_degree_reversely() {
            let mut tree = BTree::<usize, DEGREE>::new();

            for x in 0..MAX_ITERATIONS {
                tree.insert(x);
            }

            for x in (0..MAX_ITERATIONS).rev() {
                assert_eq!(tree.get(&x), Some(&x));

                tree.remove(&x);

                for y in 0..x {
                    assert_eq!(tree.get(&y), Some(&y));
                }

                for y in x..MAX_ITERATIONS {
                    assert_eq!(tree.get(&y), None);
                }
            }
        }

        macro_rules! test_remove {
            ($name:ident, $degree:expr) => {
                proptest! {
                    #[test]
                    fn $name(xs: Vec<usize>, ys: Vec<usize>) {
                        let mut tree = BTree::<usize, $degree>::new();

                        for x in xs.iter().copied() {
                            tree.insert(x);
                        }

                        for y in ys {
                            tree.remove(&y);

                            assert_eq!(tree.get(&y), None);
                        }
                    }
                }
            };
        }

        test_remove!(remove_random_with_even_degree, 4);
        test_remove!(remove_random_with_odd_degree, 5);
    }
}
