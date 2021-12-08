use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world!");

    if let Ok(lines) = read_lines("./input") {
        // Consumes the iterator, returns an (Optional) String
        let mut count = 0;
        let mut last_level = (0,0,0);
        let mut next_level = (0,0,0);
        let mut counter = 0;
        for line in lines {
            if let Ok(depth) = line {
                let this_level = depth.parse::<i32>().unwrap();
                let (_a,b,c) = last_level; 
                next_level = (b,c,this_level);
                if counter > 2 {
                    if sum_tup(next_level) > sum_tup(last_level) {
                        count += 1
                    }
                }
                last_level = next_level;
                counter += 1
            }
        }
        println!("{}", count)
    }
}

fn sum_tup(tup: (i32,i32,i32)) -> i32 {
    let (a,b,c) = tup;
    a + b + c
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}