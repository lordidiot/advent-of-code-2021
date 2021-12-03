use std::fs;
use std::env;

fn num_increase(nums: &Vec<u32>) -> u32 {
    let mut cnt = 0;
    let iter = nums.windows(2);
    for pair in iter {
        if pair[0] < pair[1] {
            cnt += 1;
        }
    };
    cnt
}

fn sliding_sum(v: &Vec<u32>) -> Vec<u32> {
    let new_v: Vec<u32> = v
        .windows(3)
        .map(|w| w[0]+w[1]+w[2])
        .collect();
    new_v
}

fn file_to_vec(filename: &str) -> Vec<u32> {
    let contents = fs::read_to_string(filename)
        .expect("Something went from wrong reading the file");
    let split = contents.split("\n");
    let v: Vec<u32> = split
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    v
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: sonar_sweep <filename>");
        return ();
    }

    let filename = &args[1];
    let v = file_to_vec(&filename);
    let window_v = sliding_sum(&v);

    let ans = num_increase(&window_v);
    println!("{}", ans);
}
