use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut data : Vec<Vec<char>> = Vec::new();

    // Read lines into a vector
    //
    if let Ok(lines) = read_lines("input.txt") {

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

    let mut last_seated_count = dump_seat_map( &data );
    let mut rounds = 1;

    loop {

        data = perform_round( &mut data );
        println!("\n**** After round: {} **********************************************", rounds );
        
        let seated_count = dump_seat_map( &data );

        if seated_count != last_seated_count {
            last_seated_count = seated_count;
        }
        else {
            println!("Final seated count: {}", seated_count );
            break;
        }

        rounds = rounds + 1;
    }
}

fn perform_round( cur_seat_map : &mut Vec<Vec<char>> ) -> Vec<Vec<char>> {

    let mut new_seat_map = cur_seat_map.clone();

    for row in 0..cur_seat_map.len() {

        for col in 0..cur_seat_map[row].len() {

            let curseat = cur_seat_map[row][col];

            if curseat == 'L' && is_adjacent_to_seat_type( '#', row, col, &cur_seat_map ) == 0 {
                new_seat_map[row][col] = '#';
            }
            else if curseat == '#' && is_adjacent_to_seat_type( '#', row, col, &cur_seat_map ) >=4 {
                new_seat_map[row][col] = 'L';
            }
            else  {
                new_seat_map[row][col] = curseat;
            }
        }
    }

    new_seat_map
}

fn is_adjacent_to_seat_type( seat_type : char, row : usize, col : usize, map : &Vec<Vec<char>> ) -> i32 {

    let mut adjacent_count = 0;

    // Cover seat to left if there is one
    //
    if col > 0 {
        if map[row][col-1] == seat_type {
            adjacent_count += 1;
        }
    }    

    // Cover seat to right if there is one
    //
    if col < map[row].len()-1 {
        if map[row][col+1] == seat_type {
            adjacent_count += 1;
        }
    }

    // Cover seat above if there is one
    //
    if row > 0 {
        if map[row-1][col] == seat_type {
            adjacent_count += 1;
        }
    }

    // Cover seat below if there is one
    //
    if row < map.len()-1 {
        if map[row+1][col] == seat_type {
            adjacent_count += 1;
        }
    }

    // Cover diagonal up left
    //
    if col > 0 && row > 0 {
        if map[row-1][col-1] == seat_type {
            adjacent_count += 1;
        }
    }

    // Cover diagonal up right
    //
    if  row > 0 && col < map[row].len()-1 {
        if map[row-1][col+1] == seat_type {
            adjacent_count += 1;
        }
    }

    // Cover diagonal down left
    //
    if col > 0 && row < map.len()-1 {
        if map[row+1][col-1] == seat_type {
            adjacent_count += 1;
        }
    }

    // Cover diagonal down right
    //
    if  row < map.len()-1 && col < map[row].len()-1 {
        if map[row+1][col+1] == seat_type {
            adjacent_count += 1;
        }
    }

    adjacent_count
}

fn dump_seat_map( seat_map : &Vec<Vec<char>> ) -> i32 {

    let mut seat_count = 0;

        for row in 0..seat_map.len() {

            for col in 0..seat_map[row].len() {

                if seat_map[row][col] == '#' {
                    seat_count = seat_count + 1;
                }
                print!("{}", seat_map[row][col] );
            }

        println!();
    }

    seat_count
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}