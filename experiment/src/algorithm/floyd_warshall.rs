use crate::core::graph::DiGraphMap;

pub fn floyd_warshall(graph: &DiGraphMap<i32, i32>) -> Vec<Vec<i32>> {
    let n = graph.node_count();
    let mut dist: Vec<Vec<i32>> = vec![vec![i32::MAX / 2; n]; n];

    for (u, v, w) in graph.all_edges() {
        dist[*u as usize][*v as usize] = *w;
    }

    for v in graph.nodes() {
        dist[v as usize][v as usize] = 0;
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[i][k] == i32::MAX / 2 || dist[k][j] == i32::MAX / 2 {
                    continue;
                }
                if let Some(sum) = dist[i][k].checked_add(dist[k][j]) {
                    if dist[i][j] > sum {
                        dist[i][j] = sum;
                    }
                }
            }
        }
    }

    dist
}
