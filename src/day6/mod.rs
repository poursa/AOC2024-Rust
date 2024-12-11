
#![allow(warnings)]

use std::time::Instant;

pub fn main() {
    let now = Instant::now();


    let available_map = part1();
    part2(available_map);
    let elapsed_time = now.elapsed();
    println!("{} μs.", elapsed_time.as_micros());

}

enum Direction {
    Up,
    Down,
    Left,
    Right
}
const MAP_SIZE: usize = 130;

fn part2(available_map: ([[char; MAP_SIZE]; MAP_SIZE],i32,i32)){
    // Read from input file
    let mut map: [[char; MAP_SIZE]; MAP_SIZE] = available_map.0.clone();
    let mut sum = 0;
    for i in 0..map.len(){
        for j in 0..map[i].len(){
            map = available_map.0.clone();
            if available_map.0[i][j] != 'X' && !(i == available_map.1 as usize && j == available_map.2 as usize){
                continue;
            }
            let now = Instant::now();
            println!("Checking ({}, {})\n", i, j);
            if creates_circle(available_map.0, available_map.1 as usize, available_map.2 as usize, i, j){
                sum += 1;
            }
            let elapsed_time = now.elapsed();
            println!("{} μs.", elapsed_time.as_micros());
        
        }
    }
    
    pretty_print_array(map);

    print!("Day 6: Part 2: {}\n", sum);
    
}


fn creates_circle(available_map: [[char; MAP_SIZE]; MAP_SIZE], starting_i: usize, starting_j: usize, obstacle_i: usize, obstacle_j:usize) -> bool{

    let mut direction = Direction::Up;
    let mut inside_the_map = true;
    let mut map: [[char; MAP_SIZE]; MAP_SIZE] = available_map.clone();
    // up, right, down, left
    let mut map_dir: [[(bool,bool,bool,bool); MAP_SIZE]; MAP_SIZE] = [[(false,false,false,false); MAP_SIZE]; MAP_SIZE];

    let mut cursor_i = starting_i;
    let mut cursor_j = starting_j;
    map[obstacle_i][obstacle_j] = '#';
    let mut forms_circle = false;



    while inside_the_map && !forms_circle{
        match direction {
            Direction::Up => {
                for i in (0..=cursor_i).rev() {
                    if map[i][cursor_j] == '#' {
                        direction = Direction::Right;
                        cursor_i = i+1;
                        break;
                    }else{
                        match map_dir[i][cursor_j]{
                            (false,_,_,_) => {
                                map_dir[i][cursor_j] = (true,map_dir[i][cursor_j].1,map_dir[i][cursor_j].2,map_dir[i][cursor_j].3);
                            },
                            (true,_,_,_) => {
                                forms_circle = true;
                                break;
                            }
                        }
                        if i == 0{
                            inside_the_map = false;
                        }
                    }
                }
            },
            Direction::Right => {
                for j in cursor_j..map[cursor_i].len() {
                    if map[cursor_i][j] == '#' {
                        direction = Direction::Down;
                        cursor_j = j-1;
                        break;
                    }else{
                        match map_dir[cursor_i][j]{
                            (_,false,_,_) => {
                                map_dir[cursor_i][j] = (map_dir[cursor_i][j].0,true,map_dir[cursor_i][j].2,map_dir[cursor_i][j].3);
                            },
                            (_,true,_,_) => {
                                forms_circle = true;
                                break;
                            }
                        }
                        if j == map[cursor_i].len()-1{
                            inside_the_map = false;
                        }
                    }
                    
                }
            },
            Direction::Down => {
                for i in cursor_i..map.len() {
                    if map[i][cursor_j] == '#' {
                        direction = Direction::Left;
                        cursor_i = i-1;
                        break;
                    }else{
                        match map_dir[i][cursor_j]{
                            (_,_,false,_) => {
                                map_dir[i][cursor_j] = (map_dir[i][cursor_j].0,map_dir[i][cursor_j].1,true,map_dir[i][cursor_j].3);
                            },
                            (_,_,true,_) => {
                                forms_circle = true;
                                break;
                            }
                        }
                        if i == map.len()-1{
                            inside_the_map = false;
                        }
                    }
                    
                }
            },
            Direction::Left => {
                for j in (0..=cursor_j).rev() {
                    if map[cursor_i][j] == '#' {
                        direction = Direction::Up;
                        cursor_j = j+1;
                        break;
                    }else{
                        match map_dir[cursor_i][j]{
                            (_,_,_,false) => {
                                map_dir[cursor_i][j] = (map_dir[cursor_i][j].0,map_dir[cursor_i][j].1,map_dir[cursor_i][j].2,true);
                            },
                            (_,_,_,true) => {
                                forms_circle = true;
                                break;
                            }
                        }
                        if j == 0{
                            inside_the_map = false;
                        }
                    }
                }
            }
        }
    }
    // pretty_print_array_bools(map_dir);
    return inside_the_map;
}

fn part1() -> ([[char; MAP_SIZE]; MAP_SIZE] , i32, i32) {
    // Read from input file
    let input: &str = include_str!("input.txt");
    let map_in = input.lines();
    let mut map: [[char; MAP_SIZE]; MAP_SIZE] = [['r'; MAP_SIZE]; MAP_SIZE];
    let mut inside_the_map = true;
    let starting_i: i32;
    let starting_j: i32;
    let mut cursor_i: usize = 0;
    let mut cursor_j:  usize = 0;
    let mut direction = Direction::Up;
    {
        let mut i = 0;
        let mut j = 0;
        for line in map_in {
            j = 0;
            for char in line.chars(){
                if char == '\n' {
                    continue;
                }
                map[i][j] = char;
                if char == '^' {
                    cursor_i = i;
                    cursor_j = j;
                }
                j += 1;
            }
            i += 1;
        }
    }

    starting_i = cursor_i as i32;
    starting_j = cursor_j as i32;
    while inside_the_map{
        match direction {
            Direction::Up => {
                for i in (0..=cursor_i).rev() {
                    if map[i][cursor_j] == '#' {
                        direction = Direction::Right;
                        cursor_i = i+1;
                        break;
                    }else{
                        map[i][cursor_j] = 'X';
                        if i == 0{
                            inside_the_map = false;
                        }
                    }
                }
            },
            Direction::Right => {
                for j in cursor_j..map[cursor_i].len() {
                    if map[cursor_i][j] == '#' {
                        direction = Direction::Down;
                        cursor_j = j-1;
                        break;
                    }else{
                        map[cursor_i][j] = 'X';
                        if j == map[cursor_i].len()-1{
                            inside_the_map = false;
                        }
                    }
                    
                }
            },
            Direction::Down => {
                for i in cursor_i..map.len() {
                    if map[i][cursor_j] == '#' {
                        direction = Direction::Left;
                        cursor_i = i-1;
                        break;
                    }else{
                        map[i][cursor_j] = 'X';
                        if i == map.len()-1{
                            inside_the_map = false;
                        }
                    }
                    
                }
            },
            Direction::Left => {
                for j in (0..=cursor_j).rev() {
                    if map[cursor_i][j] == '#' {
                        direction = Direction::Up;
                        cursor_j = j+1;
                        break;
                    }else{
                        map[cursor_i][j] = 'X';
                        if j == 0{
                            inside_the_map = false;
                        }
                    }
                    
                }
            }
        }
    }
    
    // pretty_print_array(map);

    print!("Day 6: Part 1: {}\n", total_x_in_array(map));
    return (map, starting_i, starting_j);
    
}

fn pretty_print_array(array: [[char; MAP_SIZE]; MAP_SIZE]){
    for i in 0..array.len(){
        for j in 0..array[i].len(){
            print!("{}", array[i][j]);
        }
        print!("\n");
    }
}

fn pretty_print_array_bools(array: [[(bool,bool,bool,bool); MAP_SIZE]; MAP_SIZE]){
    for i in 0..array.len(){
        for j in 0..array[i].len(){
            match array[i][j]{
                (true,true,true,true) => print!("#"),
                (true,true,true,false) => print!("┘"),
                (true,true,false,true) => print!("└"),
                (true,true,false,false) => print!("┴"),
                (true,false,true,true) => print!("┐"),
                (true,false,true,false) => print!("┌"),
                (true,false,false,true) => print!("┬"),
                (true,false,false,false) => print!("^"),
                (false,true,true,true) => print!("┤"),
                (false,true,true,false) => print!("┴"),
                (false,true,false,true) => print!("├"),
                (false,true,false,false) => print!(">"),
                (false,false,true,true) => print!("┼"),
                (false,false,true,false) => print!("v"),
                (false,false,false,true) => print!("^"),
                (false,false,false,false) => print!(" ")
            }
        }
        print!("\n");
    }
}

fn total_x_in_array(array: [[char; MAP_SIZE]; MAP_SIZE]) -> i32{
    let mut sum = 0;
    for i in 0..array.len(){
        for j in 0..array[i].len(){
            if array[i][j] == 'X'{
                sum += 1;
            }
        }
    }
    return sum;
}
