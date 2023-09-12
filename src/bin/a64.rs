use std::{
    cmp::{Ordering, Reverse},
    collections::{BinaryHeap, HashMap},
    io::{stdin, Read},
};

fn main() {
    let mut s = String::new();
    stdin().read_to_string(&mut s).unwrap();

    let xs = s
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| {
            s.split(' ')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.parse().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    let [n, _] = &xs[0][..] else { return };

    let mut ys = vec![HashMap::new(); *n];

    for xs in &xs[1..] {
        let i = xs[0] - 1;
        let j = xs[1] - 1;
        let w = xs[2];

        ys[i].insert(j, w);
        ys[j].insert(i, w);
    }

    for z in distance(&ys) {
        println!("{}", if z == usize::MAX { -1 } else { z as isize });
    }
}

fn distance(ys: &[HashMap<usize, usize>]) -> Vec<usize> {
    let mut zs = vec![usize::MAX; ys.len()];
    let mut bs = vec![false; ys.len()];
    let mut q = BinaryHeap::new();

    q.push(Reverse(Node { index: 0, cost: 0 }));
    zs[0] = 0;

    while let Some(Reverse(x)) = q.pop() {
        if bs[x.index] {
            continue;
        }

        bs[x.index] = true;

        for (&y, &w) in &ys[x.index] {
            let z = zs[x.index] + w;

            if z < zs[y] {
                zs[y] = z;

                q.push(Reverse(Node {
                    index: y,
                    cost: zs[y],
                }));
            }
        }
    }

    zs
}

#[derive(Clone, Copy, Eq, PartialEq)]
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
