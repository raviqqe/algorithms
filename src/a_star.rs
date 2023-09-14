use std::collections::{BinaryHeap, HashMap, HashSet};

fn search(
    start: usize,
    end: usize,
    nodes: &[HashSet<(usize, usize)>],
    h: impl Fn(usize) -> usize,
) -> Option<usize> {
    let mut q = BinaryHeap::from_iter([start]);
    let mut closed = HashMap::new();

    let mut f = HashMap::from_iter([(start, h(start))]);
    let mut g = HashMap::from_iter([(start, 0)]);

    while let Some(i) = q.pop() {
        if i == end {
            return Some(f[&end]);
        }

        for (j, w) in nodes[i] {
            // d(current,neighbor) is the weight of the edge from current to neighbor
            // tentative_gScore is the distance from start to the neighbor through current
            // tentative_gScore := gScore[current] + d(current, neighbor)
            // if tentative_gScore < gScore[neighbor]
            //     // This path to neighbor is better than any previous one. Record it!
            //     cameFrom[neighbor] := current
            //     gScore[neighbor] := tentative_gScore
            //     fScore[neighbor] := tentative_gScore + h(neighbor)
            //     if neighbor not in openSet
            //         openSet.add(neighbor)
        }
    }

    None
}
