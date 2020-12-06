use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() {

    let mut groups : Vec<String> = Vec::new();
    let mut str_buf : String = String::new();

    if let Ok(lines) = read_lines("./input.txt") {

        // Consumes the iterator, returns an (Optional) String
        for line in lines {

            if let Ok(l) = line {

                if l.len() > 0 {
                    str_buf.push_str( &l );
                }
                else {
                    groups.push( str_buf.clone() );
                    str_buf.clear();
                }
            }
        }

        if str_buf.len() > 0 {
            groups.push( str_buf.clone() );
        }
    }

    let mut collapse_groups : Vec<HashSet<char>> = Vec::new();
    for group in groups {

        let mut set: HashSet<char> = HashSet::new();

        for c in group.chars() {
            if c != ' ' {
                set.insert( c );
            }
        }

        collapse_groups.push( set );
    }

    let mut sum = 0;
    for cg in collapse_groups {
        // println!("{:?}", cg );
        sum = sum + cg.len();
    }

    println!("Sum of all group answers: {}", sum );
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}