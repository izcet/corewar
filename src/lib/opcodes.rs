// modifiers
.a	// a -> a
.b  // b -> b
.ab // a -> b
.ba // b -> a
.f  // a and b -> a and b 
.x  // a and b -> b and a
.i  // whole instruction -> whole instruction for mov seq and sne

// forget about registers for now
.ar // a -> register
.br // b -> register
.ra // register -> a
.rb // register -> b
.fr // a and b -> register
.fx // a and b -> regester (b and a)
.

// addressing modes
'#' immediate // address of 0, 
              //raw data read/written exactly in place not referencing/modifying anything else
'$' direct // relative address from executing instruction to another instruction
'*' A indirect 	// use the A value from the instruction at the referenced address
				// resolve relative to the referenced address
'@' B indirect // same as A indirect but B address
'{' A pre-decrement indirect	// same as a indirect BUT
								// decrement number at address
								// then use that address's number as a pointer
							
'}' A post-increment indirect
								// same as A indirect BUT
								// use that address's number as a pointer
								// then increment it
'<' B pre-decrement indirect    // same but B
'>' B post-increment indirect   // same but B

// core instructions
FF dat // data instruction - kills process on execution
FF -- -- -- -- -- -- --
// does not increment the instruction pointer
// does not modify the carry
// increments the program counter by one


00 nop // no operation
00 -- -- -- -- -- -- --
// increments the instruction pointer by 1
// does not modify the carry
// does not increment the program counter
// addressing modes are done as normal

01 mov // move copies data
// default modifier is .i

02 add // arithmetic addition
// default is .ab
// .i is same as .f

03 sub // arithmetic subtraction
// default is .ab

04 mul // arithmetic multiplication
05 div // arithmetic division
06 mod // arithmetic modulo
07 jmp // jump
08 jmz // jump if zero
09 jmn // jump if nonzero
0A djn // decrement jump if nonzero
0B cmp // alias for seq
0C seq // skip if equal
0D sne // skip if not equal
0E slt // skip if less than
0F spl // split

// borrowed from 42 variant
11 liv // "live" -- not implemented, equivalent to NOP
12 and // bitwise and
13 bor // bitwise or
14 xor // bitwise xor
15 not // bitwise not
16 ldi // load into register
17 sti // set from register

// dare I dream a little?


opcode
|
|  opcode modifier
|  |
|  |  a-modifier
|  |  |
|  |  |  b-modifer
|  |  |  |
|  |  |  |  a-value (big end)
|  |  |  |  |
|  |  |  |  |  a-value (little end)
|  |  |  |  |  |
|  |  |  |  |  |  b-value (big end
|  |  |  |  |  |  |
|  |  |  |  |  |  |  b-value (little end)
|  |  |  |  |  |  |  |
xx xx xx xx xx xx xx xx

PC has
1 64-bit current instruction
xx xx xx xx xx xx xx xx
1 carry flag
1 8-bit 
xx xx xx xx xx xx xx xx
4 32-bit registers
xx xx xx xx xx xx xx xx
xx xx xx xx xx xx xx xx
4 64-bit registers
xx xx xx xx xx xx xx xx
xx xx xx xx xx xx xx xx
xx xx xx xx xx xx xx xx
xx xx xx xx xx xx xx xx
