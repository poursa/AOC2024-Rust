#![allow(warnings)]

use core::num;
use std::result;
use std::time::Instant;


pub fn main() {
    let now = Instant::now();

    part1();
    let elapsed_time = now.elapsed();
    println!("{} μs.", elapsed_time.as_micros());

    let now = Instant::now();

    part2();
    let elapsed_time = now.elapsed();
    println!("{} μs.", elapsed_time.as_micros());
}

#[derive(PartialEq, Debug)]
enum Operation {
    Add,
    Multiply,
    Concat,
}
#[derive(PartialEq, Debug)]
enum Small_Operation {
    Add,
    Multiply
}

fn part2(){
    // Read from input file
    let input: &str = include_str!("input.txt");
    let map_in = input.lines();
    let mut sum = 0;
    for line in map_in {
        let mut split_line = line.split(':');
        let total = split_line.next().unwrap().parse::<u64>().expect( "Not a number");
        let parts = split_line.next().unwrap().split_whitespace().map(|num| num.parse::<u64>().expect("Not a number")).collect::<Vec<u64>>();
        if combine_to_total_with_concat(parts, total){
            sum += total;
        }
    }
    println!("Day 7: Part 2:  {}",  sum);
}

fn part1(){
    // Read from input file
    let input: &str = include_str!("input.txt");
    let map_in = input.lines();
    let mut sum = 0;
    for line in map_in {
        let mut split_line = line.split(':');
        let total = split_line.next().unwrap().parse::<u64>().expect( "Not a number");
        let parts = split_line.next().unwrap().split_whitespace().map(|num| num.parse::<u64>().expect("Not a number")).collect::<Vec<u64>>();
        if combine_to_total(parts, total){
            sum += total;
        }
    }
    println!("Day 7: Part 1: {}",  sum);
}


fn combine_to_total_with_concat(mut parts: Vec<u64>, total: u64) -> bool
{
    for i in 0..parts.len(){
        if parts[i] == total{
            return true;
        }
        if i + 1 < parts.len(){
            let mut new_parts = parts.clone();
            new_parts[i] = (parts[i].to_string() + &parts[i+1].to_string()).parse::<u64>().expect("Concat is not a number");
            new_parts.remove(i+1);
            if combine_to_total(new_parts, total){
                return true;
            }
        }
    }
    false
}

fn combine_to_total(mut parts: Vec<u64>, total: u64) -> bool
{
    fn pop_stacks(result_stack: &mut Vec<u64>, operation_stack: &mut Vec<Small_Operation>) {
        result_stack.pop();
        operation_stack.pop();
    }
    let mut result_stack: Vec<u64> = vec![parts[0]*parts[1]];
    let mut operation_stack: Vec<Small_Operation> = vec![Small_Operation::Multiply];
    while result_stack.len() < parts.len(){

        if result_stack[result_stack.len()-1] == total{
            return true;
        }
        if result_stack[result_stack.len()-1] > total || result_stack.len() + 1 == parts.len(){
            match operation_stack.pop().unwrap(){
                Small_Operation::Add => {
                    result_stack.pop();
                    if result_stack.len() != 0{
                        while operation_stack.len() != 0 && operation_stack[operation_stack.len()-1] == Small_Operation::Add{
                            pop_stacks(&mut result_stack, &mut operation_stack);
                        }
                    }
                    if result_stack.len() == 0{
                        return false;
                    }
                    operation_stack.pop();
                    result_stack.pop();
                    operation_stack.push(Small_Operation::Add);
                    if result_stack.len() == 0{
                        result_stack.push(parts[0] + parts[1]);
                    }else{
                        result_stack.push(result_stack[result_stack.len()-1] + parts[result_stack.len()  + 1]);
                    }                },
                    Small_Operation::Multiply => {
                    result_stack.pop();
                    if result_stack.len() == 0{
                        result_stack.push(parts[0] + parts[1]);
                    }else{
                        result_stack.push(result_stack[result_stack.len()-1] + parts[result_stack.len()  + 1]);
                    }
                    operation_stack.push(Small_Operation::Add);
                }
            }
        }else if result_stack[result_stack.len()-1] < total{

            operation_stack.push(Small_Operation::Multiply);
            result_stack.push(result_stack[result_stack.len()-1] * parts[result_stack.len() + 1]);  
            
        }
    }
    result_stack[result_stack.len()-1] == total    
}