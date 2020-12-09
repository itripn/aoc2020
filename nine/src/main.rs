use std::{fs::File};
use std::io::{self, BufRead};
use std::path::Path;


fn main() {

    let mut data : Vec<u64> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {

        // Consumes the iterator, returns an (Optional) String
        for line in lines {

            // Collapse all lines for a group into a single line
            if let Ok(l) = line {

                if l.len() > 0 {
                    data.push( l.parse::<u64>().unwrap() );
                }
            }
        }
    }

    // let end_preamble : usize = 25;
    let mut current_index : usize = 25;

    loop {

        let d = data[current_index];

        match find_summing_pair( &data, current_index, d) {

            Some(pair) => {
                println!("{} is sum of {} + {}", d, pair.0, pair.1);
            },

            None => {
                println!("{} is bad", d);
                break;
            }
        }

        current_index = current_index + 1;
    }
}


fn find_summing_pair( data : &Vec<u64>, upto : usize, expected : u64 ) -> Option<(u64,u64)> {

    let mut start_idx : usize = 0;

    for num in data[start_idx..upto].into_iter() {

        for num2 in data[start_idx+1..upto].into_iter() {

            if num + num2 == expected && num != num2 {
                return Some( (*num,*num2) );
            }
        }

        start_idx = start_idx + 1;
    }

    None
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}