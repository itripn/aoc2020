
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

// ecl:gry pid:860033327 eyr:2020 hcl:#fffffd       // Valid
// byr:1937 iyr:2017 cid:147 hgt:183cm
//
// iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884  // Invalid
// hcl:#cfa07d byr:1929
//
// hcl:#ae17e1 iyr:2013                             // Valid
// eyr:2024
// ecl:brn pid:760753108 byr:1931
// hgt:179cm
//
// hcl:#cfa07d eyr:2025 pid:166559648               // Invalid
// iyr:2011 ecl:brn hgt:59in

fn main() {

    let mut ppdata : Vec<String> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {
        // Consumes the iterator, returns an (Optional) String

        let mut str_buf : String = String::new();

        for line in lines {

            if let Ok(ppline) = line {

                if ppline.len() > 0 {
                    str_buf.push_str( &(ppline + " ") );
                }
                else {
                    // println!( "{}", str_buf );
                    ppdata.push( str_buf.clone() );
                    str_buf.clear();
                }
            }
        }
        // if there isn't a blank line at the end of the input, we'll likely have a buffer
        // still to process
        //
        if str_buf.len() > 0 {
            ppdata.push( str_buf.clone() );
        }
    }

    let valid_passports = count_valid_passports( &ppdata ) ;
    println!( "Valid passports: {}", valid_passports ); 
}

// byr (Birth Year)
// iyr (Issue Year)
// eyr (Expiration Year)
// hgt (Height)
// hcl (Hair Color)
// ecl (Eye Color)
// pid (Passport ID)
// cid (Country ID)
// 

fn count_valid_passports( passports : &Vec<String> ) -> i32 {

    let mut valid_count = 0;

    for passport in passports {

        let byr = passport.contains( "byr:" );
        let iyr = passport.contains( "iyr:" );
        let eyr = passport.contains( "eyr:" );
        let hgt = passport.contains( "hgt:" );
        let hcl = passport.contains( "hcl:" );
        let ecl = passport.contains( "ecl:" );
        let pid = passport.contains( "pid:" );
        
        if byr && iyr && eyr && hgt && hcl && ecl && pid {
            valid_count = valid_count + 1;
        }
    }

    valid_count
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}