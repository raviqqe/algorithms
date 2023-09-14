use std::collections::{BinaryHeap, HashMap, HashSet};

// https://en.wikipedia.org/wiki/A*_search_algorithm
pub fn search(
    start: usize,
    end: usize,
    nodes: &[HashSet<(usize, usize)>],
    h: impl Fn(usize) -> usize,
) -> Option<(usize, Vec<usize>)> {
    let mut q = BinaryHeap::from_iter([start]);
    let mut from = HashMap::new();

    let mut f = HashMap::<usize, _>::from_iter([(start, h(start))]);
    let mut g = HashMap::<usize, _>::from_iter([(start, 0)]);

    while let Some(i) = q.pop() {
        if i == end {
            return Some((g[&end], reconstruct(end, &from)));
        }

        for &(j, w) in &nodes[i] {
            let c = g[&i] + w;

            if c < g.get(&j).copied().unwrap_or(usize::MAX) {
                from.insert(j, i);

                g.insert(j, c);
                f.insert(j, c + h(j));

                if !q.iter().any(|&i| i == j) {
                    q.push(j);
                }
            }
        }
    }

    None
}

fn reconstruct(i: usize, from: &HashMap<usize, usize>) -> Vec<usize> {
    let mut xs = vec![i];

    while let Some(&j) = from.get(&i) {
        xs.push(j);
    }

    xs.reverse();
    xs
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
