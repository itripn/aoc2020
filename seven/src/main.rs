
use std::{fs::File};
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;
use std::collections::HashSet;
#[macro_use] extern crate lazy_static;


// Compile our regex once and as late as possible
//
lazy_static! {
    static ref RE : Regex = Regex::new(r"(\b[^\s]+\b \b[^\s]+\b) bags contain (.*)").unwrap();
}

fn main() {

    let mut raw_data : Vec<String> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {

        // Consumes the iterator, returns an (Optional) String
        for line in lines {

            // Collapse all lines for a group into a single line
            if let Ok(l) = line {
                raw_data.push( l );
            }
        }
    }

    let bag_name = "shiny gold";
    let mut bags : HashSet<String> = HashSet::new();

    find_containers( &raw_data, &bag_name, &mut bags );
    println!("{} bags can hold {} bags", bags.len(), bag_name );
}

// Recursive: for each bag that can hold the passed in bag, find the bags that can hold
// it.
//
// This is likely extremely inefficient, needs work. But gets the job done for now
//
fn find_containers( raw_data : &Vec<String>, bag_name : &str, bags : &mut HashSet<String> ) {

    for line in raw_data { 

        let cap = RE.captures( line );
        match cap {
            Some(c) => {
                let trailer = c.get(2).unwrap().as_str();
                if trailer.find( bag_name ) != None {
                    let new_bag_name = c.get(1).unwrap().as_str();

                    if !bags.contains( new_bag_name ) {
                        // println!("{}: can contain {} bags", new_bag_name, bag_name );
                        bags.insert( new_bag_name.to_string() );

                        find_containers( raw_data, &new_bag_name, bags );
                    }
                }
            },
            None => (),
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