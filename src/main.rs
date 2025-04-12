use core::{logspace::logspace, statistics::{run_algorithm, BellmanFord, FloydWarshall}};
use std::vec;


pub mod core;
pub mod algorithm;



fn main() {
    let max_duration_seconds = 300.0;
    let max_iterations = 1000;
    let vertices = logspace(1000, 1_000_000, 10);
    let edges = vec![1.0, 1.5, 2.0, 3.0];

    let bellman_ford = BellmanFord {};
    let floyd_warshall = FloydWarshall {};
    
    run_algorithm(
        bellman_ford, 
        edges, 
        vertices, 
        max_duration_seconds, 
        max_iterations, 
        "bellman_ford"
    );
    
}
