use std::fs;
use std::env;
use std::str::FromStr;

enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

impl FromStr for Command {
    type Err = ();
    fn from_str(input: &str) -> Result<Command, Self::Err> {
        let split: Vec<&str> = input.split_whitespace().collect();
        let val = split[1].parse::<i32>().unwrap();
        let cmd = match split[0] {
            "forward" => Command::Forward(val),
            "down" => Command::Down(val),
            "up" => Command::Up(val),
            _ => return Err(()),
        };
        Ok(cmd)
    }
}

fn file_to_lines(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
        .expect("Something went from wrong reading the file");
    let split = contents.lines()
                        .map(|s| s.to_string())
                        .collect();
    split
}

fn cmds_to_position(cmds: &Vec<Command>) -> (i32, i32) {
    let mut hori: i32 = 0;
    let mut depth: i32 = 0;
    for cmd in cmds {
        match cmd {
            Command::Forward(x) => hori += x,
            Command::Down(x) => depth += x,
            Command::Up(x) => depth -= x,
        }
    }

    (hori, depth)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: dive <filename>");
        return ();
    }

    let filename = &args[1];
    let lines = file_to_lines(filename);
    let cmds: Vec<Command> = lines
        .iter()
        .map(|s| Command::from_str(s).unwrap())
        .collect();
    
    let (hori, depth) = cmds_to_position(&cmds);

    println!("{} = {} * {}", hori*depth, hori, depth);
}