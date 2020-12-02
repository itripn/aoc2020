
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut count = 0;

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String

        for line in lines {
            if let Ok(pwline) = line {
                if check_pw_policy( &pwline ) {
                    count += 1;
                }
            }
        }
    }

    println!( "Valid passwords: {}", count );
}

fn check_pw_policy( line : &str ) -> bool {

    let section_iter = line.split_whitespace();
    let pieces : Vec<&str> = section_iter.collect();

    let ( min, max ) = get_min_max( pieces[0] );
    let letter = pieces[1].chars().nth(0).unwrap();
    let pw = pieces[2];

    // print!( "Min = {}, Max = {} -- ", min, max );
    // print!( "letter = {} -- ", letter );
    // println!( "pw = {}", pw );

    let n : u32 = pw.matches( letter ).count() as u32;

    return n >= min && n <= max
}

fn get_min_max( range : &str ) -> ( u32, u32 ) {

    let nums : Vec<&str> = range.split("-").collect();
    let min = nums[0].parse::<u32>().unwrap();
    let max = nums[1].parse::<u32>().unwrap();

    (min,max)
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}