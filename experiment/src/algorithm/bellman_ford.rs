use crate::core::graph::DiGraphMap;

pub fn bellman_ford(graph: &DiGraphMap<i32, i32>, source: i32) -> bool {
    let mut distances: Vec<i32> = vec![i32::MAX; graph.node_count()];

    distances[source as usize] = 0;

    for _ in 0..(graph.node_count() - 1) {
        for (u, v, w) in graph.all_edges() {
            if distances[*u as usize] != i32::MAX && distances[*u as usize] + w < distances[*v as usize] {
                distances[*v as usize] = distances[*u as usize] + w;
            }
        }
    }

    for (u, v, w) in graph.all_edges() {
        if distances[*u as usize] != i32::MAX && distances[*u as usize] + w < distances[*v as usize] {
            return false;
        }
    }

    true
}
