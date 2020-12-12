use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Direction {

    direction : char,
    distance : i32
}

fn main() {

    let mut directions : Vec<Direction> = Vec::new();

    // Read lines into a vector
    //
    if let Ok(lines) = read_lines("./input.txt") {

        // Consumes the iterator, returns an (Optional) String
        for line in lines {

            if let Ok(l) = line {
                if l.len() > 0 {

                    directions.push(  

                        Direction {

                            direction : l.chars().nth(0).unwrap(),
                            distance : l[1..].to_string().parse::<i32>().unwrap()
                        }

                    );
                }
            }
        }
    }

    follow_directions( &directions );
}

fn follow_directions( directions : &Vec<Direction> ) {

    let mut northsouth = 0;
    let mut eastwest = 0;
    let mut curdirection = 0;

    for dir in directions {

        match dir.direction {

            'F' => match curdirection {

                0 => eastwest = eastwest + dir.distance, // east
                90 => northsouth = northsouth - dir.distance, // south
                180 => eastwest = eastwest - dir.distance, // west
                270 => northsouth = northsouth + dir.distance, // north
                _ => ()
            },

            'S' => northsouth = northsouth - dir.distance,
            'N' => northsouth = northsouth + dir.distance,
            'E' => eastwest = eastwest + dir.distance,
            'W' => eastwest = eastwest - dir.distance,

            'L' => {
                curdirection = curdirection - dir.distance;
                if curdirection < 0 {

                    
                }
                else if curdirection > 360 {
                    curdirection = curdirection % 360;
                }
            },
            'R' => {
                
            },

            _ => ()
        }
    }

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}