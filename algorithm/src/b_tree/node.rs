use core::fmt::Debug;

macro_rules! assert_invariant {
    ($self:expr) => {
        let nodes = &$self.nodes;
        let values = &$self.values;

        debug_assert!(nodes.is_empty() || nodes.len() == values.len() + 1);
        debug_assert!(!$self.is_full());
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

    #[must_use]
    pub fn get(&self, value: &T) -> Option<&T> {
        let index = match self.values.binary_search(value) {
            Ok(index) => return self.values.get(index),
            Err(index) => index,
        };

        self.nodes.get(index).and_then(|node| node.get(value))
    }

    #[must_use]
    pub fn insert(&mut self, value: T) -> Option<(T, Self)> {
        assert_invariant!(self);

        match self.values.binary_search(&value) {
            Ok(index) => {
                self.values[index] = value;
                None
            }
            Err(index) => {
                if self.nodes.is_empty() {
                    self.values.insert(index, value);
                } else if let Some((value, node)) = self.nodes[index].insert(value) {
                    self.nodes.insert(index + 1, node);
                    self.values.insert(index, value);
                }

                if self.is_full() {
                    Some(self.split())
                } else {
                    None
                }
            }
        }
    }

    #[must_use]
    fn split(&mut self) -> (T, Self) {
        let index = self.values.len() / 2 + 1;
        let nodes = self.nodes.split_off(self.nodes.len().min(index));
        let values = self.values.split_off(index);
        let value = self.values.pop().unwrap();

        assert_invariant!(self);
        debug_assert!(self.values.iter().all(|element| element < &value));
        debug_assert!(values.iter().all(|element| element > &value));

        (value, Self::new(nodes, values))
    }

    #[must_use]
    pub fn remove(&mut self, value: &T) -> Option<(T, Self)> {
        match self.values.binary_search(&value) {
            Ok(index) => {
                if let Some(node) = self.nodes.get_mut(index + 1) {
                    let value = node.remove_left();

                    // TODO
                    if node.is_empty() {
                        dbg!(&node);
                        dbg!(&self);
                        self.nodes.remove(index + 1);
                        self.values.remove(index);
                        dbg!(&self);
                        let foo = self.insert(value);
                        dbg!(&self);
                        foo
                    } else {
                        self.values[index] = value;
                        None
                    }
                } else {
                    self.values.remove(index);
                    None
                }
            }
            Err(index) if index < N - 1 && !self.nodes.is_empty() => {
                let node = &mut self.nodes[index];

                if let Some((_value, _node)) = node.remove(&value) {
                    // TODO
                }

                if node.is_empty() {
                    *node = Self::new(vec![], vec![self.values.remove(index)]);
                }

                assert_invariant!(self);

                None
            }
            Err(_) => None,
        }
    }

    fn remove_left(&mut self) -> T {
        if let Some(node) = self.nodes.get_mut(0) {
            let value = node.remove_left();
            self.underflow(0, 0);
            value
        } else {
            self.values.remove(0)
        }
    }

    fn underflow(&self, index: usize, right: bool) -> Option<()> {
        let node_index = index + if right { 1 } else { 0 };

        if self.nodes[node_index].is_empty() {
            let node = self.nodes.remove(node_index);
            self.values.remove(value_index);
            self.insert(value)
        } else {
            None
        }
    }

    pub const fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    const fn is_full(&self) -> bool {
        self.values.len() >= N
    }

    #[cfg(test)]
    pub fn assert_depth(&self) -> usize {
        if self.nodes.is_empty() {
            0
        } else {
            let depths = self
                .nodes
                .iter()
                .map(Self::assert_depth)
                .collect::<Vec<_>>();

            assert!(depths.iter().all(|depth| *depth == depths[0]));

            depths[0] + 1
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
            assert_eq!(node.insert(x), None);

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

    mod remove {
        use super::*;
        use pretty_assertions::assert_eq;

        const DEGREE: usize = 8;

        #[test]
        fn remove_none() {
            let mut node = Node::<usize, DEGREE>::new(vec![], vec![0]);

            assert_eq!(node.remove(&1), None);
            assert_eq!(node, Node::new(vec![], vec![0]));
        }

        #[test]
        fn remove_element() {
            let mut node = Node::<usize, DEGREE>::new(vec![], vec![0]);

            assert_eq!(node.remove(&0), None);
            assert_eq!(node, Node::new(vec![], vec![]));
        }

        #[test]
        fn remove_two_elements() {
            let mut node = Node::<usize, DEGREE>::new(vec![], vec![0, 1]);

            assert_eq!(node.remove(&0), None);
            assert_eq!(node.get(&0), None);
            assert_eq!(node.get(&1), Some(&1));

            assert_eq!(node.remove(&1), None);
            assert_eq!(node, Node::new(vec![], vec![]));
        }

        #[test]
        fn remove_leftmost_element_in_left_node() {
            let mut node = Node::<usize, DEGREE>::new(
                vec![Node::new(vec![], vec![0, 1]), Node::new(vec![], vec![3])],
                vec![2],
            );

            assert_eq!(node.remove(&0), None);
            assert_eq!(
                node,
                Node::new(
                    vec![Node::new(vec![], vec![1]), Node::new(vec![], vec![3])],
                    vec![2],
                )
            );
        }

        #[test]
        fn remove_rightmost_element_in_left_node() {
            let mut node = Node::<usize, DEGREE>::new(
                vec![Node::new(vec![], vec![0, 1]), Node::new(vec![], vec![3])],
                vec![2],
            );

            assert_eq!(node.remove(&1), None);
            assert_eq!(
                node,
                Node::new(
                    vec![Node::new(vec![], vec![0]), Node::new(vec![], vec![3])],
                    vec![2],
                )
            );
        }

        #[test]
        fn remove_leftmost_element_in_right_node() {
            let mut node = Node::<usize, DEGREE>::new(
                vec![Node::new(vec![], vec![0]), Node::new(vec![], vec![2, 3])],
                vec![1],
            );

            assert_eq!(node.remove(&2), None);
            assert_eq!(
                node,
                Node::new(
                    vec![Node::new(vec![], vec![0]), Node::new(vec![], vec![3])],
                    vec![1],
                )
            );
        }

        #[test]
        fn remove_rightmost_element_in_right_node() {
            let mut node = Node::<usize, DEGREE>::new(
                vec![Node::new(vec![], vec![0]), Node::new(vec![], vec![2, 3])],
                vec![1],
            );

            assert_eq!(node.remove(&3), None);
            assert_eq!(
                node,
                Node::new(
                    vec![Node::new(vec![], vec![0]), Node::new(vec![], vec![2])],
                    vec![1],
                )
            );
        }

        #[test]
        fn remove_element_from_non_leaf() {
            let mut node = Node::<usize, DEGREE>::new(
                vec![Node::new(vec![], vec![0]), Node::new(vec![], vec![2, 3])],
                vec![1],
            );

            assert_eq!(node.remove(&1), None);
            assert_eq!(
                node,
                Node::new(
                    vec![Node::new(vec![], vec![0]), Node::new(vec![], vec![3])],
                    vec![2],
                )
            );
        }

        #[test]
        fn remove_element_from_non_leaf_with_underflow() {
            let mut node = Node::<usize, DEGREE>::new(
                vec![Node::new(vec![], vec![0]), Node::new(vec![], vec![3])],
                vec![1],
            );

            let option = node.remove(&1);

            assert_eq!(option, Some((3, Node::new(vec![], vec![0]))));
        }
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
