use std::collections::{BinaryHeap, HashMap, HashSet};

fn search(
    start: usize,
    end: usize,
    nodes: &[HashSet<usize>],
    h: impl Fn(usize) -> usize,
) -> Option<usize> {
    let mut open = BinaryHeap::from_iter([start]);
    let mut closed = HashMap::new();

    let mut g = vec![usize::MAX; nodes.len()];
    g[start] = 0;

    let mut f = vec![usize::MAX, nodes.len()];
    f[start] = g[start] + h(start);

    while let Some(i) = open.pop() {
        if i == end {
            return Some(f[end]);
        }

        for each neighbor of current
            // d(current,neighbor) is the weight of the edge from current to neighbor
            // tentative_gScore is the distance from start to the neighbor through current
            tentative_gScore := gScore[current] + d(current, neighbor)
            if tentative_gScore < gScore[neighbor]
                // This path to neighbor is better than any previous one. Record it!
                cameFrom[neighbor] := current
                gScore[neighbor] := tentative_gScore
                fScore[neighbor] := tentative_gScore + h(neighbor)
                if neighbor not in openSet
                    openSet.add(neighbor)
    }

    None
}
