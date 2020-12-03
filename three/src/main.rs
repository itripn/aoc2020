use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let slopes : Vec<(usize,usize)> = vec![(1,1), (3,1), (5,1), (7,1), (1,2) ];
    let mut map : Vec<String> = Vec::new();
    let mut expanded_map = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String

        for line in lines {

            if let Ok(mapline) = line {
                map.push( mapline );
            }
        }
    }

    println!("{} entries", map.len() );
    println!("Expanding map width...");

    // Bad -- hard coding the maxium right step length, should pull it from the slope tuples
    //
    let new_width = 7 * map.len();
    let line_multiplier = new_width / map[0].len() + 1;
    println!("new width: {}, line mult: {}", new_width, line_multiplier );

    for l in map {
        expanded_map.push(  l.repeat( line_multiplier ) );
    }

    let mut total_tree_count = 1;
    for s in slopes {
    
        let count = traverse_map_and_count_trees( &expanded_map, expanded_map.len(), s );
        total_tree_count *= count;

        println!( "Slope: {:?} has {} trees", s, count );
    }

    println!("Total tree count: {}", total_tree_count );
}

// Traverse the passed in tree map using the given slope (right, down) and return the number of trees encountered
//
fn traverse_map_and_count_trees( trees_map : &Vec<String>, last_line : usize, slope : ( usize, usize ) ) -> u32 {

    let mut line_idx : usize = 0;
    let mut col_idx : usize = 0;
    let mut tree_count : u32 = 0;

    loop {

        if trees_map[line_idx].chars().nth( col_idx ).unwrap() == '#' {
            tree_count = tree_count + 1;
        }

        col_idx = col_idx + slope.0;
        line_idx = line_idx + slope.1;
        if line_idx >= last_line { break; }

    }

    tree_count
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}