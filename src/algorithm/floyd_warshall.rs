use crate::core::graph::DiGraphMap;

pub fn floyd_warshall(graph: &DiGraphMap<i32, i32>) -> Vec<Vec<i32>> {
    let n = graph.node_count();
    let mut distances: Vec<Vec<i32>> = vec![vec![i32::MAX / 2; n]; n];

    for (u, v, w) in graph.all_edges() {
        distances[u as usize][v as usize] = w;
    }

    for v in graph.nodes() {
        distances[v as usize][v as usize] = 0;
    }

    for k in 0..n {
        println!("{:?}", distances);
        for i in 0..n {
            for j in 0..n {
                if distances[i][j] > distances[i][k] + distances[k][j] {
                    distances[i][j] = distances[i][k] + distances[k][j];
                }
            }
        }
    }

    distances
}
