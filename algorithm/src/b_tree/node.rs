use core::fmt::Debug;

macro_rules! assert_value_count {
    ($self:expr) => {
        let nodes = &$self.nodes;
        let values = &$self.values;

        debug_assert!(nodes.is_empty() || nodes.len() == values.len() + 1);
        debug_assert!(!$self.is_full());
    };
}

macro_rules! assert_node_count {
    ($self:expr) => {
        debug_assert!($self.nodes.len() != 1);
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

        assert_value_count!(&this);

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
        assert_value_count!(self);

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

        assert_value_count!(self);
        debug_assert!(self.values.iter().all(|element| element < &value));
        debug_assert!(values.iter().all(|element| element > &value));

        (value, Self::new(nodes, values))
    }

    pub fn remove(&mut self, value: &T) {
        match self.values.binary_search(value) {
            Ok(index) => {
                if let Some(node) = self.nodes.get_mut(index + 1) {
                    self.values[index] = node.remove_left();
                    self.underflow(index + 1);
                } else {
                    self.values.remove(index);
                }
            }
            Err(index) => {
                if !self.nodes.is_empty() {
                    self.nodes[index].remove(value);
                    self.underflow(index);
                }
            }
        }
    }

    fn remove_left(&mut self) -> T {
        if let Some(node) = self.nodes.get_mut(0) {
            let value = node.remove_left();
            self.underflow(0);
            value
        } else {
            self.values.remove(0)
        }
    }

    fn underflow(&mut self, index: usize) {
        let node = &mut self.nodes[index];
        let left_index = index.saturating_sub(1);

        if node.nodes.len() == 1 {
            self.merge(left_index);
        } else if node.values.is_empty() {
            self.nodes.remove(index);
            let value = self.values.remove(left_index);
            let option = self.insert(value);

            debug_assert_eq!(option, None);
        }
    }

    fn merge(&mut self, index: usize) {
        let right = self.nodes.remove(index + 1);
        let left = &mut self.nodes[index];

        left.nodes.extend(right.nodes);
        left.values.push(self.values.remove(index));
        left.values.extend(right.values);

        assert_value_count!(left);
        assert_node_count!(left);

        // TODO Split the left node.
    }

    pub fn flatten(&mut self) {
        if self.nodes.len() == 1 {
            *self = self.nodes.remove(0);
        }
    }

    const fn is_full(&self) -> bool {
        self.values.len() >= N
    }

    #[cfg(test)]
    pub fn validate(&self) -> usize {
        assert_value_count!(self);
        assert_node_count!(self);

        if self.nodes.is_empty() {
            0
        } else {
            let depths = self.nodes.iter().map(Self::validate).collect::<Vec<_>>();

            debug_assert!(depths.iter().all(|depth| *depth == depths[0]));

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

            node.remove(&1);

            assert_eq!(node, Node::new(vec![], vec![0]));
        }

        #[test]
        fn remove_element() {
            let mut node = Node::<usize, DEGREE>::new(vec![], vec![0]);

            node.remove(&0);

            assert_eq!(node, Node::new(vec![], vec![]));
        }

        #[test]
        fn remove_two_elements() {
            let mut node = Node::<usize, DEGREE>::new(vec![], vec![0, 1]);

            node.remove(&0);

            assert_eq!(node.get(&0), None);
            assert_eq!(node.get(&1), Some(&1));

            node.remove(&1);

            assert_eq!(node, Node::new(vec![], vec![]));
        }

        #[test]
        fn remove_leftmost_element_in_left_node() {
            let mut node = Node::<usize, DEGREE>::new(
                vec![Node::new(vec![], vec![0, 1]), Node::new(vec![], vec![3])],
                vec![2],
            );

            node.remove(&0);

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

            node.remove(&1);

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

            node.remove(&2);

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

            node.remove(&3);

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

            node.remove(&1);

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

            node.remove(&1);

            assert_eq!(node, Node::new(vec![Node::new(vec![], vec![0, 3])], vec![]));
        }

        #[test]
        fn remove_left_element_from_non_leaf_with_underflow() {
            let mut node = Node::<usize, DEGREE>::new(
                vec![
                    Node::new(vec![], vec![0]),
                    Node::new(vec![], vec![2]),
                    Node::new(vec![], vec![4]),
                ],
                vec![1, 3],
            );

            node.remove(&1);

            assert_eq!(
                node,
                Node::new(
                    vec![Node::new(vec![], vec![0, 2]), Node::new(vec![], vec![4])],
                    vec![3],
                )
            );
        }

        #[test]
        fn remove_right_element_from_non_leaf() {
            let mut node = Node::<usize, DEGREE>::new(
                vec![
                    Node::new(vec![], vec![0]),
                    Node::new(vec![], vec![2]),
                    Node::new(vec![], vec![4]),
                ],
                vec![1, 3],
            );

            node.remove(&3);

            assert_eq!(
                node,
                Node::new(
                    vec![Node::new(vec![], vec![0]), Node::new(vec![], vec![2, 4]),],
                    vec![1],
                )
            );
        }

        #[test]
        fn remove_left_element_from_leaf_with_leaf_underflow() {
            let mut node = Node::<usize, DEGREE>::new(
                vec![
                    Node::new(vec![], vec![0]),
                    Node::new(vec![], vec![2]),
                    Node::new(vec![], vec![4]),
                ],
                vec![1, 3],
            );

            node.remove(&0);

            assert_eq!(
                node,
                Node::new(
                    vec![Node::new(vec![], vec![1, 2]), Node::new(vec![], vec![4]),],
                    vec![3],
                )
            );
        }

        #[test]
        fn remove_right_element_from_leaf_with_leaf_underflow() {
            let mut node = Node::<usize, DEGREE>::new(
                vec![
                    Node::new(vec![], vec![0]),
                    Node::new(vec![], vec![2]),
                    Node::new(vec![], vec![4]),
                ],
                vec![1, 3],
            );

            node.remove(&4);

            assert_eq!(
                node,
                Node::new(
                    vec![Node::new(vec![], vec![0]), Node::new(vec![], vec![2, 3]),],
                    vec![1],
                )
            );
        }

        #[test]
        fn remove_left_element_from_deep_leaf_with_leaf_underflow() {
            let mut node = Node::<usize, DEGREE>::new(
                vec![
                    Node::new(
                        vec![Node::new(vec![], vec![0]), Node::new(vec![], vec![2])],
                        vec![1],
                    ),
                    Node::new(
                        vec![Node::new(vec![], vec![4]), Node::new(vec![], vec![6])],
                        vec![5],
                    ),
                ],
                vec![3],
            );

            node.remove(&0);

            assert_eq!(
                node,
                Node::new(
                    vec![Node::new(
                        vec![
                            Node::new(vec![], vec![1, 2]),
                            Node::new(vec![], vec![4]),
                            Node::new(vec![], vec![6])
                        ],
                        vec![3, 5],
                    ),],
                    vec![],
                )
            );
        }

        #[test]
        fn remove_right_element_from_deep_leaf_with_leaf_underflow() {
            let mut node = Node::<usize, DEGREE>::new(
                vec![
                    Node::new(
                        vec![Node::new(vec![], vec![0]), Node::new(vec![], vec![2])],
                        vec![1],
                    ),
                    Node::new(
                        vec![Node::new(vec![], vec![4]), Node::new(vec![], vec![6])],
                        vec![5],
                    ),
                ],
                vec![3],
            );

            node.remove(&6);

            assert_eq!(
                node,
                Node::new(
                    vec![Node::new(
                        vec![
                            Node::new(vec![], vec![0]),
                            Node::new(vec![], vec![2]),
                            Node::new(vec![], vec![4, 5]),
                        ],
                        vec![1, 3],
                    ),],
                    vec![],
                )
            );
        }

        #[test]
        fn remove_to_merge_and_overflow() {
            let mut node = Node::<usize, 3>::new(
                vec![
                    Node::new(
                        vec![
                            Node::new(
                                vec![Node::new(vec![], vec![0]), Node::new(vec![], vec![2])],
                                vec![1],
                            ),
                            Node::new(
                                vec![Node::new(vec![], vec![4]), Node::new(vec![], vec![6])],
                                vec![5],
                            ),
                        ],
                        vec![3],
                    ),
                    Node::new(
                        vec![
                            Node::new(
                                vec![Node::new(vec![], vec![8]), Node::new(vec![], vec![10])],
                                vec![9],
                            ),
                            Node::new(
                                vec![Node::new(vec![], vec![12]), Node::new(vec![], vec![14])],
                                vec![13],
                            ),
                        ],
                        vec![11],
                    ),
                ],
                vec![7],
            );

            node.validate();
            node.remove(&4);
            node.flatten();

            assert_eq!(
                node,
                Node::new(
                    vec![
                        Node::new(
                            vec![
                                Node::new(vec![], vec![0]),
                                Node::new(vec![], vec![2]),
                                Node::new(vec![], vec![5, 6])
                            ],
                            vec![1, 3],
                        ),
                        Node::new(
                            vec![Node::new(vec![], vec![8]), Node::new(vec![], vec![10])],
                            vec![9],
                        ),
                        Node::new(
                            vec![Node::new(vec![], vec![12]), Node::new(vec![], vec![14])],
                            vec![13],
                        ),
                    ],
                    vec![7, 11],
                )
            );

            node.validate();
            node.remove(&14);

            assert_eq!(
                node,
                Node::new(
                    vec![
                        Node::new(
                            vec![
                                Node::new(vec![], vec![0]),
                                Node::new(vec![], vec![2]),
                                Node::new(vec![], vec![5, 6])
                            ],
                            vec![1, 3],
                        ),
                        Node::new(
                            vec![
                                Node::new(vec![], vec![8]),
                                Node::new(vec![], vec![10]),
                                Node::new(vec![], vec![12, 13])
                            ],
                            vec![9, 11],
                        ),
                    ],
                    vec![7],
                )
            );

            node.validate();
            node.remove(&8);

            assert_eq!(
                node,
                Node::new(
                    vec![
                        Node::new(
                            vec![
                                Node::new(vec![], vec![0]),
                                Node::new(vec![], vec![2]),
                                Node::new(vec![], vec![5, 6])
                            ],
                            vec![1, 3],
                        ),
                        Node::new(
                            vec![
                                Node::new(vec![], vec![9, 10]),
                                Node::new(vec![], vec![12, 13])
                            ],
                            vec![11],
                        ),
                    ],
                    vec![7],
                )
            );
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
