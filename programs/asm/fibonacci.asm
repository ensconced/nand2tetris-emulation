
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

($entry_somefile.add)

@0
D=A
@ARG
A=M+D
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@1
D=A
@ARG
A=M+D
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


// decrement stack pointer, so it's pointing to y
@SP
M=M-1
// load y into D
A=M
D=M
// point A to x
A=A-1
M=M+D


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

($entry_somefile.fibonacci)

@0
D=A
@ARG
A=M+D
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@0
D=A


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


// decrement stack pointer, so it's pointing to y
@SP
M=M-1
// set A to point to x
A=M-1
// use R13 as another pointer to x
D=A
@R13
M=D
// load y into D
@SP
A=M
D=M
// load x - y into D
A=A-1
D=M-D
// initially set result to true (i.e. 0xffff i.e. -1)
M=-1
// then flip to false unless condition holds
@$after_set_to_false_0
D;JEQ
@R13
A=M
M=0
($after_set_to_false_0)


// Pop into d register
@SP
MA=M-1
D=M


@somefile.fibonacci$return_zero
D;JNE


@0
D=A
@ARG
A=M+D
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@1
D=A


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


// decrement stack pointer, so it's pointing to y
@SP
M=M-1
// set A to point to x
A=M-1
// use R13 as another pointer to x
D=A
@R13
M=D
// load y into D
@SP
A=M
D=M
// load x - y into D
A=A-1
D=M-D
// initially set result to true (i.e. 0xffff i.e. -1)
M=-1
// then flip to false unless condition holds
@$after_set_to_false_1
D;JEQ
@R13
A=M
M=0
($after_set_to_false_1)


// Pop into d register
@SP
MA=M-1
D=M


@somefile.fibonacci$return_one
D;JNE


@0
D=A
@ARG
A=M+D
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@1
D=A


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


// decrement stack pointer, so it's pointing to y
@SP
M=M-1
// load y into D
A=M
D=M
// point A to x
A=A-1
M=M-D


// Load return address into D
@$return_point_0
D=A


// Push from d register
@SP
A=M
M=D
@SP
M=M+1

@LCL
D=M

// Push from d register
@SP
A=M
M=D
@SP
M=M+1

@ARG
D=M

// Push from d register
@SP
A=M
M=D
@SP
M=M+1

@THIS
D=M

// Push from d register
@SP
A=M
M=D
@SP
M=M+1

@THAT
D=M

// Push from d register
@SP
A=M
M=D
@SP
M=M+1


// Set arg pointer
@SP
D=M
@6
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_somefile.fibonacci
0;JMP

// Label for return to caller
($return_point_0)


@0
D=A
@ARG
A=M+D
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@2
D=A


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


// decrement stack pointer, so it's pointing to y
@SP
M=M-1
// load y into D
A=M
D=M
// point A to x
A=A-1
M=M-D


// Load return address into D
@$return_point_1
D=A


// Push from d register
@SP
A=M
M=D
@SP
M=M+1

@LCL
D=M

// Push from d register
@SP
A=M
M=D
@SP
M=M+1

@ARG
D=M

// Push from d register
@SP
A=M
M=D
@SP
M=M+1

@THIS
D=M

// Push from d register
@SP
A=M
M=D
@SP
M=M+1

@THAT
D=M

// Push from d register
@SP
A=M
M=D
@SP
M=M+1


// Set arg pointer
@SP
D=M
@6
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_somefile.fibonacci
0;JMP

// Label for return to caller
($return_point_1)


// decrement stack pointer, so it's pointing to y
@SP
M=M-1
// load y into D
A=M
D=M
// point A to x
A=A-1
M=M+D


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

(somefile.fibonacci$return_zero)

@0
D=A


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


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

(somefile.fibonacci$return_one)

@1
D=A


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


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

($entry_Sys.init)

@10
D=A


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


// Load return address into D
@$return_point_2
D=A


// Push from d register
@SP
A=M
M=D
@SP
M=M+1

@LCL
D=M

// Push from d register
@SP
A=M
M=D
@SP
M=M+1

@ARG
D=M

// Push from d register
@SP
A=M
M=D
@SP
M=M+1

@THIS
D=M

// Push from d register
@SP
A=M
M=D
@SP
M=M+1

@THAT
D=M

// Push from d register
@SP
A=M
M=D
@SP
M=M+1


// Set arg pointer
@SP
D=M
@6
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_somefile.fibonacci
0;JMP

// Label for return to caller
($return_point_2)
