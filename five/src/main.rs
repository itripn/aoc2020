use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

fn main() {

    let mut boardingpasses : Vec<String> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String

        let mut str_buf : String = String::new();

        for line in lines {

            if let Ok(bpline) = line {

            }
        }
    }

}
