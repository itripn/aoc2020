
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
#[macro_use] extern crate lazy_static;

mod passport;
use passport::*;

fn main() {

    let ppdata = parse_input( "./input.txt" );

    let parta_results = count_valid_passports( &ppdata );
    println!( "Part A valid passports: {}", parta_results ); 
    
    ////////////////////////////////////////////////////////////////
    ///// FIXME -- code is counting ONE extra passport as valid! ///
    ////////////////////////////////////////////////////////////////
    //
    let partb_results = count_valid_passports_and_field_ranges( &ppdata ) ;
    println!( "Part B valid passports: {}", partb_results ); 
}

// Parse the input into single line passports that can then be parsed
// by the Passport object. 
//
fn parse_input( filename : &str ) -> Vec<Passport> {

    let mut ppdata : Vec<Passport> = Vec::new();

    if let Ok(lines) = read_lines( filename ) {

        // Consumes the iterator, returns an (Optional) String

        let mut str_buf : String = String::new();

        for line in lines {

            if let Ok(ppline) = line {

                if ppline.len() > 0 {
                    str_buf.push_str( &(ppline + " ") );
                }
                else {
                    ppdata.push( Passport::parse_passport( &str_buf ) );
                    str_buf.clear();
                }
            }
        }

        // if there isn't a blank line at the end of the input, we'll likely have a buffer
        // still to process
        //
        if str_buf.len() > 0 {
            ppdata.push( Passport::parse_passport( &str_buf ) );
        }
    }

    ppdata
}

fn count_valid_passports( passports : &Vec<Passport> ) -> i32 {

    return passports
            .iter()
            .map( |p| if p.shallow_valid() { 1 } else { 0 } )
            .sum();
}


fn count_valid_passports_and_field_ranges( passports : &Vec<Passport> ) -> i32 {

        return passports
            .iter()
            .map( |p| if p.deep_valid() { 1 } else { 0 } )
            .sum();
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}