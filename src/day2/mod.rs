
pub fn _main() {
    _part1();
    _part2();
}

fn _part1(){
    // Read from input file
    let input: &str = include_str!("input.txt");
    let safe_count = input.lines().map(|line: &str| {
        let parts = line.split_whitespace();
        return _is_safe(parts);
    }).filter(|safe: &bool| *safe).count();
    print!("Day 1: Part 1: {}\n", safe_count);
}

fn _is_safe(parts: std::str::SplitWhitespace) -> bool {
    let mut diffs = parts.clone().zip(parts.clone().skip(1)).map(|(a, b)| {
        a.parse::<i32>().unwrap() - b.parse::<i32>().unwrap()
    });
    let small_diff = diffs.clone().all(|diff: i32| diff.abs() >= 1 && diff.abs() <= 3);
    let incrementing = diffs.clone().all(|diff: i32| diff > 0);
    let decrementing = diffs.all(|diff: i32| diff < 0);
    return small_diff && (decrementing || incrementing);
}

fn _is_quite_safe(parts: std::slice::Iter<&str>) -> bool {
    let mut diffs = parts.clone().zip(parts.clone().skip(1)).map(|(a, b)| {
        a.parse::<i32>().unwrap() - b.parse::<i32>().unwrap()
    });
    let small_diff = diffs.clone().all(|diff: i32| diff.abs() >= 1 && diff.abs() <= 3);
    let incrementing = diffs.clone().all(|diff: i32| diff > 0);
    let decrementing = diffs.all(|diff: i32| diff < 0);
    return small_diff && (decrementing || incrementing);
}

fn _part2(){
    // Read from input file
    let input: &str = include_str!("input.txt");
    let safe_count = input.lines().map(|line: &str| {
        let parts = line.split_whitespace();
        if _is_safe(parts.clone()){
            return true;
        }else{
            // try running is_safe, while attempting to remove each element once.
            let order_parts = parts.clone().collect::<Vec<&str>>();
            for i in 0..order_parts.len(){
                let mut new_parts = order_parts.clone();
                new_parts.remove(i);
                if _is_quite_safe(new_parts.iter()){
                    return true;
                }
            }
            return false;
        }
    }).filter(|safe: &bool| *safe).count();
    println!("Day 1: Part 2: {}\n", safe_count);
}

