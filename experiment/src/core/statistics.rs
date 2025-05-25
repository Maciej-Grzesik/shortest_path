use crate::algorithm::{bellman_ford::bellman_ford, floyd_warshall::floyd_warshall};
use csv::Writer;
use rayon::prelude::*;
use std::{fs::File, sync::Mutex, time::Instant};

use super::{generate_graph::generate_graph, graph::DiGraphMap};

#[macro_export]
macro_rules! measure_time {
    ($fn_call:expr) => {{
        let start = std::time::Instant::now();
        $fn_call;
        let duration = start.elapsed().as_secs_f64();

        duration
    }};
}

pub fn run_algorithm<A: Algorithm + Sync>(
    algorithm: A,
    edges: Vec<f64>,
    vertices: Vec<i32>,
    max_duration_seconds: f64,
    max_iterations: i32,
    fp: &str,
) {
    let writer = Mutex::new(Writer::from_writer(
        File::create(format!("{}.csv", fp)).unwrap(),
    ));

    writer
        .lock()
        .unwrap()
        .write_record(["Vertices", "Edges", "Iteration", "Time (ms)"])
        .unwrap();

    vertices.par_iter().for_each(|&vertice| {
        println!("Benchmarking size {}", vertice);
        for &edge in &edges {
            let mut iterations = 0;
            let start_time = Instant::now();

            while start_time.elapsed().as_secs_f64() < max_duration_seconds {
                let graph = generate_graph(vertice, edge, 20);

                let time = measure_time!(algorithm.run(&graph));

                writer
                    .lock()
                    .unwrap()
                    .write_record(&[
                        vertice.to_string(),
                        edge.to_string(),
                        iterations.to_string(),
                        format!("{:.3}", time * 1000.0),
                    ])
                    .unwrap();

                iterations += 1;
                if iterations % 50 == 0 {
                    println!(
                        "Core for size {}: {:.2} ms, iteration {}",
                        vertice,
                        time * 1000.0,
                        iterations
                    );
                }
                if iterations > max_iterations {
                    break;
                }
            }
        }
    });
}

pub trait Algorithm {
    fn run(&self, graph: &DiGraphMap<i32, i32>);
}

pub struct BellmanFord {}

impl Algorithm for BellmanFord {
    fn run(&self, graph: &DiGraphMap<i32, i32>) {
        let _ = bellman_ford(graph, 0);
    }
}

pub struct FloydWarshall {}

impl Algorithm for FloydWarshall {
    fn run(&self, graph: &DiGraphMap<i32, i32>) {
        let _ = floyd_warshall(graph);
    }
}

fn median(times: &mut [f64]) -> f64 {
    times.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let len = times.len();
    if len % 2 == 0 {
        (times[len / 2 - 1] + times[len / 2]) / 2.0
    } else {
        times[len / 2]
    }
}
