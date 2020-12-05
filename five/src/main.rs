use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut boardingpasses : Vec<String> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {

        // Consumes the iterator, returns an (Optional) String
        for line in lines {

            if let Ok(bpline) = line {
                boardingpasses.push( bpline );
            }
        }
    }

    let mut seats = Vec::<(i32,i32)>::new();
    for pass in boardingpasses {

        // println!( "looking at: {}", pass );
        let seat = find_seat( &pass );
        seats.push( seat );
    }

    let mut seat_ids = Vec::<i32>::new();

    for seat in seats {

        seat_ids.push( seat.0 * 8 + seat.1 );
    }

    println!("Largest seat id: {}", seat_ids.iter().max().unwrap() );
    seat_ids.sort();

    let mut last_seat_id = seat_ids[0] - 1;
    for seat_id in seat_ids {

        if seat_id - last_seat_id > 1 {
            println!("Your seat is: {}", seat_id -1 );
        }
        
        last_seat_id = seat_id;
    }
}


fn find_seat( pass : &String ) -> (i32, i32) {

    let mut row_min = 0;
    let mut row_max = 127;
    let mut seat_min = 0;
    let mut seat_max = 7;

    for instr in pass.chars() {

         match instr {

            'F' => row_max = row_min + ( row_max - row_min ) / 2 ,
            'B' => row_min = row_min + ( row_max - row_min ) / 2 + 1,
            'L' => seat_max = seat_min + ( seat_max - seat_min ) / 2 ,
            'R' => seat_min = seat_min + ( seat_max - seat_min ) / 2 + 1,
            _ => println!("invalid bp code")
        }
        
        // println!("({},{}), ({},{})", row_min, row_max, seat_min, seat_max );
    }

    assert_eq!( row_max, row_min );
    assert_eq!( seat_max, seat_min);

    (row_min, seat_min)
}



// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}