mod range_or_once;
mod task;

use range_or_once::make_range_or_once;


fn main() {

    let task = task::Task::new(1.0, 0.1, 0.001, 1.0, 40, 1.25, 0.99999);
    let factor = task.solve(60.0) / task.alpha.powf(task.plate_count as f64);
    println!("{}", factor);
}
