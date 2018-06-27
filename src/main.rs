use std::env;

mod problems;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let solution = get_solution(args[1].parse::<i32>().unwrap());
    println!("solution to problem {}:", args[1]);
    println!("{}", solution);
}

fn get_solution(num: i32) -> i64 {
    match num {
        1 => problems::problem1::solve(),
        2 => problems::problem2::solve(),
        3 => problems::problem3::solve(),
        _ => panic!("unknow problem"),
    }
}
