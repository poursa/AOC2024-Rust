pub fn _main() {
    _part1();
    _part2();
}

fn _part1(){
    // Read from input file
    let input: &str = include_str!("input.txt");
    // Split input into cols
    let mut loc_id_1:Vec<i32> = input.lines().map(|line| line.split_whitespace().next().unwrap().parse().unwrap()).collect();
    loc_id_1.sort();
    let mut loc_id_2:Vec<i32> = input.lines().map(|line| {
        let mut line_it = line.split_whitespace();
        line_it.next().unwrap();
        return line_it.next().unwrap().parse().unwrap();}).collect();
    loc_id_2.sort();
    // calculate list of diffs
    let diffs = loc_id_1.iter().zip(loc_id_2.iter()).map(|(a, b)| (a - b).abs()).sum::<i32>();
    print!("Day 1: Part 1: {}\n", diffs);
}
fn _part2(){
    // Read from input file
    let input: &str = include_str!("input.txt");
    // Split input into cols
    let loc_id_1 = input.lines().map(|line| line.split_whitespace().next().unwrap().parse().unwrap());
    let loc_id_2 = input.lines().map(|line| {
        let mut line_it = line.split_whitespace();
        line_it.next();
        line_it.next().unwrap().parse::<i32>().unwrap()});
    let s_scores: i32 = loc_id_1.map(|id_1:i32| -> i32 {
        loc_id_2.clone().filter(|id_2 |  *id_2 == id_1).count() as i32 * id_1
    }).sum();
    print!("Day 1: Part 2: {}\n", s_scores);
}
