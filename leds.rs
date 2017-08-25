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

#[derive(Debug, PartialEq)]
enum Line {
    Instruction( Instruction ),
    Label( String ),
    Empty,
    ParseError
}

#[derive(Debug, PartialEq)]
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
    for c in (0..8).map(|x| (0x80 >> x) & register_a) {
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
    let mut temp = trimmed.to_owned(); // TODO: better name    
    match temp.pop() {
        // last char in trimmed line should be the semicolon
        Some(':') => {
            // label must be a valid format, and cannot match an existing label
            if is_identifier(&temp) && !labels.contains_key(&temp) {
                Line::Label(temp)
            }
            else {
                Line::ParseError
            }
        },
        _ => Line::ParseError,
    }
}

fn is_identifier(chars: &str) -> bool {
    // this function determines which characters I allow in the name of a label
    let mut ret: bool = true;
    for c in chars.chars() {
        if !c.is_alphanumeric() && c != '_' {
            ret = false;
            break;
        }
    }
    ret
}

// TODO: boolean instead of another enum is kind of a hack, but im lazy
fn try_register_load(trimmed: &str, register_a: bool) -> Line {
    let load_value = &trimmed[5..trimmed.len()].parse::<u8>();
    //let temp_slice = trimmed[5..trimmed.len()].trim(); // TODO: is allowing a space here actually required?
    //let load_value = &temp_slice.parse::<u8>();
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
    
    // JCE: does &str actually work here?  looks like possibly yes:
    // https://users.rust-lang.org/t/efficient-string-hashmaps-for-a-frequency-count/7752/2
    // would need to refactor, label is technically a slice of line but currently a copy is created
    
    let mut instructions: Vec<Instruction> = vec![];
    let mut labels: HashMap<String, usize> = HashMap::new();

    // parse the input file into instructions
    for line in contents.lines() {
        match parse(line, &labels) {
            Line::Label(name) => { labels.insert(String::from(name), instructions.len()); },
            Line::Instruction(instruction) => instructions.push(instruction),
            Line::Empty => (),
            Line::ParseError => (),
        }
    }
    
    // execute the instructions
    let mut register_a: u8 = 0;
    let mut register_b: u8 = 0;
    let mut instruction_index: usize = 0;
    while instruction_index < instructions.len() {
        match instructions[instruction_index] {
            Instruction::LoadA(a) => register_a = a,
            Instruction::LoadB(b) => register_b = b,
            Instruction::Out => { println!("{}", leds_to_string(register_a)); },
            Instruction::Rlca => register_a = register_a.rotate_left(1),
            Instruction::Rrca => register_a = register_a.rotate_right(1),
            Instruction::Djnz{index: i} => {
                register_b = register_b.saturating_sub(1); // don't decrement if already 0
                if register_b > 0 {
                    instruction_index = i;
                    continue; // don't want to add 1 in this case, so skip that part
                }
            },
        }
        instruction_index += 1;
    }
}

/*  LoadA( u8 ),
    LoadB( u8 ),
    Out,
    Rlca,
    Rrca,
    Djnz{ index: usize },*/

fn main() {
    print!("Hello!");

    run_file("input1.txt");
}

#[test]
#[allow(dead_code)]
fn test_try_register_load() {
    assert_eq!(try_register_load("ld a,4", true), Line::Instruction(Instruction::LoadA(4)));
    assert_eq!(try_register_load("ld a,4", true), Line::Instruction(Instruction::LoadA(4)));
    assert_eq!(try_register_load("ld b,4", false), Line::Instruction(Instruction::LoadB(4)));
    assert_eq!(try_register_load("ld a,400", true), Line::ParseError);
    assert_eq!(try_register_load("ld a,a123", true), Line::ParseError);
    assert_eq!(try_register_load("ld a,", true), Line::ParseError);
}

#[test]
#[allow(dead_code)]
fn test_try_label() {
    let labels: HashMap<String, usize> = HashMap::new();
    assert_eq!(try_label("lbl:", &labels), Line::Label("lbl".to_owned())); // String::from() ?
    assert_eq!(try_label("longer_label:", &labels), Line::Label("longer_label".to_owned()));
    assert_eq!(try_label("bad:label:", &labels), Line::ParseError);
    assert_eq!(try_label("toomanycolons::", &labels), Line::ParseError);
    assert_eq!(try_label("foo", &labels), Line::ParseError);
    assert_eq!(try_label(":bar", &labels), Line::ParseError);
}

#[test]
#[allow(dead_code)]
fn test_try_jump() {
    
}

#[test]
#[allow(dead_code)]
fn test_parse() {
    let labels: HashMap<String, usize> = HashMap::new();

    // make sure correct boolean is supplied to try_register_load()
    assert_eq!(parse("  ld a,4", &labels), Line::Instruction(Instruction::LoadA(4)));
    assert_eq!(parse("ld b,4   ", &labels), Line::Instruction(Instruction::LoadB(4)));
    
    // test instructions that don't have a dedicated function
    assert_eq!(parse("    ", &labels), Line::Empty);
    assert_eq!(parse(" rlca  ", &labels), Line::Instruction(Instruction::Rlca));
    assert_eq!(parse("  rrca ", &labels), Line::Instruction(Instruction::Rrca));
    assert_eq!(parse(" out (0),a ", &labels), Line::Instruction(Instruction::Out));
    
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

