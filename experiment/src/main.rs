use core::{linspace::linspace, logspace::logspace, statistics::{run_algorithm, BellmanFord, FloydWarshall}};
use std::vec;

use algorithm::floyd_warshall::floyd_warshall;


pub mod core;
pub mod algorithm;



fn main() {
    let max_duration_seconds = 300.0;
    let max_iterations = 1000;
    let vertices = linspace(1000, 5_000, 10);
    let edges = vec![1.0, 1.5, 2.0];

    let bellman_ford = BellmanFord {};
    let floyd_warshall = FloydWarshall {};
    
    //run_algorithm(
    //    bellman_ford, 
    //    edges, 
    //    vertices, 
    //    max_duration_seconds, 
    //    max_iterations, 
    //    "bellman_ford"
    //);

    let max_duration_seconds = 300.0;
    let max_iterations = 1000;
    let vertices = linspace(10, 1000, 10);
    let edges = vec![1.0, 1.5, 2.0];

    run_algorithm(
        floyd_warshall, 
        edges,
        vertices, 
        max_duration_seconds, 
        max_iterations, 
        "floyd_warshall"
    );
    
}
