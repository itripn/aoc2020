use std::i32::MAX;

fn main() {

    // Input is small enough not gonna bother with reading it from a file
    //
    let bus_line_str = "19,x,x,x,x,x,x,x,x,41,x,x,x,x,x,x,x,x,x,523,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,17,13,x,x,x,x,x,x,x,x,x,x,29,x,853,x,x,x,x,x,37,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,x,23".split( ',' );
    let earliest_time = 1000510;

    // let (bus_id, min_wait) = lines[1]
    //     .split(',')
    //     .filter(|s| s != &"x")
    //     .map(|id| id.parse::<usize>().unwrap())
    //     .map(|id| (id, id - (earliest_timestamp % id)))
    //     .min_by_key(|pair| pair.1)
    //     .unwrap();
    //
    // bus_id * min_wait

    // pull the busses out, getting rid of the x's and converting to integers
    //
    let busses : Vec<i32> = bus_line_str
            .filter(|&b| b != "x" )
            .map(|b| b.parse::<i32>().unwrap())
            .collect();

    // get the range of times based on our input start time -- give a little room on the end to find
    // the right time... TODO: should just add max(bus line #) here.
    //
    let times : Vec<i32> = ((earliest_time)..(earliest_time + 100)).collect();

    // Show a visual representation of the schedule
    //
    dump_time_table(&times, &busses);
    
    // Find the earliest time after the start time and return that time, and the line
    // which you would take.
    //
    let (time,line) = find_earliest_time( &times, &busses );
    println!("({},{}) = {}", time-earliest_time, line, line * (time-earliest_time));
}

// This function finds the earliest time after the first time which we can take,
// and returns that time along with the line that leaves at that time
//
fn find_earliest_time( times : &Vec<i32>, busses : &Vec<i32> ) -> ( i32, i32 ) {

    let mut best_time = MAX;
    let mut best_line = 0;

    // Loop over all the busses, then for each time figure out there is a 
    // departure at that time. Keep track of the earliest time we can take,
    // and which line leaves at that time. Those two are our return values
    //
    for b in busses {
        for t in times {
            if t % b == 0 {
                if *t < best_time { // current best time and it's line
                    best_time = *t; 
                    best_line = *b; 
                }
            }
        }
    }

    (best_time, best_line)
}

// Show a visual representation of the bus schedule
//
fn dump_time_table( times : &Vec<i32>, busses : &Vec<i32> ) {

    println!("\n\n");
    busses.iter().for_each(|b| print!("\t{}", b) );
    println!();
    for t in times {

        print!("{}\t", t);

        busses.iter().for_each(|b| {
            if t % b == 0 {
                print!("D\t")
            }
            else {
                print!(".\t")
            }
        });

        println!();
    }
}