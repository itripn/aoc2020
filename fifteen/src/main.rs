
use std::collections::HashMap;

fn main() {

    let mut starting_numbers : Vec<i32> = vec![0,3,6]; //vec![1,0,18,10,19];
    let mut numbers : Vec<i32> = Vec::new();

    numbers.append( &mut starting_numbers );

    parta( numbers );
}

fn parta( numbers : Vec<i32> ) {

    let mut cur_idx = 1;
    let mut last_spoken;
    let mut seen_numbers : HashMap<i32,usize> = HashMap::new();

    for n in numbers.iter().take(numbers.len()-1) {
        println!("{}: {}", cur_idx, n);
        seen_numbers.insert(*n, cur_idx );
        cur_idx += 1;
    }
    last_spoken = *numbers.last().unwrap();

    println!("-----");
    loop {

        println!("Turn: {}: {}", cur_idx, last_spoken);

        // println!("{}: speaking {}", cur_idx, last_spoken );
        if seen_numbers.contains_key( &last_spoken ) {

            let lidx = seen_numbers.get( &last_spoken ).unwrap();
            last_spoken = ( cur_idx - lidx ) as i32;
        }
        else {
            seen_numbers.insert( last_spoken, cur_idx );
            last_spoken = 0;
        }

        cur_idx += 1;
        if cur_idx > 2020 {
            break;
        }
    }

    println!("{}", last_spoken);
}
