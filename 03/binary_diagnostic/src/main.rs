use std::fs;
use std::env;

fn file_to_lines(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
        .expect("Something went from wrong reading the file");
    let split = contents.lines()
                        .map(|s| s.to_string())
                        .collect();
    split
}

fn gamma_rate(lines: &Vec<String>) -> u32 {
    let n = lines[0].len();
    let mut a: Vec<u32> = vec![0; n];
    for line in lines {
        for (i, c) in line.chars().enumerate() {
            if c == '1' {
                a[i] += 1;
            }
        }
    }
    let m = lines.len() as u32;
    let g = a
        .iter()
        .fold(0, |acc, x| {
            if x >= &(m/2) { (acc<<1)+1 } else { acc << 1 }
        });
    g
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: binary_diagnostic <filename>");
        return ();
    }

    let filename = &args[1];
    let lines = file_to_lines(filename);

    let n = lines[0].len() as u32;
    let g = gamma_rate(&lines);
    let e = (u32::pow(2, n)-1) ^ g;

    println!("{} = {} * {}", e*g, g, e);
}
