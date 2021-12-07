use super::convex_hull::{andrew_algo, chan_algo, chan_algo_threaded, jarvis_march};
use crate::geometry::Point;
use std::time::{Duration, Instant};

pub fn run_benchmark(data: &Vec<Point>) {
    // benchmark_jarvis(&data);
    // benchmark_chan(&data);
    benchmark_chan_threaded(&data);
    println!("\n\n\n");
}

fn benchmark_jarvis(data: &Vec<Point>) {
    for _ in 0..3 {
        println!("=====");
        let now = Instant::now();
        let result = jarvis_march(&data);
        let duration = now.elapsed();
        println!("CH pt count\t{}", result.len());
        println!("jarvis time\t{}", duration.as_nanos());
    }
}

fn benchmark_andrew(data: &Vec<Point>) {
    for _ in 0..3 {
        println!("=====");
        let now = Instant::now();
        let result = andrew_algo(&data);
        let duration = now.elapsed();
        println!("CH pt count\t{}", result.len());
        println!("andrew time\t{}", duration.as_nanos());
    }
}

fn benchmark_chan(data: &Vec<Point>) {
    const init_count: usize = 2;
    for exp1 in 1..11u32 {
        for _ in 0..3 {
            println!("=====");
            // let input_data = data.clone();
            let sub_hull_count: usize = init_count * 2u32.pow(exp1.clone()) as usize;
            let now = Instant::now();
            let result = chan_algo(&data, sub_hull_count);
            let duration = now.elapsed();
            println!("CH pt count\t{}", result.len());
            println!(
                "sub_hull_count\t{}\t chan time\t{}",
                sub_hull_count,
                duration.as_nanos()
            );
        }
    }
}

fn benchmark_chan_threaded(data: &Vec<Point>) {
    const init_count: usize = 2;
    for exp1 in 1..11u32 {
        for _ in 0..3 {
            println!("=====");
            let input_data = data.clone();
            let sub_hull_count: usize = init_count * 2u32.pow(exp1.clone()) as usize;
            let now = Instant::now();
            let result = chan_algo_threaded(input_data, sub_hull_count);
            let duration = now.elapsed();
            if result.len() <= 1 {
                continue;
            }
            println!("CH pt count\t{}", result.len());
            println!(
                "sub_hull_count\t{}\t chan_threaded time\t{}",
                sub_hull_count,
                duration.as_nanos()
            );
        }
    }
}
