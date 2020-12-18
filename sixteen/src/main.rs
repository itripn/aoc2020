use regex::Regex;
use std::fs;

fn main() {

    let contents = fs::read_to_string("input.txt").expect("Error");
    let tickets: Vec<&str> = contents.split('\n').collect();
    let mut exploded_ranges : Vec<i32> = Vec::new();
    let mut my_ticket_next = false;
    let mut their_tickets_next = false;
    let mut my_ticket : Vec<i32> = Vec::new();
    let mut their_tickets : Vec<Vec<i32>> = Vec::new();

    let ranges_re = Regex::new(r"^(.*): ([0-9]*)-([0-9]*) or ([0-9]*)-([0-9]*)").unwrap();

    for line in tickets {

        if my_ticket_next {
            my_ticket.extend( line.split(",").map( |x| x.parse::<i32>().unwrap() ) );
            my_ticket_next = false;
            continue;
        }

        if their_tickets_next {
            if line.len() > 0 {
                let ticket : Vec<i32> = line.split(",").map( |x| x.parse::<i32>().unwrap() ).collect();
                their_tickets.push( ticket );
                continue;
            }
            else {
                break;
            }
        }


        if line.starts_with( "nearby tickets:" ) {
            their_tickets_next = true;
            continue;
        }

        if line.starts_with( "your ticket:" ) {
            my_ticket_next = true;
            continue;
        }        


        let caps = ranges_re.captures(line);
        match caps {

            Some(_c) => {

                let rb1 = _c.get(2).unwrap().as_str().parse::<i32>().unwrap();
                let re1 = _c.get(3).unwrap().as_str().parse::<i32>().unwrap();
                let rb2 = _c.get(4).unwrap().as_str().parse::<i32>().unwrap();
                let re2 = _c.get(5).unwrap().as_str().parse::<i32>().unwrap();

                exploded_ranges.extend( rb1..=re1 );
                exploded_ranges.extend( rb2..=re2 );

            },

            None => continue
        }
    }

    println!("{} tickets", their_tickets.len() );

    let total : i32 = their_tickets
        .into_iter()
        .flatten().collect::<Vec<i32>>()
        .into_iter()
        .filter(|n| !exploded_ranges.contains(n))
        .sum();

    println!("\nTotal scanning error rate: {}", total);
}
