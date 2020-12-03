
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut old_policy_valid_count = 0;
    let mut new_policy_valid_count = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String

        for line in lines {

            if let Ok(pwline) = line {

                // Challenge 2a
                //
                if check_old_pw_policy( &pwline ) {
                    old_policy_valid_count += 1;
                }

                // Challenge 2b
                //
                if check_new_pw_policy( &pwline ) {
                    new_policy_valid_count += 1;
                }
            }
        }
    }

    println!( "Valid against old policy: {}", old_policy_valid_count );
    println!( "Valid against new policy: {}", new_policy_valid_count );
}

fn check_new_pw_policy( line : &str ) -> bool {

    let section_iter = line.split_whitespace();
    let pieces : Vec<&str> = section_iter.collect();

    let ( first, second ) = get_min_max( pieces[0] );
    let letter = pieces[1].chars().nth(0).unwrap();
    let pw = pieces[2];

    let first_ch = pw.chars().nth( ( first - 1 ) as usize ).unwrap();
    let second_ch = pw.chars().nth( ( second - 1 ) as usize ).unwrap();

    ( first_ch == letter && second_ch != letter ) || ( first_ch != letter && second_ch == letter )
}

fn check_old_pw_policy( line : &str ) -> bool {

    let section_iter = line.split_whitespace();
    let pieces : Vec<&str> = section_iter.collect();

    let ( min, max ) = get_min_max( pieces[0] );
    let letter = pieces[1].chars().nth(0).unwrap();
    let pw = pieces[2];

    let n : u32 = pw.matches( letter ).count() as u32;

    n >= min && n <= max
}

fn get_min_max( range : &str ) -> ( u32, u32 ) {

    let nums : Vec<&str> = range.split("-").collect();
    let min = nums[0].parse::<u32>().unwrap();
    let max = nums[1].parse::<u32>().unwrap();

    ( min, max )
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}