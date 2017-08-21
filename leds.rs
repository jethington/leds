// solution to r/dailyprogrammer challenge #290 [Intermediate] Blinking LEDs
// https://www.reddit.com/r/dailyprogrammer/comments/5as91q/20161102_challenge_290_intermediate_blinking_leds/
// port of the c++ version

//use std::io;
//use std::io::Write;
use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;


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
    Djnz{ index: usize },
}

impl Instruction {
    #[allow(dead_code)]
    fn to_string(&self) -> String {
        match self {
            &Instruction::LoadA( a ) => format!("ld a,{}", a),
            &Instruction::LoadB( b ) => format!("ld b,{}", b),
            &Instruction::Out => String::from("out (0),a"),
            &Instruction::Rlca => String::from("rlca"),
            &Instruction::Rrca => String::from("rrca"),
            &Instruction::Djnz{ index } => format!("djnz {}", index),
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

fn parse(line: &str, labels: &HashMap<String, usize>) -> Line {
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
                try_label(trimmed, labels)
            }
            else {
                match &trimmed[0..5] {
                    "ld a," => try_register_load(trimmed, true),
                    "ld b," => try_register_load(trimmed, false),
                    "djnz " => try_jump(trimmed, labels),
                    _ => try_label(trimmed, labels),
                }
            }
        },
    }
}

fn try_jump(trimmed: &str, labels: &HashMap<String, usize>) -> Line {
    //Line::ParseError
    
    // note: am I in trouble because trimmed is an &str instead of a string?
    
    match labels.get(trimmed) {
        Some(line) => Line::Instruction(Instruction::Djnz{ index: *line }),
        None => Line::ParseError,
    }
}

fn try_label(trimmed: &str, labels: &HashMap<String, usize>) -> Line {
    if labels.contains_key(trimmed) {
        Line::ParseError
    }
    else {
        // TODO: still need to see if it makes a valid label
        Line::ParseError
    } 
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
    //let mut labels = vec![]; // TODO: replace with map
    
    // JCE: does &str actually work here?  looks like possibly yes:
    // https://users.rust-lang.org/t/efficient-string-hashmaps-for-a-frequency-count/7752/2
    // would need to refactor, label is technically a slice of line but currently a copy is created
    
    let mut instructions: Vec<Instruction> = vec![];
    let mut labels: HashMap<String, usize> = HashMap::new(); // TODO: is usize the right type of index?
    let mut index: usize = 0;
    for line in contents.lines() {
        match parse(line, &labels) {
            Line::Label(name) => { labels.insert(String::from(name), index); },
            _ => (),
        }
        index += 1;
    }
    
    index = 0;
    while index < instructions.len() {
 
        index += 1;
    }
}

fn main() {
    print!("Hello!");

    run_file("input1.txt");
}

#[test]
#[allow(dead_code)]
fn test_try_register_load() {
    assert_eq!(try_register_load("ld a,4", true), Line::Instruction(Instruction::LoadA(4)));
    assert_eq!(try_register_load("ld a, 4", true), Line::Instruction(Instruction::LoadA(4)));
    assert_eq!(try_register_load("ld b, 4", false), Line::Instruction(Instruction::LoadB(4)));
    assert_eq!(try_register_load("ld a, 400", true), Line::ParseError);
    assert_eq!(try_register_load("ld a, a123", true), Line::ParseError);
    assert_eq!(try_register_load("ld a,", true), Line::ParseError);
}

#[test]
#[allow(dead_code)]
fn test_try_label() {
    assert_eq!(test_try_label("lbl:"), Line::Label("lbl")); // String::from() ?
    assert_eq!(test_try_label("longer_label:"), Line::Label("longer_label"));
    assert_eq!(test_try_label("bad:label:"), Line::ParseError);
    assert_eq!(test_try_label("toomanycolons::"), Line::ParseError);
    assert_eq!(test_try_label("foo"), Line::ParseError);
    assert_eq!(test_try_label(":bar"), Line::ParseError);
}

#[test]
#[allow(dead_code)]
fn test_try_jump() {
    
}

#[test]
#[allow(dead_code)]
fn test_parse() {
    // make sure correct boolean is supplied to try_register_load()
    assert_eq!(parse("  ld a,4"), Line::Instruction(Instruction::LoadA(4)));
    assert_eq!(parse("ld b, 4   "), Line::Instruction(Instruction::LoadB(4)));
    
    // test instructions that don't have a dedicated function
    assert_eq!(parse("    "), Line::Empty);
    assert_eq!(parse(" rlca  "), Line::Instruction(Instruction::Rlca));
    assert_eq!(parse("  rrca "), Line::Instruction(Instruction::Rrca));
    assert_eq!(parse(" out (0),a "), Line::Instruction(Instruction::Out));
    
    // note: whitespace handling is covered in above tests
}

#[test]
#[allow(dead_code)]
fn test_leds_to_string() {
    assert_eq!(leds_to_string(0x80), "*.......");
    assert_eq!(leds_to_string(0x55), ".*.*.*.*");
    assert_eq!(leds_to_string(0xAA), "*.*.*.*.");
    assert_eq!(leds_to_string(0x0F), "....****");
}

