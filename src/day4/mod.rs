
#![allow(warnings)]
pub fn main() {
    part1();
    part2();
}

#[derive(PartialEq, Debug)]
pub enum XmasState {
    X,
    XM,
    XMA
}

pub enum Dir{
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight
}

pub struct Xmas {
    pub state: XmasState,
    pub count: i32,
    pub buffer: Vec<Vec<char>>
}

impl Xmas {
    pub fn new(map_in: Vec<Vec<char>>) -> Self {
        Xmas {
            state: XmasState::X,
            count: 0,
            buffer: map_in
        }
    }
    pub fn find_all_x_pos(&mut self){
        for i in 0..self.buffer.len(){
            for j in 0..self.buffer[i].len(){
                if self.buffer[i][j] == 'X' {
                    self.parse_xmas_current_dir(i, j);
                }
            }
        }
    }
    fn parse_xmas_current_dir(&mut self, parent_i: usize, parent_j: usize){
        let mut i ;
        let mut j ;
        for dir in vec![Dir::Up, Dir::Down, Dir::Left, Dir::Right, Dir::UpLeft, Dir::UpRight, Dir::DownLeft, Dir::DownRight]{
            i = parent_i.clone();
            j = parent_j.clone();
            match dir {
                Dir::Up => {
                    if i == 0 {
                        continue;
                    }
                    while self.parse_xmas(i - 1, j){
                        i -= 1;
                        if i == 0 {
                            break;
                        }
                    }
                }
                Dir::Down => {
                    while self.parse_xmas(i + 1 ,j){
                        i += 1;
                    }
                },
                Dir::Left => {
                    if j == 0 {
                        continue;
                    }
                    while self.parse_xmas(i, j - 1){
                        j -= 1;
                        if j == 0 {
                            break;
                        }
                    }
                },
                Dir::Right => {
                    while self.parse_xmas(i, j + 1){
                        j += 1;
                    }
                },
                Dir::UpLeft => {
                    if i == 0 {
                        continue;
                    }
                    if j == 0 {
                        continue;
                    }
                    while self.parse_xmas(i - 1, j -1){
                        i -= 1;
                        j -= 1;
                        if i == 0 {
                            break;
                        }
                        if j == 0 {
                            break;
                        }
                    }
                },
                Dir::UpRight => {
                    if i == 0 {
                        continue;
                    }
                    while self.parse_xmas(i -1, j + 1){
                        i -= 1;
                        j += 1;
                        if i == 0 {
                            break;
                        }
                    }
                },
                Dir::DownLeft => {
                    if j == 0 {
                        continue;
                    }
                    while self.parse_xmas(i + 1, j - 1){
                        i += 1;
                        j -= 1;
                        if j == 0 {
                            break;
                        }
                    }
                },
                Dir::DownRight => {
                    while self.parse_xmas(i + 1, j + 1){
                        i += 1;
                        j += 1;
                    }
                }
            }
            if self.state != XmasState::X {
                self.state = XmasState::X;
            }
        }
    }
    fn parse_xmas(&mut self, i: usize, j: usize) -> bool{
        if i >= self.buffer.len() ||  j >= self.buffer[i].len(){
            self.state = XmasState::X;
            return false;
        }
        match self.state {
            XmasState::X => {
                if self.buffer[i][j] == 'M' {
                    self.state = XmasState::XM;
                    return true;
                }else {
                    self.state = XmasState::X;
                    return false;
                }
            }
            XmasState::XM => {
                if self.buffer[i][j] == 'A' {
                    self.state = XmasState::XMA;
                    return true;
                }else{
                    self.state = XmasState::X;
                    return false;
                }
            }
            XmasState::XMA => {
                if self.buffer[i][j] == 'S' {
                    self.count += 1;
                }
                self.state = XmasState::X;
                return false;
            }
        }
    }
}
fn part1(){
    // Read from input file
    let input: &str = include_str!("input.txt");
    let map_in: Vec<Vec<char>> = input.lines().map(|line: &str| {
        line.chars().collect::<Vec<char>>()
    }).collect();
    let mut xmas = Xmas::new(map_in);
    xmas.find_all_x_pos();
    print!("Day 4: Part 1: {}\n", xmas.count);
}

fn parse_x_mas(i: usize, j: usize, map_in: Vec<Vec<char>>) -> i32{
    let mut diagupleft = false;
    let mut diagupright = false;
    if i != 0 && j != 0 && map_in[i - 1][j - 1] == 'M' {
        if i < map_in.len() - 1 && j < map_in[i].len()  - 1 && map_in[i + 1][j + 1] == 'S' {
            diagupleft = true;
        }
    }
    if i != 0 && j != 0 && map_in[i - 1][j - 1] == 'S' {
        if i < map_in.len() - 1 && j < map_in[i].len() - 1 && map_in[i + 1][j + 1] == 'M' {
            diagupleft = true;
        }
    }
    if i != 0 && j < map_in[i].len()  - 1 && map_in[i - 1][j + 1] == 'M'{
        if i < map_in.len() - 1 && j != 0 && map_in[i + 1][j - 1] == 'S' {
            diagupright = true;
        }
    }
    if i != 0 && j < map_in[i].len()  - 1 && map_in[i - 1][j + 1] == 'S'{
        if i < map_in.len() - 1 && j != 0 && map_in[i + 1][j - 1] == 'M' {
            diagupright = true;
        }
    }
    if diagupleft && diagupright{
        return 1;
    }else {
        return 0;
    }
}

fn part2(){
    let input: &str = include_str!("input.txt");
    let map_in: Vec<Vec<char>> = input.lines().map(|line: &str| {
        line.chars().collect::<Vec<char>>()
    }).collect();
    let mut count = 0;
    for i in 0..map_in.len(){
        for j in 0..map_in[i].len(){
            if map_in[i][j] == 'A' {
                count += parse_x_mas(i, j, map_in.clone());
            }
        }
    }
    print!("Day 4: Part 2: {}\n", count);
}


