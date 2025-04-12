use rand::Rng;
use crate::core::graph::DiGraphMap;

pub fn generate_graph(number_of_nodes: i32, temperature: f64, weight: i32) -> DiGraphMap<i32, i32> {
    let mut rng = rand::rng();
    let mut graph: DiGraphMap<i32, i32> = DiGraphMap::new();

    for i in 0..number_of_nodes {
        graph.add_node(i);
    }

    for i in 0..(number_of_nodes - 1) {
        let w = rng.random_range(-weight..weight);
        graph.add_edge(i, i + 1, w);
    }

    let additional_edges = (number_of_nodes as f64 * temperature) as i32;
    for _ in 0..additional_edges {
        let u = rng.random_range(0..number_of_nodes);
        let v = rng.random_range(0..number_of_nodes);
        if u != v && !graph.contains_edge(u, v) {
            let w = rng.random_range(-weight..weight);
            graph.add_edge(u, v, w);
        }
    }

    graph
}
