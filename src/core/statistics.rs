use std::{fs::File, time::Instant};
use statrs::statistics::Statistics;
use csv::Writer;
use crate::algorithm::{bellman_ford::bellman_ford, floyd_warshall::floyd_warshall};

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

pub fn run_algorithm<A: Algorithm>(
    algorithm: A, 
    edges: Vec<f64>,
    vertices: Vec<i32>,
    max_duration_seconds: f64,
    max_iterations: i32,
    fp: &str
    ) {
    let mut writer = Writer::from_writer(File::create(format!("{}.csv", fp)).unwrap());

    writer.write_record(["Vertices", "Edges", "Iteration", "Time (ms)"]).unwrap();


    for &vertice in &vertices {
        println!("Benchmarking size {}", vertice);
        for &edge in &edges {
            let mut all_times: Vec<f64> = Vec::new();
            let mut iterations = 0;
            let start_time = Instant::now();

            while start_time.elapsed().as_secs_f64() < max_duration_seconds {
                let graph = generate_graph(vertice, edge, 20);

                let time = measure_time!(algorithm.run(&graph));
                all_times.push(time);
                writer.write_record(&[
                    vertice.to_string(),
                    edge.to_string(),
                    iterations.to_string(),
                    format!("{:.3}", time * 1000.0),
                ]).unwrap();

                iterations += 1;
                if iterations > max_iterations {
                    break;
                } 
            }

            //if iterations > 0 {
            //    let avg_time = all_times.clone().mean() * 1000.0;
            //    let std_dev = all_times.clone().std_dev() * 1000.0;
            //    let median_time = median(&mut all_times) * 1000.0;
            //    let avg_time_formatted = format!("{:.1}", avg_time);
            //    let std_dev_formatted = format!("{:.1}", std_dev);
            //    let median_time_formatted = format!("{:.1}", median_time);
            //    println!("avg: {}, median: {}, std_dev: {}",  avg_time, median_time, std_dev);
            //    writer.write_record(&[
            //        vertice.to_string(),
            //        edge.to_string(),
            //        avg_time_formatted,
            //        median_time_formatted,
            //        std_dev_formatted,
            //    ]).unwrap();
            //}
        }
    }
}

pub trait Algorithm {
    fn run(&self, graph: &DiGraphMap<i32, i32>);
}

pub struct BellmanFord {
}

impl Algorithm for BellmanFord {
    fn run(&self, graph: &DiGraphMap<i32, i32>) {
        let _ = bellman_ford(graph, 0);
    }
}

pub struct FloydWarshall {
}

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
