
// This will be the very first instruction in the computer's ROM.
// We don't want to go into an infinite loop quite yet, so skip over it!
@$skip_infinite_loop
0;JMP

// This will be the return address of the main Sys.init function, so when
// that function exits, the computer just goes into an infinite loop
($infinite_loop)
@$infinite_loop
0;JMP

($skip_infinite_loop)

// For each stack frame, ARG points to the base of the frame. This is the
// first stack frame, so here ARG points to the base of the entire stack.
@256
D=A
@ARG
M=D

// Initialize the stack pointer. Even though there is no real caller
// function for Sys.init, we leave the customary space for the saved LCL,
// ARG, THIS and THAT of the caller. This in addition to the return
// address means the stack pointer will start 5 addresses above the base
// of the stack.
@261
D=A
@SP
M=D

// LCL starts off pointing to the same address as the stack pointer.
@261
D=A
@LCL
M=D

// Load the return address. Sys.init takes no arguments, so this is
// located right at the base of the stack.
@$infinite_loop
D=A
@256
M=D

// Call Sys.init
@$entry_Sys.init
0;JMP

($entry_Sys.init)

@16384
D=A


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


// Pop into d register
@SP
MA=M-1
D=M


@3
M=D


@0
D=A


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@SP
A=M-1
M=!M


// Pop into d register
@SP
MA=M-1
D=M


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@THIS
D=M

// add index
@0
D=D+A

// stash memory address in R14
@R14
M=D

// get value back into D
@R13
D=M

// load value into memory
@R14
A=M
M=D


@ARG
D=M
@R13
M=D


@LCL
D=M
@R14
M=D


// Pop into d register
@R14
MA=M-1
D=M


@THAT
M=D


// Pop into d register
@R14
MA=M-1
D=M


@THIS
M=D


// Pop into d register
@R14
MA=M-1
D=M


@ARG
M=D


// Pop into d register
@R14
MA=M-1
D=M


@LCL
M=D


// Pop into d register
@R14
MA=M-1
D=M


@R14
M=D


// Pop into d register
@SP
MA=M-1
D=M


@R13
A=M
M=D


@R13
D=M
@SP
M=D+1


@R14
A=M
0;JMP