#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Node<T, const N: usize> {
    nodes: Vec<Self>,
    values: Vec<T>,
}

impl<T: Ord, const N: usize> Node<T, N> {
    pub fn new(value: T) -> Self {
        Self {
            nodes: vec![],
            values: vec![value],
        }
    }

    pub fn new_split(left: Self, value: T, right: Self) -> Self {
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

    pub fn insert(&mut self, value: T) -> Option<(T, Self)> {
        let index = match self.values.binary_search(&value) {
            Ok(index) => {
                self.values[index] = value;
                return None;
            }
            Err(index) => index,
        };

        if !self.nodes.is_empty() {
            return if let Some((value, node)) = self.nodes[index].insert(value) {
                if self.values.len() < N - 1 {
                    self.nodes.insert(index + 1, node);
                    self.values.insert(index, value);
                    None
                } else {
                    let index = N.div_ceil(2);
                    let mut nodes = self.nodes.split_off(index + 1);
                    let values = self.values.split_off(index);
                    let mut left = self.nodes.pop().unwrap();
                    let right = left.split();
                    let value = self.values.pop().unwrap();

                    self.nodes.push(left);
                    nodes.insert(0, right);

                    debug_assert_eq!(nodes.len(), values.len() + 1);
                    debug_assert!(values.iter().all(|element| element > &value));

                    Some((value, Self { nodes, values }))
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

        Some((
            self.values.pop().unwrap(),
            Self {
                nodes: vec![],
                values,
            },
        ))
    }

    fn split(&mut self) -> Self {
        let index = self.nodes.len() / 2;
        let mut nodes = if self.nodes.is_empty() {
            vec![]
        } else {
            self.nodes.split_off(index + 1)
        };
        let values = self.values.split_off(index);

        if self.nodes.is_empty() {
            Self { nodes, values }
        } else {
            let mut left = self.nodes.pop().unwrap();
            let right = left.split();

            self.nodes.push(left);
            nodes.insert(0, right);

            Self { nodes, values }
        }
    }
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
    fn insert_before_degree() {
        const DEGREE: usize = 8;
        let mut node = Node::<usize, DEGREE>::new(0);

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
        let mut node = Node::<usize, DEGREE>::new(0);

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
        let mut node = Node::<usize, DEGREE>::new(0);

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
