use std::{fs::File};
use std::io::{self, BufRead};
use std::path::Path;


fn main() {

    let mut data : Vec<u64> = Vec::new();

    // Read lines into a vector
    //
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

    // Walk through the vector starting past the preamble, and look
    // for a pair of numbers previous to the current number which
    // add up to that number. 
    //
    let mut current_index : usize = 25;
    let mut bad_num : u64;
    loop {

        bad_num = data[current_index];

        match find_summing_pair( &data, current_index, bad_num) {

            Some(_pair) => {
                // println!("{} is sum of {} + {}", bad_num, pair.0, pair.1);
            },

            None => {
                println!("{} is bad", bad_num);
                break;
            }
        }

        current_index = current_index + 1;
    }

    // Now we have the bad number in 'd', let's look for a contiguous block
    // of numbers (more than 2) whose sum is this bad number.
    //
    current_index = 25;
    let mut sum : u64 = 0;
    let mut bottom : usize;
    let mut top : usize = 0;
    let mut found = false;

    loop {

        bottom = current_index;
        for n in current_index..data.len()-1 {

            sum = sum + data[n];
            if sum == bad_num {
                top = n;
                found = true;
                break;
            }
            else if sum > bad_num {
                sum = 0;
                current_index = current_index + 1;
                break;                
            }
        }

        // If we found a set of numbers that works, get the min and max and print their sum
        // as the problem solution
        //
        if found {
            let slice : Vec<u64> = data[bottom..top].to_vec();
            println!("Sum of smallest and largest is {}", slice.iter().max().unwrap() + slice.iter().min().unwrap() );
            break;
        }
    }
}


fn find_summing_pair( data : &Vec<u64>, upto : usize, expected : u64 ) -> Option<(u64,u64)> {

    let mut start_idx : usize = 0;

    for num in data[start_idx..upto].into_iter() {

        for num2 in data[start_idx+1..upto].into_iter() {

            if num + num2 == expected && num != num2 {
                return Some( (*num, *num2) );
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