// modifiers
"00"  	// no modifier -- default for instruction
"01"	.a	// a -> a
"02"	.b  // b -> b
"03"	.ab // a -> b
"04"	.ba // b -> a
"05"	.f  // a and b -> a and b 
"06"	.x  // a and b -> b and a
"07"	.i  // whole instruction -> whole instruction for mov seq and sne

// custom modifiers
"08"	.m  // m -> m
"09"	.am // a -> m
"0A"	.bm // b -> m
"0B"	.ma // m -> a
"0C"	.mb // m -> b
"0D"	.o  // o -> o
"0E"	.ao // a -> o
"0F"	.bo // b -> o
"10"	.oa // o -> a
"11"	.ob // o -> b
"12"	.mf // m,a,b -> m,a,b
"13"	.fm // m,a,b -> a,b,m
"14"	.y  // o,m,a,b -> a,b,o,m


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

01 mov .i // move copies data
02 add .ab // arithmetic addition, .i is same as .f
03 sub .ab // arithmetic subtraction, .i same as .f
04 mul .ab // arithmetic multiplication, .i same as .f
05 div .ab // arithmetic division, floor, b / a, div/0 illegal
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
12 and // bitwise and
13 bor // bitwise or
14 xor // bitwise xor
15 not // bitwise not
xx ldp
yy stp

/*
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

*/
