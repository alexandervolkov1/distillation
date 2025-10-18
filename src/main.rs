mod task;


fn main() {
    
    let mut task = task::Task::new(0.75, 0.07, 0.01, 0.3, 20, 1.5);

    task.do_drop_while(3.0, 2.0, 0.99);

    println!("-------------------------------------------------------------------");
    println!("Total drops: {};", task.drop_count);
    println!("Total imurity removed fraction: {:.6};", task.sum_removed_impurity);
    println!("Total time: {:.2} days.", task.times.last().unwrap() / 24.0);
    println!("The substance remained: {:.2} liters;", task.v_pot);
    println!("-------------------------------------------------------------------");
    println!("Productivity: {:.2} liters per hour;", task.get_productivity());
    println!("Fraction of usefull product: {:.2} %;", task.get_product_yield());
    println!("-------------------------------------------------------------------");
}
