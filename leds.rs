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
            //LoadA( ref a) => (),
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
    assert!(leds_to_string(0x80) == "*.......");
    assert!(leds_to_string(0x55) == ".*.*.*.*");
    assert!(leds_to_string(0xAA) == "*.*.*.*.");
    assert!(leds_to_string(0x0F) == "....****");
}

fn run_file(file_name: &str) {
    let mut file = File::open(file_name).expect(&(format!("Failed to open {}", file_name))); // TODO: error handling here!
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect(&(format!("Failed to read from {}", file_name)));
}

fn main() {
    print!("Hello!");

    run_file("input1.txt");
}
