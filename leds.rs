// solution to r/dailyprogrammer challenge #290 [Intermediate] Blinking LEDs
// https://www.reddit.com/r/dailyprogrammer/comments/5as91q/20161102_challenge_290_intermediate_blinking_leds/
// port of the c++ version

use std::io;
use std::io::Write;

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
   fn print(&self) {
      match self {
         //LoadA( ref a) => (),
         &Instruction::LoadA( a ) => print!("ld a,{}", a),
         &Instruction::LoadB( b ) => print!("ld b,{}", b),
         &Instruction::Out => print!("out (0),a"),
         &Instruction::Rlca => print!("rlca"),
         &Instruction::Rrca => print!("rrca"),
         &Instruction::Djnz{ label_index } =>print!("djnz {}", label_index),
         &Instruction::End => print!("End"),
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

fn main() {
   print!("Hello!");
}