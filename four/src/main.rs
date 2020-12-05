
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;

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

    ////////////////////////////////////////////////////////////////
    ///// FIXME -- code is counting ONE extra passport as valid! ///
    ////////////////////////////////////////////////////////////////
    let valid_passports = count_valid_passports( &ppdata ) ;
    println!( "Valid passports: {}", valid_passports ); 
}


// byr (Birth Year) - four digits; at least 1920 and at most 2002.
// iyr (Issue Year) - four digits; at least 2010 and at most 2020.
// eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
// hgt (Height) - a number followed by either cm or in:
// If cm, the number must be at least 150 and at most 193.
// If in, the number must be at least 59 and at most 76.
// hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
// ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
// pid (Passport ID) - a nine-digit number, including leading zeroes.
// cid (Country ID) - ignored, missing or not.

fn count_valid_passports( passports : &Vec<String> ) -> i32 {

    let mut valid_count = 0;

    for passport in passports {

        let byr = passport.contains( "byr:" ) && validate_year( &passport, "byr:", (1920, 2002) );
        let iyr = passport.contains( "iyr:" ) && validate_year( &passport, "iyr:", (2010, 2020) );
        let eyr = passport.contains( "eyr:" ) && validate_year( &passport, "eyr:", (2020, 2030) );
        let hgt = passport.contains( "hgt:" ) && validate_height( &passport, (150,193,59,76) );
        let hcl = passport.contains( "hcl:" ) && validate_hair_color( &passport );
        let ecl = passport.contains( "ecl:" ) && validate_eye_color( &passport );
        let pid = passport.contains( "pid:" ) && validate_pid( &passport );
        
        // println!( "{} {} {} {} {} {} {}", byr, iyr, eyr, hgt, hcl, ecl, pid );
        if byr && iyr && eyr && hgt && hcl && ecl && pid {
            valid_count = valid_count + 1;
        }
        else {
            // println!( "bad: {}\n", passport );
        }
    }

    valid_count
}

fn validate_pid( passport : &String ) -> bool {

    let re = Regex::new(r"pid:(\d{9})").unwrap();
    let cap = re.captures( passport );

    match cap {

        Some(_c) => {
            // println!( "pid: {}", c.get(1).unwrap().as_str() );
            true
        },

        None => { /*println!("bad pp: {}", passport );*/ false }
    }
}

fn validate_hair_color( passport : &String ) -> bool {

    let re = Regex::new( r"hcl:(\#[0-9a-f]{6})" ).unwrap();
    let cap = re.captures( passport );
    match cap {
        Some(_c) => { /*println!( "hcl: {}", c.get(1).unwrap().as_str() );*/ return true; },
        None => { /*println!("bad hcl: {}", passport);*/ false }
    }
}

fn validate_eye_color( passport : &String ) -> bool {

    let re = Regex::new( r"ecl:(amb|blu|brn|gry|grn|hzl|oth)" ).unwrap();
    let cap = re.captures( passport );
    match cap {
        Some(_c) => { /*println!( "ecl: {}", _c.get(1).unwrap().as_str() );*/ return true; },
        None => { /*println!("bad ecl: {}", passport );*/ false}
    }
}

fn validate_height( passport : &String, bounds : (i32,i32,i32,i32) ) -> bool {

    let re = Regex::new( r"hgt:(\d{2,3})(cm|in)" ).unwrap();
    let cap = re.captures( passport );
    match cap {

        Some(c) => { 

            let hgt_str = c.get(1).unwrap().as_str();
            let unit_str = c.get(2).unwrap().as_str();

            let hgt = hgt_str.parse::<i32>().unwrap();
            let good_height =  match unit_str {

                "cm" =>  hgt >= bounds.0 && hgt <= bounds.1,
                "in" =>  hgt >= bounds.2 && hgt <= bounds.3,
                _ => return false
            };

            // println!("height: {}", good_height );
            if !good_height {
                println!("bad hgt: {}\n", passport );
            }

            good_height
        },
        None => false
    }
}

fn get_regex( field : &str ) -> &str {

    match field {

        "iyr:" => r"iyr:(\d{4})",
        "eyr:" => r"eyr:(\d{4})",
        "byr:" => r"byr:(\d{4})",
        _ => ""
    }
}

fn validate_year( passport : &String, field : &str, bounds : (i32, i32) ) -> bool {

    let re = Regex::new(get_regex( field ) ).unwrap();
    let cap = re.captures( passport );
    match cap {

        Some(c) => { 

            let year_str = c.get(1).unwrap().as_str();
            let year = year_str.parse::<i32>().unwrap();

            let good_year = year >= bounds.0 && year <= bounds.1;
            // println!("year: {}, ({} - {},{})", good_year, year, bounds.0, bounds.1 );

            good_year
        },

        None => false
    }
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}