use proconio::{input, marker::Usize1};
use std::{collections::HashSet, mem::swap};

fn main() {
    input! {
        n: usize,
        m: usize,
        es: [(Usize1, Usize1); m],
        q: usize,
        qs: [[Usize1] ;q],
    }

    let mut tree = Tree::new(n);
    let set = qs
        .iter()
        .flat_map(|q| (q.len() == 1).then_some(q[0]))
        .collect::<HashSet<_>>();

    for (i, (x, y)) in es.iter().enumerate() {
        if !set.contains(&i) {
            tree.union(*x, *y);
        }
    }

    let mut ys = vec![];

    for q in qs.iter().rev() {
        if q.len() == 1 {
            let (x, y) = &es[q[0]];
            tree.union(*x, *y);
        } else {
            ys.push(tree.root(q[0]) == tree.root(q[1]));
        }
    }

    for y in ys.into_iter().rev() {
        println!("{} ", if y { "Yes" } else { "No" });
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
