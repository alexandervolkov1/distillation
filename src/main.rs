mod task;


fn main() {

    let mut task = task::Task::new(1.0, 0.1, 0.001, 1.0, 40, 1.25);
    println!("{:#?}", task);

    let factor = task.solve(60.0) / task.alpha.powf(task.plate_count as f64);
    println!("{}", factor);

    task.do_sample_if_lim(0.5 * task.alpha.powf(task.plate_count as f64));
    println!("{:#?}", task);

    task.do_sample_if_lim(0.999 * task.alpha.powf(task.plate_count as f64));
    println!("{:#?}", task);
}
