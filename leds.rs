// solution to r/dailyprogrammer challenge #290 [Intermediate] Blinking LEDs
// https://www.reddit.com/r/dailyprogrammer/comments/5as91q/20161102_challenge_290_intermediate_blinking_leds/
// port of the c++ version

//use std::io;
//use std::io::Write;
use std::fs::File;
use std::io::prelude::*;



/* grammar of our mini programming language:

      <line>: <whitespace> <instruction> |
              <label>                    |
              <empty>

      <instruction> : ld a,<num> |
                      ld b,<num> |
                      out (0),a  |
                      rlca       |
                      rrca       |
                      djnz <labelref>             
*/

enum Line {
    Instruction( Instruction ),
    Label( String ),
    Empty,
    ParseError
}

enum Instruction {
    LoadA( u8 ),
    LoadB( u8 ),
    Out,
    Rlca,
    Rrca,
    Djnz{ label_index: u8 },
    End
}

impl Instruction {
    fn to_string(&self) -> String {
        match self {
            &Instruction::LoadA( a ) => format!("ld a,{}", a),
            &Instruction::LoadB( b ) => format!("ld b,{}", b),
            &Instruction::Out => String::from("out (0),a"),
            &Instruction::Rlca => String::from("rlca"),
            &Instruction::Rrca => String::from("rrca"),
            &Instruction::Djnz{ label_index } => format!("djnz {}", label_index),
            &Instruction::End => String::from("End"),
       }
    }
}

fn leds_to_string(register_a: u8) -> String {
    let mut result = String::new();
    for c in (0..7).map(|x| (0x80 >> x) & register_a) {
        match c {
            0 => result.push('.'),
            _ => result.push('*'),
        }
    }
    result
}

#[test]
#[allow(dead_code)]
fn test_leds_to_string() {
    assert!(leds_to_string(0x80) == "*......."); // TODO assert_eq!() is better ??
    assert!(leds_to_string(0x55) == ".*.*.*.*");
    assert!(leds_to_string(0xAA) == "*.*.*.*.");
    assert!(leds_to_string(0x0F) == "....****");
}



fn parse(line: &str, labels: &Vec<String>) -> Line {
    // note: allowing the following even though they are technically not part of the problem description:
    //  line -> <whitespace> <instruction> <whitespace>
    //  line -> <whitespace> <label> <whitespace>

    let trimmed: &str = line.trim();
    match trimmed {
        "rlca" => Line::Instruction(Instruction::Rlca),
        "rrca" => Line::Instruction(Instruction::Rrca),
        "out (0),a" => Line::Instruction(Instruction::Out),
        "" => Line::Empty,
        _ => {
            if trimmed.len() < 5 {
                try_label(trimmed)
            }
            else {
                match &trimmed[0..5] {
                    "ld a," => try_register_load(trimmed, true),
                    "ld b," => try_register_load(trimmed, false),
                    "djnz " => try_jump(trimmed, labels),
                    _ => try_label(trimmed),
                }
            }
        },
    }
}

fn try_jump(trimmed: &str, labels: &Vec<String>) -> Line {
    Line::ParseError
}

fn try_label(trimmed: &str) -> Line {
    // only alphanumeric chars and ends in a :
    Line::ParseError
}

// TODO: boolean instead of another enum is kind of a hack, but im lazy
fn try_register_load(trimmed: &str, register_a: bool) -> Line {
    let load_value = &trimmed[5..trimmed.len()].parse::<u8>();
    match (register_a, load_value) {
        (true, &Ok(a)) => Line::Instruction(Instruction::LoadA(a)),
        (false, &Ok(b)) => Line::Instruction(Instruction::LoadB(b)),
        (_, &Err(_)) => Line::ParseError,
    }
}

fn run_file(file_name: &str) {
    let mut file = File::open(file_name).expect(&(format!("Failed to open {}", file_name))); // TODO: error handling here!
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(&(format!("Failed to read from {}", file_name)));
    let mut labels = vec![];    
    for line in contents.lines() {
        match parse(line, &labels) {
            Line::Label(name) => labels.push(String::from(name)),
            _ => ()
        }
    }
}

fn main() {
    print!("Hello!");

    run_file("input1.txt");
}
