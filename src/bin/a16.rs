use proconio::{input, marker::Usize1};
use std::{collections::HashSet, mem::swap};

fn main() {
    input! {
        n: usize,
        ls: [usize; n - 1],
        ms: [usize; n - 1],
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
