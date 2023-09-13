use proconio::{input, marker::Usize1};
use std::mem::swap;

fn main() {
    input! {
        n: usize,
        m: usize,
        es: [(Usize1, Usize1); m],
        q: usize,
        qs: [(Usize1, Usize1, Usize1);q],
    }

    let mut tree = Tree::new(n);

    for (q, i, j) in qs {
        if q == 0 {
            tree.union(i, j);
        } else {
            println!(
                "{} ",
                if tree.root(i) == tree.root(j) {
                    "Yes"
                } else {
                    "No"
                }
            );
        }
    }
}

struct Tree {
    parents: Vec<Option<usize>>,
    sizes: Vec<usize>,
}

impl Tree {
    pub fn new(n: usize) -> Self {
        Self {
            parents: vec![None; n],
            sizes: vec![1; n],
        }
    }

    fn union(&mut self, x: usize, y: usize) {
        let mut x = self.root(x);
        let mut y = self.root(y);

        if x == y {
            return;
        } else if self.sizes[x] > self.sizes[y] {
            swap(&mut x, &mut y);
        }

        self.parents[x] = Some(y);
        self.sizes[y] += self.sizes[x];
    }

    fn root(&self, mut x: usize) -> usize {
        while let Some(y) = self.parents[x] {
            x = y;
        }

        x
    }
}
