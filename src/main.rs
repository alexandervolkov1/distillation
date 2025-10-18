mod task;
mod range_or_once;

use range_or_once::{Input, make_range_or_once};
use itertools::iproduct;

fn main() {
    
    let mut task = task::Task::new(60.0, 3.0, 0.08, 25.0, 27, 1.5);

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

    let mut mesh = get_tasks_meshgrid(3.0.into(), 0.08.into(), 25.0.into(), (20.0,31.0, 1.0).into(), 1.5.into());

    println!("{:#?}", mesh.next().unwrap());
    println!("{:#?}", mesh.next().unwrap());
    println!("{:#?}", mesh.next().unwrap());
    println!("{:#?}", mesh.next().unwrap());
    println!("{:#?}", mesh.next().unwrap());
    println!("{:#?}", mesh.next().unwrap());
    println!("{:#?}", mesh.next().unwrap());
}

fn get_tasks_meshgrid(
    v_sec: Input,
    v_def: Input,
    flow: Input,
    plate_count: Input,
    alpha: Input,
) -> impl Iterator<Item = (f64, f64, f64, f64, f64)> {
    let v_sec_iter = make_range_or_once(v_sec);
    let v_def_iter = make_range_or_once(v_def);
    let flow_iter = make_range_or_once(flow);
    let plate_count_iter = make_range_or_once(plate_count);
    let alpha_iter = make_range_or_once(alpha);
    
    iproduct!(v_sec_iter, v_def_iter, flow_iter, plate_count_iter, alpha_iter)
}