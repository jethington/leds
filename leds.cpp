// solution to r/dailyprogrammer challenge #290 [Intermediate] Blinking LEDs
// https://www.reddit.com/r/dailyprogrammer/comments/5as91q/20161102_challenge_290_intermediate_blinking_leds/

#include <iostream>
#include <string>
#include <cassert>
#include <fstream>
#include <vector>
#include <array>
#include <unordered_map>
#include <cstdint>

typedef enum {
   OFF = 0,
   ON = 1
} Led_State;

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

typedef enum {
   LOAD_A = 0,
   LOAD_B = 1,
   OUT = 2,
   RLCA = 3,
   RRCA = 4,
   DJNZ = 5,
   END = 6
} Instruction_t; // TODO: better name

struct Instruction {
   Instruction(Instruction_t type, std::uint8_t data) : type(type), data(data) {
      assert((type == LOAD_A) || (type == LOAD_B) || (type == DJNZ)); 
      // TODO: would be nice to separate LOAD from DJNZ...
   }
   Instruction(Instruction_t type) : Instruction(type, 0u) {
      assert((type == OUT) || (type == RLCA) ||(type == RRCA) || (type == END));
   }
   const Instruction_t type;
   std::uint8_t get_load_value(void) {
      assert((type == LOAD_A) || (type == LOAD_B));
      return data;
   }
   std::uint8_t get_label(void) {
      assert(type == DJNZ);
      return data;
   }
private:   
   const std::uint8_t data; // value to load A or B, or label index
                      // note: assuming B is 8 bits (not clear from problem), also assuming there will be a max of 255 labels
};

void run(std::string file_name);
std::string leds_to_string(std::array<Led_State, 8> leds);

std::string leds_to_string(std::array<Led_State, 8> leds) {
   std::string str;
   for (int i = 7; i >= 0; i--) {
      if (leds[i] == OFF) {
         str.push_back('.');
      }
      else {
         str.push_back('*');
      }
   }
   return str;
}

void run(std::string file_name) {   
   std::vector<Instruction> instructions;
   std::unordered_map<std::string, std::uint8_t> label_indexes; // TODO: spelling? 
   std::array<Led_State, 8> leds = { OFF, OFF, OFF, OFF, OFF, OFF, OFF, OFF };
   
   // parse the file
   std::ifstream in_file(file_name);
   std::string line;
   while (std::getline(in_file, line)) {
      // process one line at a time
      if (line == "rlca") {
         instructions.push_back(Instruction(RLCA));
      }
      else if (line == "rrca") {
         instructions.push_back(Instruction(RRCA));
      }
      else if (line == "out (0),a") {
         instructions.push_back(Instruction(OUT));
      }
      else if (line == "") {
         // empty, skip
      }
      else {
         // TODO: handle LOAD_A, LOAD_B, DJNZ, labels, and everything else (parse error)
      }
   }
   instructions.push_back(Instruction(END));
   
   // run the file
   int instruction_index = 0;
   
}

int main(void) {
   //run("input1.txt");
   //run("input2.txt");
   //run("input3.txt");
   //run("input4.txt");
   
   return 0;
}