use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
};

// https://en.wikipedia.org/wiki/A*_search_algorithm
pub fn search(
    start: usize,
    end: usize,
    nodes: &[HashSet<(usize, usize)>],
    h: impl Fn(usize) -> usize,
) -> Option<(usize, Vec<usize>)> {
    let mut open = BinaryHeap::from_iter([Node {
        index: start,
        cost: 0,
    }]);
    let mut closed = HashSet::new();
    let mut from = HashMap::new();

    let mut f = HashMap::<_, _>::from_iter([(start, h(start))]);
    let mut g = HashMap::<_, _>::from_iter([(start, 0)]);

    while let Some(Node { index: i, .. }) = open.pop() {
        closed.insert(i);

        if i == end {
            return Some((g[&end], reconstruct(end, &from)));
        }

        for &(j, w) in &nodes[i] {
            let c = g[&i] + w;

            if c < g.get(&j).copied().unwrap_or(usize::MAX) {
                from.insert(j, i);

                g.insert(j, c);
                f.insert(j, c + h(j));

                if !closed.contains(&j) {
                    open.push(Node {
                        index: j,
                        cost: f[&j],
                    });
                }
            }
        }
    }

    None
}

fn reconstruct(mut i: usize, from: &HashMap<usize, usize>) -> Vec<usize> {
    let mut xs = vec![i];

    while let Some(&j) = from.get(&i) {
        xs.push(j);
        i = j;
    }

    xs.reverse();
    xs
}

#[derive(Eq, PartialEq)]
struct Node {
    index: usize,
    cost: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn simple() {
        assert_eq!(
            search(
                0,
                1,
                &[
                    [(1, 1)].into_iter().collect(),
                    [(0, 1)].into_iter().collect()
                ],
                |_| 0
            ),
            Some((1, vec![0, 1]))
        );
    }
}
