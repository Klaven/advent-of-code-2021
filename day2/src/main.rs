use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world!");

    let mut h = 0;
    let mut d = 0;
    let mut aim = 0;
    if let Ok(lines) = read_lines("./input") {
        for line in lines {
            if let Ok(command) = line {
                let cmd : Vec<&str> = command.split(" ").collect();
                match cmd[0] {
                    "forward" => {
                        let val = cmd[1].parse::<i32>().unwrap();
                        h = h + val;
                        let res = d + aim * val;
                        if res < 0 {
                            d = 0;
                        }
                        else {
                            d = res;
                        }
                }
                    "down" => aim = aim + cmd[1].parse::<i32>().unwrap(),
                    "up" => aim = aim - cmd[1].parse::<i32>().unwrap(),
                    _ => println!("Invalid case")
                }
            }
        }
    }
    println!("{}", d * h)
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}