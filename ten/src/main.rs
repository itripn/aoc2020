use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut data : Vec<i32> = Vec::new();

    // Read lines into a vector
    //
    if let Ok(lines) = read_lines("./input.txt") {

        // Consumes the iterator, returns an (Optional) String
        for line in lines {

            // Collapse all lines for a group into a single line
            if let Ok(l) = line {
                if l.len() > 0 {
                    data.push( l.parse::<i32>().unwrap() );
                }
            }
        }
    }

    data.sort();

    let mut ones = 1;
    let mut threes = 1;

    for i in 0..data.len()-1 {

        if data[i+1] - data[i] == 1 {
            ones = ones + 1;
        }
        else if data[i+1] - data[i] == 3 {
            threes = threes + 1;
        }
        else {
            panic!("got bad delta!");
        }
    }

    println!("Ones count {} * Threes count {} = {}", ones, threes, ones * threes );
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}