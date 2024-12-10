#![allow(warnings)]
pub fn main() {
    part1();
    part2();
}


fn part2(){
    // Read from input file
    let input: &str = include_str!("input.txt");
    let map_in = input.lines();
    let mut comp: [[i32; 100]; 100] = [[0; 100]; 100];
    let mut reading_comparison = true;
    let mut sum = 0;
    for line in map_in {
        // 97|75
        if reading_comparison{
            if line == ""{
                reading_comparison = false;
                continue;
            }
            let mut split = line.split("|");
            let a = split.next().unwrap().parse::<usize>().unwrap();
            let b = split.next().unwrap().parse::<usize>().unwrap();
            comp[a][b] = -1;
            comp[b][a] = 1;
        }else{
            //75,47,61,53,29
            let mut split = line.split(",");
            if !is_sorted(split.clone(), comp){
                let mut sorted_split = split.clone().collect::<Vec<&str>>();
                sorted_split.sort_by(|a, b| compare(a, b, comp));
                sum += middle(sorted_split.iter());
            }
        }
    }
    print!("Day 5: Part 2: {}\n", sum);

    
}

fn part1(){
    // Read from input file
    let input: &str = include_str!("input.txt");
    let map_in = input.lines();
    let mut comp: [[i32; 100]; 100] = [[0; 100]; 100];
    let mut reading_comparison = true;
    let mut sum = 0;
    for line in map_in {
        // 97|75
        if reading_comparison{
            if line == ""{
                reading_comparison = false;
                continue;
            }
            let mut split = line.split("|");
            let a = split.next().unwrap().parse::<usize>().unwrap();
            let b = split.next().unwrap().parse::<usize>().unwrap();
            comp[a][b] = -1;
            comp[b][a] = 1;
        }else{
            //75,47,61,53,29
            let mut split = line.split(",");
            if is_sorted(split.clone(), comp){
                let split_vec: Vec<&str> = split.collect();
                sum += middle(split_vec.iter());
            }
        }
    }
    print!("Day 5: Part 1: {}\n", sum);
    
}
fn compare(a: &&str, b: &&str, comp: [[i32; 100]; 100]) -> std::cmp::Ordering {
    let a = a.parse::<usize>().unwrap();
    let b = b.parse::<usize>().unwrap();
    match comp[a][b]{
        -1 => std::cmp::Ordering::Less,
        1 => std::cmp::Ordering::Greater,
        0 => std::cmp::Ordering::Equal,
        _ => std::cmp::Ordering::Equal,
    }
    
}

fn is_sorted( split: std::str::Split<&str>, comp: [[i32; 100]; 100]) -> bool{
    let mut vec: Vec<i32> = Vec::new();
    for s in split{
        vec.push(s.parse::<i32>().unwrap());
    }
    let len = vec.len();
    for i in 0..len-1{
        if comp[vec[i] as usize][vec[i+1] as usize] == 1{
            return false;
        }else if comp[vec[i] as usize][vec[i+1] as usize] == 0{
            println!("Non-existing pairing {} {}", vec[i], vec[i+1]);
        }
    }
    return true;
}

fn middle(split: std::slice::Iter<&str>) -> i32{
    let mut vec: Vec<i32> = Vec::new();
    for s in split{
        vec.push(s.parse::<i32>().unwrap());
    }
    let len = vec.len();
    return vec[len/2];
}
