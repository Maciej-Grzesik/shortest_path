pub mod core;
pub mod algorithm;


use crate::core::graph::Graph;
use crate::algorithm::dijkstra::dijkstra;

fn main() {
    let mut graph = Graph::<i32, i32, ()>::new();

    graph.push_vertex(1, ());
    graph.push_vertex(2, ());
    graph.push_vertex(3, ());
    graph.push_vertex(4, ());

    graph.push_edge(1, 2, 5);
    graph.push_edge(1, 3, 10);
    graph.push_edge(2, 3, 1);
    graph.push_edge(2, 4, 1000);
    graph.push_edge(3, 4, 15);
}
