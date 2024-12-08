use std::env;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;



fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    println!("Hello, world!");
    // day1::main();
    // day2::main();
    // day3::main();
    // day4::main();
    // day5::main();
    day6::main();
}
