use regex::Regex;

#[derive(Debug)]
pub struct Passport {
    pps: String,
    ecl: Option<String>,
    byr: Option<String>,
    iyr: Option<String>,
    eyr: Option<String>,
    hgt: Option<(String, String)>,
    hcl: Option<String>,
    pid: Option<String>,
    _cid: Option<String>,
}

impl Passport {

    // Parse a passport from the raw text line
    // into our structure fields.
    //
    pub fn parse_passport(pp: &String) -> Self {
        Passport { pps: pp.clone(),
                   pid: Passport::parse_pid(&pp),
                   ecl: Passport::parse_eye_color(&pp),
                   byr: Passport::parse_year(&pp, "byr:"),
                   iyr: Passport::parse_year(&pp, "iyr:"),
                   eyr: Passport::parse_year(&pp, "eyr:"),
                   hgt: Passport::parse_height(&&pp),
                   hcl: Passport::parse_hair_color(&pp),
                   _cid: None }
    }

    // Do a quick sanity check that the required
    // fields exist -- does NOT check to see if
    // they have the right values. See
    // fn deep_valid() below
    //
    pub fn shallow_valid(&self) -> bool {
        self.pps.contains("byr:")
        && self.pps.contains("iyr:")
        && self.pps.contains("eyr:")
        && self.pps.contains("hgt:")
        && self.pps.contains("hcl:")
        && self.pps.contains("ecl:")
        && self.pps.contains("pid:")
    }

    // Validate our fields have some data in them,
    // helps with deep_valid() below.
    //
    pub fn all_fields_present(&self) -> bool {
        return self.byr != None
               && self.iyr != None
               && self.eyr != None
               && self.hgt != None
               && self.hcl != None
               && self.ecl != None
               && self.pid != None;
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
    pub fn deep_valid(&self) -> bool {
        if !self.all_fields_present() || !self.shallow_valid() {
            return false;
        }

        let byr = self.byr.clone().unwrap().parse::<i32>().unwrap();
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
            }

            "in" => {
                if val < 59 || val > 76 {
                    return false;
                }
            }
            _ => return false,
        }

        true
    }

    fn parse_pid(passport: &String) -> Option<String> {
        let re = Regex::new(r"pid:(\d{9})\b").unwrap();
        let cap = re.captures(passport);

        match cap {
            Some(_c) => Some(String::from(_c.get(1).unwrap().as_str())),

            None => {
                /*println!("bad pid: {}", passport);*/
                None
            }
        }
    }

    fn parse_hair_color(passport: &String) -> Option<String> {
        let re = Regex::new(r"hcl:(\#[0-9a-f]{6})\b").unwrap();
        let cap = re.captures(passport);

        match cap {
            Some(_c) => Some(String::from(_c.get(1).unwrap().as_str())),

            None => {
                /*println!("bad hcl: {}", passport);*/
                None
            }
        }
    }

    fn parse_eye_color(passport: &String) -> Option<String> {
        let re = Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\b").unwrap();
        let cap = re.captures(passport);

        match cap {
            Some(_c) => Some(String::from(_c.get(1).unwrap().as_str())),

            None => {
                /*println!("bad ecl: {}", passport);*/
                None
            }
        }
    }

    fn parse_height(passport: &String) -> Option<(String, String)> {
        let re = Regex::new(r"hgt:(\d{2,3})(cm|in)\b").unwrap();
        let cap = re.captures(passport);

        match cap {
            Some(c) => {
                let hgt_str = c.get(1).unwrap().as_str();
                let unit_str = c.get(2).unwrap().as_str();

                Some((String::from(hgt_str), String::from(unit_str)))
            }

            None => {
                /*println!("bad hgt: {}", passport);*/
                None
            }
        }
    }

    fn get_regex(field: &str) -> &str {
        match field {
            "iyr:" => r"iyr:(\d{4})\b",
            "eyr:" => r"eyr:(\d{4})\b",
            "byr:" => r"byr:(\d{4})\b",
            _ => "",
        }
    }

    fn parse_year(passport: &String, field: &str) -> Option<String> {
        let re = Regex::new(Passport::get_regex(field)).unwrap();
        let cap = re.captures(passport);

        match cap {
            Some(_c) => Some(String::from(_c.get(1).unwrap().as_str())),

            None => {
                /*println!("bad {} {}", field, passport);*/
                None
            }
        }
    }
}
