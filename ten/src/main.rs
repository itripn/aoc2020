use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {

    let mut data : Vec<i32> = Vec::new();

    // Read lines into a vector
    //
    if let Ok(lines) = read_lines("./input.txt") {

        // Consumes the iterator, returns an (Optional) String
        for line in lines {

            // Collapse all lines for a group into a single line
            if let Ok(l) = line {
                if l.len() > 0 {
                    data.push( l.parse::<i32>().unwrap() );
                }
            }
        }
    }

    data.sort();

    let mut ones = 1;
    let mut threes = 1;

    for i in 0..data.len()-1 {

        let cur = data[i];
        let nxt = data[i+1];

        if nxt - cur == 1 {
            ones = ones + 1;
        }
        else if nxt - cur == 3 {
            threes = threes + 1;
        }
        else {
            panic!("got bad delta!");
        }
    }

    println!("Ones count {} * Threes count {} = {}", ones, threes, ones * threes );
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}



// Part 1 was nothing fancy:
// fn part1(adaptors: &[u64]) -> Result<u64> {
//     let [_, ones, _, threes] =
//         adaptors
//             .windows(2)
//             .map(|pair| pair[1] - pair[0])
//             .fold([1; 4], |mut counts, it| {
//                 counts[it as usize] += 1;
//                 counts
//             });
//     Ok(ones * threes)
// }
// Part 2 gave me a lot of trouble. I spent an hour trying to figure out why my solution wouldn't work, only to realise that I'd forgot to include the starting 0. Can't say I was best pleased...
// fn part2_search(adaptors: &[u64], db: &mut HashMap<u64, u64>) -> u64 {
//     match adaptors.split_first() {
//         Some((_, [])) => 1,
//         Some((first, rest)) => rest
//             .iter()
//             .take_while(|a| *a - first <= 3)
//             .enumerate()
//             .map(|(idx, val)| {
//                 db.get(val).copied().unwrap_or_else(|| {
//                     let sub_count = part2_search(&rest[idx..], db);
//                     *db.entry(*val).or_insert(sub_count)
//                 })
//             })
//             .sum(),
//         None => 0, // Shouldn't get an empty list, but just in case...
//     }
// }





// Part 1
//
// let mut numbers: Vec<u64> = lines_from_file("inputs/day10_1.txt").iter().map(|x|     x.parse::<u64>().unwrap()).collect();
// let mut counts = HashMap::new();
// numbers.push(0u64);
// numbers.push(*numbers.iter().max().unwrap() + 3);

// numbers.sort();
// // println!("{:#?}", numbers);

// let mut niter = numbers.iter();
// let mut source: u64 = *niter.next().unwrap();
// for adapter in niter {
//     let step = counts.entry(adapter-source).or_insert(0);
//     *step += 1;
//     source = *adapter;
//     // println!("{:#?}", counts);
// }
// (counts.get(&1).unwrap() * (counts.get(&3).unwrap())).to_string()


// part 2
//
// pub fn n_paths_to_end(rest: &[u64], cache: &mut     HashMap<u64,u64>) -> u64 {
//     println!("{:?}", rest);
//     if cache.contains_key(rest.first().unwrap() )     {
//         println!("\tPaths Cache Hit {:?} =>     {:?}", rest, *cache.get(    rest.first().unwrap()).unwrap());
//         return *cache.get(    rest.first().unwrap()).unwrap();
//     }
//     else {
//         if rest.len() == 1 {
//             println!("\tPaths {:?} => 1", rest);
//             cache.insert(*rest.first().unwrap(),     1);
//             return 1;
//         } else {
//             let mut count = 0u64;
//             let mut niter = rest.iter();
//             niter.next();
//             for (i, next) in niter.enumerate() {
//                 if next-rest.first().unwrap() <=     3 {
//                     let cn =     n_paths_to_end(&rest[(i +     1)..], cache);
//                     count += cn;
//                 } else {
//                     break;
//                 }
//             }
//             cache.insert(*rest.first().unwrap(),     count);
//             println!("\tPaths {:?} => {:?}",     rest, count);
//             return count;
//         }
//     }
// }