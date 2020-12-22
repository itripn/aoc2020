
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use regex::Regex;


#[derive(Debug)]
struct Passport {

    pps: String,
    ecl : Option<String>,
    byr : Option<String>,
    iyr : Option<String>,
    eyr : Option<String>,
    hgt : Option<(String,String)>,
    hcl : Option<String>,
    pid : Option<String>,
    _cid : Option<String>
}

impl Passport {

    // Parse a passport from the raw text line
    // into our structure fields. 
    //
    fn parse_passport( pp : &String ) -> Self {

        Passport {
            pps: pp.clone(),
            pid: parse_pid( &pp ),
            ecl: parse_eye_color( &pp ),
            byr: parse_year( &pp, "byr:" ),
            iyr: parse_year( &pp, "iyr:" ),
            eyr: parse_year( &pp, "eyr:"),
            hgt: parse_height( &&pp ),
            hcl: parse_hair_color( &pp ),
            _cid: None,
        }
    }

    // Do a quick sanity check that the required 
    // fields exist -- does NOT check to see if
    // they have the right values. See 
    // fn deep_valid() below
    //
    fn shallow_valid( &self ) -> bool {

         self.pps.contains( "byr:" ) &&
         self.pps.contains( "iyr:" ) &&
         self.pps.contains( "eyr:" ) &&
         self.pps.contains( "hgt:" ) &&
         self.pps.contains( "hcl:" ) &&
         self.pps.contains( "ecl:" ) &&
         self.pps.contains( "pid:" )

    }

    // Validate our fields have some data in them, 
    // helps with deep_valid() below.
    //
    fn valid( &self ) -> bool {

        return  self.byr != None &&
                self.iyr != None &&
                self.eyr != None &&
                self.hgt != None &&
                self.hcl != None &&
                self.ecl != None &&
                self.pid != None;
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
    //
    // Check values and make sure they're within range.
    //
    fn deep_valid( &self ) -> bool {

        if !self.valid() {
            return false;
        }

        if !self.shallow_valid() {
            return false;
        }

        // let byrt = self.byr.clone();
        let byr = self.byr.clone().unwrap().parse::<i32>().unwrap().clone();
        if byr < 1920 || byr > 2002 {
            return false;
        }

        let iyr = self.iyr.clone().unwrap().parse::<i32>().unwrap();
        if iyr < 2010 || iyr > 2020 {
            return false;
        }

        let eyr = self.eyr.clone().unwrap().parse::<i32>().unwrap();
        if eyr < 2020 || eyr > 2030 {
            return false;
        }

        if self.pid == None || self.pid.clone().unwrap().len() != 9 {
            return false;
        }

        let unit = self.hgt.clone().unwrap().1; // unit
        let val = self.hgt.clone().unwrap().clone().0.parse::<i32>().unwrap();
        match &unit[..] {

            "cm" => {
                if val < 150 || val > 193 {
                    return false;
                }
            },

            "in" => {
                if val < 59 || val > 76 {
                    return false;
                }
            },
            _ => return false
        }

        true
    }
}

fn main() {

    let ppdata = parse_input( "./input1.txt" );

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
            //.inspect(|p| println!("{:?}", p))
            .map( |p| if p.shallow_valid() { 1 } else { 0 } )
            .sum();
}


fn count_valid_passports_and_field_ranges( passports : &Vec<Passport> ) -> i32 {

        return passports
            .iter()
            //.inspect(|p| println!("{:?}", p))
            .map( |p| if p.deep_valid() { 1 } else { 0 } )
            .sum();

}

fn parse_pid( passport : &String ) -> Option<String> {

    let re = Regex::new(r"pid:(\d{9})").unwrap();
    let cap = re.captures( passport );

    match cap {

        Some(_c) => {
            Some( String::from( _c.get(1).unwrap().as_str() ) )
        },

        None => { /*println!("bad pid: {}", passport);*/ None }
    }
}

fn parse_hair_color( passport : &String ) -> Option<String> {

    let re = Regex::new( r"hcl:(\#[0-9a-f]{6})" ).unwrap();
    let cap = re.captures( passport );

    match cap {

        Some(_c) => { Some( String::from( _c.get(1).unwrap().as_str() ) ) },

        None => { /*println!("bad hcl: {}", passport);*/ None }
    }
}

fn parse_eye_color( passport : &String ) -> Option<String> {

    let re = Regex::new( r"ecl:(amb|blu|brn|gry|grn|hzl|oth)" ).unwrap();
    let cap = re.captures( passport );

    match cap {

        Some(_c) => { Some( String::from( _c.get(1).unwrap().as_str() ) ) },
        
        None => { /*println!("bad ecl: {}", passport);*/ None }
    }
}

fn parse_height( passport : &String ) -> Option<(String, String)> {

    let re = Regex::new( r"hgt:(\d{2,3})(cm|in)" ).unwrap();
    let cap = re.captures( passport );

    match cap {

        Some(c) => { 

            let hgt_str = c.get(1).unwrap().as_str();
            let unit_str = c.get(2).unwrap().as_str();

            Some( (String::from(hgt_str), String::from(unit_str) ) )

        },

        None => { /*println!("bad hgt: {}", passport);*/ None }
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

fn parse_year( passport : &String, field : &str ) -> Option<String> {


    let re = Regex::new(get_regex( field ) ).unwrap();
    let cap = re.captures( passport );

    match cap {

        Some(_c) => { 
            Some( String::from( _c.get(1).unwrap().as_str() ) )
        },

        None => { /*println!("bad {} {}", field, passport);*/ None }
    }
}


// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}