use std::time::Instant;

use advent_utilities::solve_task;
use human_panic::{self, setup_panic};

fn main() {
    let start_timestamp = Instant::now();
    setup_panic!();
    solve_task();
    let elapsed = start_timestamp.elapsed().as_secs_f32();
    println!("==========================");
    println!("Done in {elapsed} seconds!")
}
