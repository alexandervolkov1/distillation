mod task;
mod range_or_once;

use std::time::Instant;
use itertools::iproduct;

use rayon::prelude::*;

use task::Task;
use range_or_once::{Input, make_range_or_once};

fn main() {

    let start = Instant::now();
    
    let tasks = get_tasks(
        60.0,
        3.0,
        (0.005, 0.015, 0.001),
        25.0,
        27.0,
        1.5, 
        (5.0, 18.0, 1.0),
        (0.01, 0.051, 0.01),
        0.9999
    );

    let best_task = tasks.max_by(|task1, task2| {
        let eff1 = task1.efficiency();
        let eff2 = task2.efficiency();
        eff1.partial_cmp(&eff2).unwrap()
    }).unwrap();

    println!("===================================================================");
    println!("THE BEST PARAMETERS ARE:");
    println!("-------------------------------------------------------------------");
    println!("Total drops: {:.2};", best_task.drop_count);
    println!("Start period: {:.2} hours;", best_task.times[0]);
    println!("Time between drops: {:.2} hours;", best_task.times[1] - best_task.times[0]);
    println!("One drop volume: {:.2} liters;", best_task.v_def);
    println!("-------------------------------------------------------------------");
    println!("Total imurity removed fraction: {:.6};", best_task.sum_removed_impurity);
    println!("Total time: {:.2} hours;", best_task.times.last().unwrap());
    println!("The substance remained: {:.2} liters;", best_task.v_pot);
    println!("-------------------------------------------------------------------");
    println!("Productivity: {:.2} liters per hour;", best_task.productivity());
    println!("Fraction of usefull product: {:.2} %;", best_task.product_yield());
    println!("Effeciency of process: {:.2};", best_task.efficiency());
    println!("-------------------------------------------------------------------");    

    let duration = start.elapsed();
    println!("Total execution time: {:.2?}", duration);
    }

fn get_meshgrid(
    v_sec: impl Into<Input>,
    v_def: impl Into<Input>,
    flow: impl Into<Input>,
    plate_count: impl Into<Input>,
    alpha: impl Into<Input>,
    start_period: impl Into<Input>,
    time_between_drops: impl Into<Input>
) -> impl Iterator<Item = (f64, f64, f64, f64, f64, f64, f64)> {
    
    iproduct!(
        make_range_or_once(v_sec),
        make_range_or_once(v_def),
        make_range_or_once(flow),
        make_range_or_once(plate_count),
        make_range_or_once(alpha),
        make_range_or_once(start_period),
        make_range_or_once(time_between_drops)
    )
    
}

fn get_tasks(
    v_0: f64,
    v_sec: impl Into<Input>,
    v_def: impl Into<Input>,
    flow: impl Into<Input>,
    plate_count: impl Into<Input>,
    alpha: impl Into<Input>,
    start_period: impl Into<Input>,
    time_between_drops: impl Into<Input>,
    needed_fraction: f64
) -> impl ParallelIterator<Item = Task> {
    get_meshgrid(v_sec, v_def, flow, plate_count, alpha, start_period, time_between_drops)
        .par_bridge()
        .map(move |(a, b, c, d, e, f, g)| {
            let mut temp_task = Task::new(v_0, a, b, c, d as usize, e);
            temp_task.do_drop_while(f, g, needed_fraction);
            temp_task
        })
}
