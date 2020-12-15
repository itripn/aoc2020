use regex::Regex;
use std::fs;

fn main() {

    let contents = fs::read_to_string("input.txt").expect("Error");
    let program: Vec<&str> = contents.split('\n').collect();
    
    
    parta(&program);
    
}

fn parta(program: &Vec<&str>) {

    let mut memory: Vec<u64> = vec![0; 64 * 1024];
    let mut current_mask = String::new();
    let mask_re = Regex::new(r"mask = (.*)$").unwrap();
    let mem_re = Regex::new(r"mem\[(\d+)\] = (\d+)$").unwrap();

    program.iter().for_each(|&instr| {
        
        let mask_cap = mask_re.captures(instr);

        match mask_cap {
            Some(_c) => {
                current_mask = _c.get(1).unwrap().as_str().to_string();
            }

            None => {
                // not a mask, must be a store instruction

                let mem_cap = mem_re.captures(instr);

                match mem_cap {
                    Some(_c) => {
                        let addr = _c.get(1).unwrap().as_str().parse::<usize>().unwrap();
                        let value = _c.get(2).unwrap().as_str().parse::<u64>().unwrap();

                        store_value_with_mask(&mut memory, addr, value, &current_mask);
                        // println!("Store {} into {}", _c.get(2).unwrap().as_str(), _c.get(1).unwrap().as_str());
                    }

                    None => (),
                }
            }
        }
    });

    println!("Sum: {}", memory.iter().sum::<u64>());
}

fn store_value_with_mask(memory: &mut Vec<u64>, addr: usize, value: u64, mask: &str) {
    let mut val_digits: Vec<char> = format!("{:0>36b}", value).chars().collect();
    let mask_digits: Vec<char> = mask.chars().collect();

    for idx in 0..mask_digits.len() {
        if mask_digits[idx] == 'X' {
            continue;
        } else {
            val_digits[idx] = mask_digits[idx];
        }
    }

    let new_value_str: String = val_digits.into_iter().collect::<String>();
    let new_value = u64::from_str_radix(&new_value_str, 2).unwrap();

    memory[addr] = new_value;
}
