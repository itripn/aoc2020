use std::{fs::File};
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, Clone)]
struct Instruction {

    opcode : String,
    operand : i32,
    been_seen : bool
}

fn main() {

    let mut program : Vec<Instruction> = Vec::new();

    if let Ok(lines) = read_lines("./input.txt") {

        // Consumes the iterator, returns an (Optional) String
        for line in lines {

            // Collapse all lines for a group into a single line
            if let Ok(l) = line {

                if l.len() > 0 {
                    let words : Vec<&str> = l.split(" ").collect();

                    program.push( 
                        Instruction { 
                            opcode: words[0].to_string(),
                            operand: words[1].parse::<i32>().unwrap(),
                            been_seen: false 
                        } 
                    );
                }
            }
        }
    }

    for i in 0..program.len() {

        // FIXME: got lucky and it was a jmp instruction -- need 
        // to check for nop in case input is different. HACK.
        //
        if program[i].opcode == "jmp" {  // for each jmp, change it to a nop and run the program

            program[i].opcode = "nop".to_string();
            let (accumulator, finished) = execute_until_infinite_or_finished( &mut program );
            if finished {
                println!("Instruction changed: {}", i );
                println!("Final accumulator: {}", accumulator );
                break;
            }
            else {
                program[i].opcode = "jmp".to_string(); // Didn't fix infinite loop, change back and move to next one
            }
        }

        // Reset seen flags for next run
        //
        for instr in &mut program {
            instr.been_seen = false;
        }
    }
}

fn execute_until_infinite_or_finished( program : &mut Vec<Instruction> ) -> (i32, bool) {

    let mut accumulator = 0;
    let mut pc : usize = 0;
    let mut finished = false;

    loop {

        // println!("{:?} -- {}", program[ pc ], pc );

        let instr = program[ pc ].opcode.clone();

        if program[pc].been_seen { break; } // exit if we detect an infinite loop

        program[pc].been_seen = true;
        match instr.as_str() {

            "nop" => pc = increment_pc( pc, program.len() ),

            "acc" => {
                accumulator = accumulator + program[pc].operand;
                pc = increment_pc( pc, program.len() );
            },

            "jmp" => {
                
                let offset = program[pc].operand;
                // Some hackiness because usize can't be negative
                // Must be a better way...
                //
                if offset < 0 { 
                    pc = pc - offset.abs() as usize 
                }
                else {
                    pc = pc + offset as usize;
                }

                assert!( pc <= program.len() )
            },

            _ => println!("bad opcode")

        }

        if pc >= program.len() { println!("reached end of program"); finished = true; break; }

    }

    (accumulator, finished)
}

fn increment_pc( cur_pc : usize, max_pc : usize ) -> usize {

    assert!( cur_pc + 1 <= max_pc );
    cur_pc as usize + 1

}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}