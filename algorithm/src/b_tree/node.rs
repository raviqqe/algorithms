use core::fmt::Debug;

macro_rules! assert_invariant {
    ($self:expr) => {
        let nodes = &$self.nodes;
        let values = &$self.values;

        debug_assert!(nodes.is_empty() || nodes.len() == values.len() + 1);
        debug_assert!(values.len() < N);
    };
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node<T, const N: usize> {
    nodes: Vec<Self>,
    values: Vec<T>,
}

impl<T: Debug + Ord, const N: usize> Node<T, N> {
    pub fn new(nodes: Vec<Self>, values: Vec<T>) -> Self {
        let this = Self { nodes, values };

        assert_invariant!(&this);

        this
    }

    pub fn get(&self, value: &T) -> Option<&T> {
        let index = match self.values.binary_search(value) {
            Ok(index) => return self.values.get(index),
            Err(index) => index,
        };

        self.nodes.get(index).and_then(|node| node.get(value))
    }

    pub fn insert(&mut self, value: T) -> Option<(T, Self)> {
        assert_invariant!(self);

        let index = match self.values.binary_search(&value) {
            Ok(index) => {
                self.values[index] = value;
                return None;
            }
            Err(index) => index,
        };

        if !self.nodes.is_empty() {
            return if let Some((value, node)) = self.nodes[index].insert(value) {
                self.nodes.insert(index + 1, node);
                self.values.insert(index, value);

                if self.values.len() < N - 1 {
                    None
                } else {
                    assert_invariant!(self);

                    let index = self.values.len().div_ceil(2);
                    let mut nodes = self.nodes.split_off(index + 1);
                    let values = self.values.split_off(index);
                    let mut left = self.nodes.pop().unwrap();
                    let (value, right) = left.split();

                    self.nodes.push(left);
                    nodes.insert(0, right);

                    debug_assert_eq!(nodes.len(), values.len() + 1);
                    debug_assert!(self.values.iter().all(|element| element < &value));
                    debug_assert!(values.iter().all(|element| element > &value));

                    Some((value, Self::new(nodes, values)))
                }
            } else {
                None
            };
        }

        self.values.insert(index, value);

        if self.values.len() < N {
            None
        } else {
            Some(self.split())
        }
    }

    fn split(&mut self) -> (T, Self) {
        let index = self.values.len() / 2 + 1;
        let nodes = self.nodes.split_off(self.nodes.len().min(index));
        let values = self.values.split_off(index);
        let value = self.values.pop().unwrap();

        assert_invariant!(self);

        (value, Self::new(nodes, values))
    }
}

impl<T, const N: usize> Default for Node<T, N> {
    fn default() -> Self {
        // A temporary node with dummy data.
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
    fn insert_before_degree() {
        const DEGREE: usize = 8;
        let mut node = Node::<usize, DEGREE>::new(vec![], vec![0]);

        for x in 1..DEGREE - 1 {
            assert_eq!(node.get(&x), None);
            node.insert(x);

            for y in 0..x + 1 {
                assert_eq!(node.get(&y), Some(&y));
            }
        }
    }

    #[test]
    fn insert_after_degree_with_even_degree() {
        const DEGREE: usize = 8;
        let mut node = Node::<usize, DEGREE>::new(vec![], vec![0]);

        for x in 1..DEGREE - 1 {
            assert_eq!(node.get(&x), None);

            assert_eq!(node.insert(x), None);
        }

        assert_eq!(
            node.insert(7),
            Some((
                4,
                Node {
                    nodes: vec![],
                    values: vec![5, 6, 7],
                }
            ))
        );
    }

    #[test]
    fn insert_after_degree_with_odd_degree() {
        const DEGREE: usize = 9;
        let mut node = Node::<usize, DEGREE>::new(vec![], vec![0]);

        for x in 1..DEGREE - 1 {
            assert_eq!(node.get(&x), None);
            assert_eq!(node.insert(x), None);
        }

        assert_eq!(
            node.insert(8),
            Some((
                4,
                Node {
                    nodes: vec![],
                    values: vec![5, 6, 7, 8],
                }
            ))
        );
    }

    mod split {
        use super::*;
        use pretty_assertions::assert_eq;

        const DEGREE: usize = 8;

        fn split(
            mut left: Node<usize, DEGREE>,
        ) -> (Node<usize, DEGREE>, usize, Node<usize, DEGREE>) {
            let (value, right) = left.split();

            (left, value, right)
        }

        #[test]
        fn split_value() {
            assert_eq!(
                split(Node::new(vec![], vec![42])),
                (Node::new(vec![], vec![]), 42, Node::new(vec![], vec![]))
            );
        }

        #[test]
        fn split_two_values() {
            assert_eq!(
                split(Node::new(vec![], vec![1, 2])),
                (Node::new(vec![], vec![1]), 2, Node::new(vec![], vec![]))
            );
        }

        #[test]
        fn split_three_values() {
            assert_eq!(
                split(Node::new(vec![], vec![1, 2, 3])),
                (Node::new(vec![], vec![1]), 2, Node::new(vec![], vec![3]))
            );
        }

        #[test]
        fn split_four_values() {
            assert_eq!(
                split(Node::new(vec![], vec![1, 2, 3, 4])),
                (Node::new(vec![], vec![1, 2]), 3, Node::new(vec![], vec![4]))
            );
        }

        mod node {
            use super::*;
            use pretty_assertions::assert_eq;

            fn dummy_node(value: usize) -> Node<usize, DEGREE> {
                Node::new(vec![], vec![value])
            }

            #[test]
            fn split_two_nodes() {
                assert_eq!(
                    split(Node::new(vec![dummy_node(10), dummy_node(20)], vec![1])),
                    (
                        Node::new(vec![dummy_node(10)], vec![]),
                        1,
                        Node::new(vec![dummy_node(20)], vec![])
                    )
                );
            }

            #[test]
            fn split_three_nodes() {
                assert_eq!(
                    split(Node::new(
                        vec![dummy_node(10), dummy_node(20), dummy_node(30)],
                        vec![1, 2]
                    )),
                    (
                        Node::new(vec![dummy_node(10), dummy_node(20)], vec![1]),
                        2,
                        Node::new(vec![dummy_node(30)], vec![])
                    )
                );
            }

            #[test]
            fn split_four_nodes() {
                assert_eq!(
                    split(Node::new(
                        vec![
                            dummy_node(10),
                            dummy_node(20),
                            dummy_node(30),
                            dummy_node(40),
                        ],
                        vec![1, 2, 3]
                    )),
                    (
                        Node::new(vec![dummy_node(10), dummy_node(20)], vec![1]),
                        2,
                        Node::new(vec![dummy_node(30), dummy_node(40)], vec![3])
                    )
                );
            }

            #[test]
            fn split_five_nodes() {
                assert_eq!(
                    split(Node::new(
                        vec![
                            dummy_node(10),
                            dummy_node(20),
                            dummy_node(30),
                            dummy_node(40),
                            dummy_node(50),
                        ],
                        vec![1, 2, 3, 4]
                    )),
                    (
                        Node::new(
                            vec![dummy_node(10), dummy_node(20), dummy_node(30)],
                            vec![1, 2]
                        ),
                        3,
                        Node::new(vec![dummy_node(40), dummy_node(50)], vec![4])
                    )
                );
            }
        }
    }
}
