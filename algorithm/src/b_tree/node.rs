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
}
