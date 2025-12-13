#[derive(Clone, Debug)]
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

    pub fn new_split(left: Node<T, N>, value: T, right: Node<T, N>) -> Self {
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
            return if let Some((_value, _node)) = self.nodes[index].insert(value) {
                todo!();
            } else {
                None
            };
        }

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
    fn insert_after_degree() {
        const DEGREE: usize = 8;
        let mut node = Node::<usize, DEGREE>::new(0);

        for x in 1..100 {
            assert_eq!(node.get(&x), None);
            node.insert(x);

            for y in 0..x + 1 {
                assert_eq!(node.get(&y), Some(&y), "x = {x}, y = {y}");
            }
        }
    }
}
