mod sat;

extern crate time;

use std::{env, f64};
use sat::{Vector, has_collided};
use time::{PreciseTime, Duration};

fn main() {
    let iterations = match get_iterations() {
        Some(value) => value,
        _ => {
            println!("Usage: rust-collision-detector <iterations>");
            return;
        }
    };

    println!("Benchmarking sat::has_collided() using a shell and tank...");
    println!("Using {} iterations\n", iterations);

    println!("Time with max_dist: {}", run_test(iterations, Some(15.56)));
    println!("Time without:       {}", run_test(iterations, None));
}

fn run_test(iterations: i32, max_dist: Option<f64>) -> Duration {
    let start = PreciseTime::now();

    for _ in 0..iterations {
        let tank = vec![Vector(195.0, 95.0), Vector(205.0, 95.0), Vector(205.0, 105.0),
                        Vector(195.0, 105.0)];
        let mut shell = vec![Vector(99.5, 99.5), Vector(100.5, 99.5), Vector(100.5, 100.5),
                         Vector(99.5, 100.5)];

        while !has_collided(&tank, &shell, &max_dist) {
            for vector in shell.iter_mut() {
                vector.0 += 1.0;
            }
        }
    }

    return start.to(PreciseTime::now());
}

fn get_iterations() -> Option<i32> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        return args[1].parse().ok();
    } else {
        return None;
    }
}