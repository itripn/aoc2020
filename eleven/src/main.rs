use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut data : Vec<Vec<char>> = Vec::new();

    // Read lines into a vector
    //
    if let Ok(lines) = read_lines("./input.txt") {

        // Consumes the iterator, returns an (Optional) String
        for line in lines {


            if let Ok(l) = line {
                if l.len() > 0 {
                    let row : Vec<char> = l.chars().collect();
                    data.push( row );
                }
            }
        }
    }

    let mut seated;
    let mut last_seated = 0;
    loop {

        seated = perform_round( &mut data );
        if last_seated == seated {
            break;
        }
        else {
            last_seated = seated;
        }
    }
}

fn perform_round( seat_map : &mut Vec<Vec<char>> ) -> i32 {

    let mut row : usize = 0;
    let mut col : usize = 0;
    let mut seated_count = 0;

    loop {

        // execute rules
    }

    seated_count
}



// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}