# DailyProgrammer Challenge #290 [Intermediate]
## *Blinking LEDs*

Taken from: https://old.reddit.com/r/dailyprogrammer/comments/5as91q/20161102_challenge_290_intermediate_blinking_leds/

## Description

Mark saw someone doing experiments with blinking LEDs (imagine something like this ) and became fascinated by it. He wants to know more about it. He knows you are good with computers, so he comes to you asking if you can teach him how it works. You agree, but as you don't have any LEDs with you at the moment, you suggest: "Let's build an emulator with which we can see what's happening inside". And that's today's challenge.

**1st Part**

The 1st part should be easy, even though the description is rather verbose. If you want more challenge try the 2nd part afterwards.

Our system has 8 LEDs, we represent their state with a text output. When all LEDs are off, it is printed as string of eight dots "........". When a led is on, it is printed as "*". LED-0 is on the right side (least significant bit), LED-7 is on the left side. Having LEDs 0 and 1 on and all others off is written as "......**"

On input you get a sequence of lines forming a program. Read all lines of the input (detect EOF, or make the first line contain number of lines that follow, whichever is more convenient for you). Afterwards, print LED states as they are whenever the program performs an out instruction.

Each line is in the following format:

```
<line>: <whitespace> <instruction> |
        <empty>

<instruction> : ld a,<num> |
                out (0),a
```

<whitespace> is one or more of characters " " or "\t". <num> is a number between 0 and 255.

Instruction ld a,<num> sets internal 8-bit register A to the given number. Instruction out (0),a updates the LEDs according to the current number in A. The LED-0's state corresponds to bit 0 of number in A, when that number is represented in binary. For example, when A = 5, the LED state after out instruction is ".....*.*".

You should output the LED states after each out instruction.

Challenge input 1:

```
  ld a,14
  out (0),a
  ld a,12
  out (0),a
  ld a,8
  out (0),a

  out (0),a
  ld a,12
  out (0),a
  ld a,14
  out (0),a
```

Expected output:

```
....***.
....**..
....*...
....*...
....**..
....***.
```

**2nd Part**

We will extend our programming language, so that we can do more updates without writing out instruction for each of them. We will have loops.

Each line has the following format:

```
<line>: <whitespace> <instruction> |
        <label>                    |
        <empty>

<instruction> : ld a,<num> |
                ld b,<num> |
                out (0),a  |
                rlca       |
                rrca       |
                djnz <labelref>
```

<label> is a sequence of characters a-z A-Z _ terminated with one character ":". <labelref> is a sequence of characters a-z A-Z _ (it corresponds to some label minus the trailing ":").

Instruction ld b,<num> sets a number to register B. Instruction rlca rotates bits in register A one position to the left, in circle (i.e. bit 0 goes to bit 1, bit 1 to bit 2, and bit 7 to bit 0). Instruction rrca rotates bits in register A one position to the right, in circle. Instruction djnz <labelref> (decrement and jump if not zero) subtracts one from the value of register B and if the new value of register B is not zero then the processing of instructions continues at the line containg label corresponding to the <labelref>. You can assume that in the input text <label> is always given before the corresponding <labelref> (i.e. jumps go backwards).

You should output the LED states after each out instruction.

Challenge Input 2:

```
  ld b,3

triple:
  ld a,126
  out (0),a
  ld a,60
  out (0),a
  ld a,24
  out (0),a
  djnz triple
```

Challenge Output 2:

```
.******.
..****..
...**...
.******.
..****..
...**...
.******.
..****..
...**...
```

Challenge Input 3:

```
  ld a,1
  ld b,9

loop:
  out (0),a
  rlca
  djnz loop
```

Challenge Output 3:

```
.......*
......*.
.....*..
....*...
...*....
..*.....
.*......
*.......
.......*
```

Challenge Input 4:

```
  ld a,2
  ld b,9

loop:
  out (0),a
  rrca
  djnz loop
```

Challenge Output 4:

```
......*.
.......*
*.......
.*......
..*.....
...*....
....*...
.....*..
......*.
```
