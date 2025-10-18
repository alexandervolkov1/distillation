mod task;


fn main() {
    
    let mut task = task::Task::new(60.0, 3.0, 0.08, 25.0, 27, 1.25);

    let v_0 = task.v_pot + task.v_sec + task.v_def;

    task.do_drop_while(3.0, 0.5, 0.99999);

    println!("-------------------------------------------------------------------");
    println!("Total drops: {};", task.drop_count);
    println!("Total imurity removed fraction: {:.2};", task.sum_removed_impurity);
    println!("Total time: {:.2} days.", task.times.last().unwrap() / 24.0);
    println!("The substance remained: {:.2} liters;", task.v_pot);
    println!("-------------------------------------------------------------------");
    println!("Productivity: {:.2} liters per hour;", task.v_pot / task.times.last().unwrap());
    println!("Fraction of usefull product: {:.2} %;", task.v_pot / v_0 * 100.0);
    println!("-------------------------------------------------------------------");
}
