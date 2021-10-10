use regex::Regex;
use std::fs;
use std::collections::HashSet;

#[macro_use] extern crate lazy_static;

lazy_static! {

    static ref RE_ALLERGEN : Regex = Regex::new(r".*\(contains (.*)\)$").unwrap();
    static ref RE_INGRED : Regex = Regex::new(r"(.*)\(contains (.*)\)$").unwrap();

}

fn main() {

    let contents = fs::read_to_string("testinput.txt").expect("Error");
    let ingredients: Vec<&str> = contents.split('\n').collect();

    let allergens : Vec<String> = ingredients
        .iter()
        .map( |i| {

            let caps = RE_ALLERGEN.captures( i );
            match caps {

                Some(_c) => {
                    String::from (_c.get(1).unwrap().as_str())
                },
                
                _ => String::from( "" )
            }
        })
        .collect();

        let mut uniques : HashSet<&str> = HashSet::new();

        for a in &allergens {
            let pieces : Vec<&str> = a.split(", ").collect();
            for p in pieces {
                if p.len() > 0 { uniques.insert(p); }
            }
        }

        for i in ingredients {

            let caps = RE_INGRED.captures(i);
            match caps {
                Some(_c) => {
                    println!("{}", _c.get(1).unwrap().as_str());
                    println!("{}", _c.get(2).unwrap().as_str());
                },

                _ => ()

            }
        }
}