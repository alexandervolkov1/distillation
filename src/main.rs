mod task;
mod range_or_once;

use itertools::iproduct;

use task::Task;

use range_or_once::{Input, make_range_or_once};

fn main() {
    
    let mut task = Task::new(60.0, 3.0, 0.08, 25.0, 27, 1.5);

    task.do_drop_while(5.0, 2.0, 0.9999);

    println!("-------------------------------------------------------------------");
    println!("Total drops: {};", task.drop_count);
    println!("Total imurity removed fraction: {:.6};", task.sum_removed_impurity);
    println!("Total time: {:.2} days.", task.times.last().unwrap() / 24.0);
    println!("The substance remained: {:.2} liters;", task.v_pot);
    println!("-------------------------------------------------------------------");
    println!("Productivity: {:.2} liters per hour;", task.get_productivity());
    println!("Fraction of usefull product: {:.2} %;", task.get_product_yield());
    println!("-------------------------------------------------------------------");

    let mesh = get_meshgrid(3.0, 0.08, (20.0, 25.0, 0.5), (20.0, 31.0, 1.0), 1.5);

    for i in mesh {
    println!("{:?}", i);
    }

    let tasks = get_tasks(60.0, 3.0, 0.08, (20.0, 25.0, 0.5), (20.0, 31.0, 1.0), 1.5);

    for i in tasks {
    println!("{:?}", i);
    }
}

fn get_meshgrid(
    v_sec: impl Into<Input>,
    v_def: impl Into<Input>,
    flow: impl Into<Input>,
    plate_count: impl Into<Input>,
    alpha: impl Into<Input>,
) -> impl Iterator<Item = (f64, f64, f64, f64, f64)> {
    
    iproduct!(
        make_range_or_once(v_sec),
        make_range_or_once(v_def),
        make_range_or_once(flow),
        make_range_or_once(plate_count),
        make_range_or_once(alpha),
    )

}

fn get_tasks(
    v_0: f64,
    v_sec: impl Into<Input>,
    v_def: impl Into<Input>,
    flow: impl Into<Input>,
    plate_count: impl Into<Input>,
    alpha: impl Into<Input>,
) -> impl Iterator<Item = Task> {
    get_meshgrid(v_sec, v_def, flow, plate_count, alpha)
        .map(move |(a, b, c, d, e)| {
            Task::new(v_0, a, b, c, d as usize, e)
        })
}