use std::collections::HashMap;

fn search(start: usize, goal: usize, node_count: usize, h: impl Fn(i) -> usize) -> Option<usize> {
    let open = vec![start];
    let closed = HashMap::new();

    let mut g = vec![usize::MAX; node_count];
    g[start] = 0;

    let f = vec![usize::MAX, node_count];
    f[start] = g[start] + h(start);

    while let Some(i) = open.pop() {}
    // // This operation can occur in O(Log(N)) time if openSet is a min-heap or a priority queue
    // current := the node in openSet having the lowest fScore[] value
    // if current = goal {
    //     return Some()
    // }

    // openSet.Remove(current)
    // for each neighbor of current
    //     // d(current,neighbor) is the weight of the edge from current to neighbor
    //     // tentative_gScore is the distance from start to the neighbor through current
    //     tentative_gScore := gScore[current] + d(current, neighbor)
    //     if tentative_gScore < gScore[neighbor]
    //         // This path to neighbor is better than any previous one. Record it!
    //         cameFrom[neighbor] := current
    //         gScore[neighbor] := tentative_gScore
    //         fScore[neighbor] := tentative_gScore + h(neighbor)
    //         if neighbor not in openSet
    //             openSet.add(neighbor)

    None
}
