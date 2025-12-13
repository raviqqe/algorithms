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
                    let right = left.split();
                    let value = self.values.pop().unwrap();

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

        if self.values.len() < N - 1 {
            self.values.insert(index, value);
            return None;
        }

        self.values.insert(index, value);

        let values = self.values.split_off(N.div_ceil(2));
        let value = self.values.pop().unwrap();

        debug_assert!(self.values.iter().all(|element| element < &value));
        debug_assert!(values.iter().all(|element| element > &value));

        Some((
            value,
            Self {
                nodes: vec![],
                values,
            },
        ))
    }

    fn split(&mut self) -> Self {
        let index = self.values.len() / 2;
        let mut nodes = self.nodes.split_off(self.nodes.len().min(index + 1));
        let values = self.values.split_off(index);

        if let Some(mut left) = self.nodes.pop() {
            let right = left.split();

            self.nodes.push(left);
            nodes.insert(0, right);
        }

        Self::new(nodes, values)
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
                3,
                Node {
                    nodes: vec![],
                    values: vec![4, 5, 6, 7],
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
}
