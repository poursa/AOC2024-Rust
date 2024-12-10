use std::time::Instant;

pub fn main() {
    let now = Instant::now();


    part1();
    part2();
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

fn part2(){
    // Read from input file
    let input: &str = include_str!("input.txt");
    let map_in = input.lines();
    let mut map: [[char; MAP_SIZE]; MAP_SIZE] = [['r'; MAP_SIZE]; MAP_SIZE];
    let mut inside_the_map = true;
    let mut forms_circle = false;
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
                    map[i][j] = '.';
                }
                j += 1;
            }
            i += 1;
        }
    }

    while inside_the_map{
        match direction {
            Direction::Up => {
                for i in (0..=cursor_i).rev() {
                    if map[i][cursor_j] == '#' {
                        direction = Direction::Right;
                        cursor_i = i+1;
                        break;
                    }else{
                        if 
                        map[i][cursor_j] = '^';
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
                        map[cursor_i][j] = '>';
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
                        map[i][cursor_j] = 'v';
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
                        map[cursor_i][j] = '<';
                        if j == 0{
                            inside_the_map = false;
                        }
                    }
                }
            }
        }
    }
    pretty_print_array(map);

    print!("Day 6: Part 2: {}\n", total_x_in_array(map));
    
}

fn part1(){
    // Read from input file
    let input: &str = include_str!("input.txt");
    let map_in = input.lines();
    let mut map: [[char; MAP_SIZE]; MAP_SIZE] = [['r'; MAP_SIZE]; MAP_SIZE];
    let mut inside_the_map = true;
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
    
}

fn pretty_print_array(array: [[char; MAP_SIZE]; MAP_SIZE]){
    for i in 0..array.len(){
        for j in 0..array[i].len(){
            print!("{}", array[i][j]);
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
