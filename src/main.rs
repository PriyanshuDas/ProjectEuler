use crate::problems::problem_26::solve;

mod problems;
mod helper_modules;

fn main() {
    println!("Hello, world!");
//    solve_26();
    solve_51();
}

fn solve_51() {
    let answer_51 = problems::problem_51::solve(1000000, 8);
    println!("Answer for Problem 51 : {:?}", answer_51);
}

fn solve_26() {
    let answer_26 = problems::problem_26::solve(1, 1001);
    println!("Answer for Problem 26 (size, number) : {:?}", answer_26);
}