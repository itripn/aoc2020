use regex::Regex;
use std::fs;

fn main() {

    let contents = fs::read_to_string("input.txt").expect("Error");
    let tickets: Vec<&str> = contents.split('\n').collect();
    let mut exploded_ranges : Vec<i32> = Vec::new();
    let _my_ticket : Vec<i32>;
    let mut their_tickets : Vec<Vec<i32>> = Vec::new();

    let all_tickets = &tickets.clone().into_iter()
        .filter(|s| s.starts_with(|p:char| p.is_digit(10) )  )
        .collect::<Vec<&str>>();

    let all_rules = &tickets.into_iter()
        .filter(|s| !s.starts_with(|p:char| p.is_digit(10) ) && !s.ends_with(":") && s.len()!=0 )
        .collect::<Vec<&str>>();
    
    explode_ranges( &all_rules, &mut exploded_ranges );

    _my_ticket = all_tickets
        .first()
        .unwrap()
        .split(",")
        .map(|n| n.parse::<i32>().unwrap() )
        .collect();

    for t in &all_tickets[1..] {
        let nt = t.split(',').map(|n| n.parse().unwrap()).collect();
        their_tickets.push( nt );
    }

    let total : i32 = their_tickets
        .into_iter()
        .flatten()
        .collect::<Vec<i32>>()
        .into_iter()
        .filter(|n| !exploded_ranges.contains(n))
        .sum();

    println!("\nTotal scanning error rate: {}\n", total);
}

fn explode_ranges( all_rules : &Vec<&str>, exploded_ranges : &mut Vec<i32> ) {

    let ranges_re = Regex::new(r"^(.*): ([0-9]*)-([0-9]*) or ([0-9]*)-([0-9]*)").unwrap();
    
    for rule in all_rules {

        let caps = ranges_re.captures(rule);
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
}