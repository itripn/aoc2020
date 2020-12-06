use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashSet;

fn main() {

    let mut groups : Vec<String> = Vec::new();
    let mut str_buf : String = String::new();
    let mut groups_vec : Vec<Vec<String>> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {

        let mut group_vec : Vec<String> = Vec::new();

        // Consumes the iterator, returns an (Optional) String
        for line in lines {

            // Collapse all lines for a group into a single line
            if let Ok(l) = line {

                if l.len() > 0 {
                    str_buf.push_str( &l );
                    group_vec.push( l );
                }
                else {
                    groups.push( str_buf.to_string() ); 
                    groups_vec.push( group_vec );
                    str_buf.clear();
                    group_vec = Vec::new();
                }
            }
        }

        if str_buf.len() > 0 {
            groups.push( str_buf.to_string() );
            groups_vec.push( group_vec );
        }
    }


    part_a( &groups );

    part_b( &groups_vec );
}

fn part_b( groups_vec : &Vec<Vec<String>> ) {

    let mut sets : Vec<HashSet<char>> = Vec::new();

    let mut sum = 0;
    for group in groups_vec {

        for ans in group {
            let set : HashSet<char> = ans.chars().collect();
            sets.push( set );
        }

        let intersection = sets
            .iter()
            .skip(1)
            .fold(sets[0].clone(), |acc, hs| {
                acc.intersection(hs).cloned().collect()
            });

            sum = sum + intersection.len();
            sets.clear();
    }

    println!("Questions everyone answered yes: {}", sum );
}

fn part_a( groups : &Vec<String> ) {

        // For each line, pull out each character and toss it into 
    // a set to reduce to just the unique characters
    //
    let mut collapse_groups : Vec<HashSet<char>> = Vec::new();
    for group in groups {

        let mut set: HashSet<char> = HashSet::new();
        for c in group.chars() {
            if c != ' ' && c != '|' {
                set.insert( c );
            }
        }

        collapse_groups.push( set );
    }

    // Sum of number of elements in each set
    //
    let mut sum = 0;
    collapse_groups.iter().for_each(|el| sum = sum + el.len() );

    println!("Questions anyone answered yes: {}", sum );
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}