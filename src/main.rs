mod task;


fn main() {

    let mut task = task::Task::new(1.0, 0.1, 0.001, 1.0, 40, 1.25);
    task.do_drop_in_time(50.0);
    

    task.do_drop_in_time(1.0);
    println!("{:#?}", task);

}
