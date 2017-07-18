// solution to r/dailyprogrammer challenge #290 [Intermediate] Blinking LEDs
// https://www.reddit.com/r/dailyprogrammer/comments/5as91q/20161102_challenge_290_intermediate_blinking_leds/

#include <iostream>
#include <string>
#include <cassert>
#include <fstream>
#include <vector>
#include <unordered_map>
#include <cstdint>
#include <sstream>
#include <iomanip>

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
    Instruction(Instruction_t type) : type(type), data(0u) {
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
	void print(void) const {
		switch (type) {
		case LOAD_A: std::cout << "ld a," << (int)data << std::endl;		break;
		case LOAD_B: std::cout << "ld b," << (int)data << std::endl;		break;
		case OUT: std::cout << "out (0),a" << std::endl;					break;
		case RLCA: std::cout << "rlca" << std::endl;						break;
		case RRCA: std::cout << "rrca" << std::endl;						break;
		case DJNZ: std::cout << "djnz " << (int)data << std::endl;			break;
		case END: std::cout << "END" << std::endl;							break;
		default: std::cout << "ILLEGAL INSTRUCTION :" << type << std::endl; break; // should be unreachable
		}
	}
private:   
    const std::uint8_t data; // value to load A or B, or label index
			                 // note: assuming B is 8 bits (not clear from problem), also assuming there will be a max of 255 labels
};

void run(std::string file_name);
std::string leds_to_string(uint8_t register_a);

std::string leds_to_string(uint8_t register_a) {
	std::string str;
	for (int i = 0; i < 8; i++) {
		uint8_t led = register_a & (0x80u >> i);
		if (led == 0) {
			str.push_back('.'); // 0 = Off
		}
		else {
			str.push_back('*'); // 1 = On
		}
	}
	return str;
}

// TODO: split up this function?
void run(std::string file_name) {   
	std::vector<Instruction> instructions;
	std::unordered_map<std::string, std::uint8_t> label_indexes; // TODO: spelling?

	// parse the file
	// TODO: try std::regex instead
	std::ifstream in_file(file_name);
	if (!in_file.is_open()) {
		std::cout << "Error: could not open file '" << file_name << "'" << std::endl;
	}

	std::string line;
	while (std::getline(in_file, line)) {
		// process one line at a time
		size_t index = line.find_first_not_of(" \n\r\t");
		if (index == std::string::npos) {
			// empty, skip
		}
		else {
			std::string str = line.substr(index, line.length());
			if (str == "rlca") {
				instructions.push_back(Instruction(RLCA));
			}
			else if (str == "rrca") {
				instructions.push_back(Instruction(RRCA));
			}
			else if (str == "out (0),a") {
				instructions.push_back(Instruction(OUT));
			}
			else {
				std::string s = str.substr(0, 5);
				if (s == "ld a,") {
					int a;
					std::istringstream(str.substr(5, str.size())) >> a;
					if ((a >= 0) && (a <= 255)) {
						instructions.push_back(Instruction(LOAD_A, a));
					}
				}
				else if (s == "ld b,") {
					int b;
					std::istringstream(str.substr(5, str.size())) >> b;
					if ((b >= 0) && (b <= 255)) {
						instructions.push_back(Instruction(LOAD_B, b));
					}
				}
				else if (s == "djnz ") {
					std::string label = str.substr(5, str.size()); // rest of the string
					int index = label_indexes[label]; // TODO: what if label doesnt exist? 
					// note: problem description guarantees jumps to label will always be backwards - so label should already exist
					instructions.push_back(Instruction(DJNZ, index));
				}
				else {
					// try to read string as a label
					// note: technically according to the problem description labels are not allowed to have whitespace in front of them but oh well
					int colon_index = str.find_first_of(':');
					if (colon_index != std::string::npos) {
						// TODO: not as strict as problem description for label, fix later
						std::string label = str.substr(0, colon_index);
						label_indexes.insert({ label, instructions.size() }); // TODO: what if label is after last instruction?
					}
					else {
						// no : found, cant be a label
						std::cout << "error parsing line, skipping:   " << line << std::endl;
					}
				}
			}
		}
		
    }
    instructions.push_back(Instruction(END));
   
	// print the file
	//for (const Instruction& instr : instructions) {
	//instr.print();
	//}

    // run the file
    int instruction_index = 0;
    uint8_t a;
    uint8_t b;
	bool done = false;
	while (!done) {
		Instruction next = instructions[instruction_index];
		instruction_index++;

		switch (next.type) {
		case LOAD_A:
			a = next.get_load_value();
			break;

		case LOAD_B:
			b = next.get_load_value();
			break;

		case OUT:
			std::cout << leds_to_string(a) << std::endl;
			break;

		case RLCA: {
			uint8_t temp = a >> 7;
			a = (a << 1) | temp;
			break;
		}
		case RRCA: {
			uint8_t temp = a << 7;
			a = (a >> 1) | temp;
			break;
		}
		case DJNZ: 
			// dont decrement if already 0
			if (b > 0u) {
				b--;
			}
			// if still not 0 after decrement, jump
			if (b > 0u) {
				instruction_index = next.get_label();
			}
			break;

		case END: 
			done = true;
			break;

		default: 
			std::cout << "ILLEGAL INSTRUCTION :" << next.type << std::endl;
			done = true; // this should be unreachable - end immediately if something gones this wrong
			break;
		}
	}
}

int main(void) {
   //run("input1.txt");
   //run("input2.txt");
   //run("input3.txt");
   run("input4.txt");
   
   return 0;
}