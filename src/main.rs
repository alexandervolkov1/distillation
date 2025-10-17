mod task;


fn main() {

    let mut task = task::Task::new(60.0, 3.0, 0.08, 25.0, 27, 1.5);

    task.do_drop_in_time(3.0);

    for _ in 0..50 {
        task.do_drop_in_time(1.0);
    }
    println!("{:#?}", task);

}
