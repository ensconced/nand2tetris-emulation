
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

@2048
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


@2
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


@1
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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@THIS
D=M

// add index
@2
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


@7
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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@THIS
D=M

// add index
@4
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

(Sys.init$start)

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
@5
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_increment_timer
0;JMP

// Label for return to caller
($return_point_0)


@SP
M=M-1


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
@5
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_handle_frame
0;JMP

// Label for return to caller
($return_point_1)


@SP
M=M-1


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
@5
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_key_changed
0;JMP

// Label for return to caller
($return_point_2)


// Pop into d register
@SP
MA=M-1
D=M


@Sys.init$call_handle_key_change
D;JNE


@Sys.init$start
0;JMP

(Sys.init$call_handle_key_change)

// Load return address into D
@$return_point_3
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
@5
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_handle_key_change
0;JMP

// Label for return to caller
($return_point_3)


@SP
M=M-1


@Sys.init$start
0;JMP

($entry_handle_frame)
D=0

// Push from d register
@SP
A=M
M=D
@SP
M=M+1

D=0

// Push from d register
@SP
A=M
M=D
@SP
M=M+1

D=0

// Push from d register
@SP
A=M
M=D
@SP
M=M+1

D=0

// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@2080
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


@4
M=D


@0
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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@LCL
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

(handle_frame$start_loop)

@0
D=A
@LCL
A=M+D
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@32
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


@handle_frame$end_loop
D;JNE


@0
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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@LCL
D=M

// add index
@1
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


@0
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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@LCL
D=M

// add index
@2
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


@1
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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@LCL
D=M

// add index
@3
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

(handle_frame$start_inner_loop)

@1
D=A
@LCL
A=M+D
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@16
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


@handle_frame$end_inner_loop
D;JNE


@0
D=A
@LCL
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
@LCL
A=M+D
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


// Load return address into D
@$return_point_4
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
@7
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_get_next_state
0;JMP

// Label for return to caller
($return_point_4)


@3
D=A
@LCL
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
M=M&D


@2
D=A
@LCL
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
M=M|D


// Pop into d register
@SP
MA=M-1
D=M


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@LCL
D=M

// add index
@2
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


@3
D=A
@LCL
A=M+D
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@3
D=A
@LCL
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


// Pop into d register
@SP
MA=M-1
D=M


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@LCL
D=M

// add index
@3
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


@1
D=A
@LCL
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
M=M+D


// Pop into d register
@SP
MA=M-1
D=M


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@LCL
D=M

// add index
@1
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


@handle_frame$start_inner_loop
0;JMP

(handle_frame$end_inner_loop)

@2
D=A
@LCL
A=M+D
D=M


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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@THAT
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


@4
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
M=M+D


// Pop into d register
@SP
MA=M-1
D=M


@4
M=D


@0
D=A
@LCL
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
M=M+D


// Pop into d register
@SP
MA=M-1
D=M


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@LCL
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


@handle_frame$start_loop
0;JMP

(handle_frame$end_loop)

// Load return address into D
@$return_point_5
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
@5
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_copy_next_buffer
0;JMP

// Label for return to caller
($return_point_5)


@SP
M=M-1


// Load return address into D
@$return_point_6
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
@5
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_draw_from_compact_representation
0;JMP

// Label for return to caller
($return_point_6)


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

($entry_read_value)
D=0

// Push from d register
@SP
A=M
M=D
@SP
M=M+1

D=0

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


// Pop into d register
@SP
MA=M-1
D=M


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@LCL
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


@0
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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@LCL
D=M

// add index
@1
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

(read_value$start_loop)

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


@1
D=A
@LCL
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
@$after_set_to_false_2
D;JEQ
@R13
A=M
M=0
($after_set_to_false_2)


// Pop into d register
@SP
MA=M-1
D=M


@read_value$end_loop
D;JNE


@0
D=A
@LCL
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
@LCL
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


// Pop into d register
@SP
MA=M-1
D=M


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@LCL
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


@1
D=A
@LCL
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
M=M+D


// Pop into d register
@SP
MA=M-1
D=M


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@LCL
D=M

// add index
@1
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


@read_value$start_loop
0;JMP

(read_value$end_loop)

@2048
D=A


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


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


// decrement stack pointer, so it's pointing to y
@SP
M=M-1
// load y into D
A=M
D=M
// point A to x
A=A-1
M=M+D


// Pop into d register
@SP
MA=M-1
D=M


@3
M=D


@0
D=A
@THIS
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
@LCL
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
M=M&D


@0
D=A
@LCL
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
@$after_set_to_false_3
D;JEQ
@R13
A=M
M=0
($after_set_to_false_3)


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

($entry_get_next_state)
D=0

// Push from d register
@SP
A=M
M=D
@SP
M=M+1


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


// Load return address into D
@$return_point_7
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
@7
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_live_neighbour_count
0;JMP

// Label for return to caller
($return_point_7)


// Pop into d register
@SP
MA=M-1
D=M


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@LCL
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


// Load return address into D
@$return_point_8
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
@7
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_read_value
0;JMP

// Label for return to caller
($return_point_8)


// Pop into d register
@SP
MA=M-1
D=M


@get_next_state$handle_live_cell
D;JNE


@get_next_state$handle_dead_cell
0;JMP

(get_next_state$handle_live_cell)

@0
D=A
@LCL
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
@$after_set_to_false_4
D;JEQ
@R13
A=M
M=0
($after_set_to_false_4)


// Pop into d register
@SP
MA=M-1
D=M


@get_next_state$live
D;JNE


@0
D=A
@LCL
A=M+D
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@3
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
@$after_set_to_false_5
D;JEQ
@R13
A=M
M=0
($after_set_to_false_5)


// Pop into d register
@SP
MA=M-1
D=M


@get_next_state$live
D;JNE


@get_next_state$die
0;JMP

(get_next_state$handle_dead_cell)

@0
D=A
@LCL
A=M+D
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@3
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
@$after_set_to_false_6
D;JEQ
@R13
A=M
M=0
($after_set_to_false_6)


// Pop into d register
@SP
MA=M-1
D=M


@get_next_state$live
D;JNE


@get_next_state$die
0;JMP

(get_next_state$die)

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

(get_next_state$live)

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

($entry_left)

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


@15
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
@$after_set_to_false_7
D;JEQ
@R13
A=M
M=0
($after_set_to_false_7)


// Pop into d register
@SP
MA=M-1
D=M


@left$call_handle_left_edgecase
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


// Pop into d register
@SP
MA=M-1
D=M


@5
M=D


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
M=M+D


// Pop into d register
@SP
MA=M-1
D=M


@6
M=D


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

(left$call_handle_left_edgecase)

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


// Load return address into D
@$return_point_9
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
@7
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_left_edgecase
0;JMP

// Label for return to caller
($return_point_9)


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

($entry_left_edgecase)

@0
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


@6
M=D


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


// Load return address into D
@$return_point_10
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
@$entry_is_even
0;JMP

// Label for return to caller
($return_point_10)


// Pop into d register
@SP
MA=M-1
D=M


@left_edgecase$handle_even_wordidx
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


// Pop into d register
@SP
MA=M-1
D=M


@5
M=D


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

(left_edgecase$handle_even_wordidx)

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
M=M+D


// Pop into d register
@SP
MA=M-1
D=M


@5
M=D


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

($entry_right)

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
@$after_set_to_false_8
D;JEQ
@R13
A=M
M=0
($after_set_to_false_8)


// Pop into d register
@SP
MA=M-1
D=M


@right$handle_right_edgecase
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


// Pop into d register
@SP
MA=M-1
D=M


@5
M=D


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


// Pop into d register
@SP
MA=M-1
D=M


@6
M=D


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

(right$handle_right_edgecase)

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


// Load return address into D
@$return_point_11
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
@7
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_right_edgecase
0;JMP

// Label for return to caller
($return_point_11)


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

($entry_right_edgecase)

@15
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


@6
M=D


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


// Load return address into D
@$return_point_12
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
@$entry_is_even
0;JMP

// Label for return to caller
($return_point_12)


// Pop into d register
@SP
MA=M-1
D=M


@right_edgecase$handle_even_wordidx
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


// Pop into d register
@SP
MA=M-1
D=M


@5
M=D


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

(right_edgecase$handle_even_wordidx)

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
M=M+D


// Pop into d register
@SP
MA=M-1
D=M


@5
M=D


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

($entry_up)

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


@30
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
M=M+D


@31
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
M=M&D


// Pop into d register
@SP
MA=M-1
D=M


@5
M=D


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


// Pop into d register
@SP
MA=M-1
D=M


@6
M=D


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

($entry_down)

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
M=M+D


@31
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
M=M&D


// Pop into d register
@SP
MA=M-1
D=M


@5
M=D


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


// Pop into d register
@SP
MA=M-1
D=M


@6
M=D


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

($entry_live_neighbour_count)
D=0

// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@2112
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


// Load return address into D
@$return_point_13
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
@7
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_neighbour_positions
0;JMP

// Label for return to caller
($return_point_13)


@SP
M=M-1


@0
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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@LCL
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

(live_neighbour_count$start_loop)

@3
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@2128
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
@$after_set_to_false_9
D;JEQ
@R13
A=M
M=0
($after_set_to_false_9)


// Pop into d register
@SP
MA=M-1
D=M


@live_neighbour_count$end_loop
D;JNE


@0
D=A
@THIS
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
@THIS
A=M+D
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


// Load return address into D
@$return_point_14
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
@7
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_read_value
0;JMP

// Label for return to caller
($return_point_14)


// Pop into d register
@SP
MA=M-1
D=M


@live_neighbour_count$add_to_sum
D;JNE


@live_neighbour_count$skip_add_to_sum
0;JMP

(live_neighbour_count$add_to_sum)

@0
D=A
@LCL
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
M=M+D


// Pop into d register
@SP
MA=M-1
D=M


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@LCL
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

(live_neighbour_count$skip_add_to_sum)

@3
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
M=M+D


// Pop into d register
@SP
MA=M-1
D=M


@3
M=D


@live_neighbour_count$start_loop
0;JMP

(live_neighbour_count$end_loop)

@0
D=A
@LCL
A=M+D
D=M


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

($entry_neighbour_positions)

@2112
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


// Load return address into D
@$return_point_15
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
@7
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_up
0;JMP

// Label for return to caller
($return_point_15)


@SP
M=M-1


@5
D=M


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


@6
D=M


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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@THIS
D=M

// add index
@1
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


@5
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@6
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


// Load return address into D
@$return_point_16
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
@7
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_right
0;JMP

// Label for return to caller
($return_point_16)


@SP
M=M-1


@5
D=M


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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@THIS
D=M

// add index
@2
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


@6
D=M


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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@THIS
D=M

// add index
@3
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


@5
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@6
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


// Load return address into D
@$return_point_17
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
@7
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_down
0;JMP

// Label for return to caller
($return_point_17)


@SP
M=M-1


@5
D=M


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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@THIS
D=M

// add index
@4
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


@6
D=M


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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@THIS
D=M

// add index
@5
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


@5
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@6
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


// Load return address into D
@$return_point_18
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
@7
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_down
0;JMP

// Label for return to caller
($return_point_18)


@SP
M=M-1


@5
D=M


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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@THIS
D=M

// add index
@6
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


@6
D=M


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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@THIS
D=M

// add index
@7
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


@5
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@6
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


// Load return address into D
@$return_point_19
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
@7
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_left
0;JMP

// Label for return to caller
($return_point_19)


@SP
M=M-1


@5
D=M


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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@THIS
D=M

// add index
@8
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


@6
D=M


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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@THIS
D=M

// add index
@9
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


@5
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@6
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


// Load return address into D
@$return_point_20
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
@7
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_left
0;JMP

// Label for return to caller
($return_point_20)


@SP
M=M-1


@5
D=M


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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@THIS
D=M

// add index
@10
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


@6
D=M


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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@THIS
D=M

// add index
@11
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


@5
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@6
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


// Load return address into D
@$return_point_21
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
@7
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_up
0;JMP

// Label for return to caller
($return_point_21)


@SP
M=M-1


@5
D=M


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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@THIS
D=M

// add index
@12
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


@6
D=M


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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@THIS
D=M

// add index
@13
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


@5
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@6
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


// Load return address into D
@$return_point_22
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
@7
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_up
0;JMP

// Label for return to caller
($return_point_22)


@SP
M=M-1


@5
D=M


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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@THIS
D=M

// add index
@14
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


@6
D=M


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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@THIS
D=M

// add index
@15
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

($entry_copy_next_buffer)

@2048
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


@2080
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


@4
M=D

(copy_next_buffer$start_loop)

@3
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@2080
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
@$after_set_to_false_10
D;JEQ
@R13
A=M
M=0
($after_set_to_false_10)


// Pop into d register
@SP
MA=M-1
D=M


@copy_next_buffer$end_loop
D;JNE


@0
D=A
@THAT
A=M+D
D=M


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


@3
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
M=M+D


// Pop into d register
@SP
MA=M-1
D=M


@3
M=D


@4
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
M=M+D


// Pop into d register
@SP
MA=M-1
D=M


@4
M=D


@copy_next_buffer$start_loop
0;JMP

(copy_next_buffer$end_loop)

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

($entry_is_even)

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
M=M&D


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
@$after_set_to_false_11
D;JEQ
@R13
A=M
M=0
($after_set_to_false_11)


@SP
A=M-1
M=!M


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

($entry_draw_from_compact_representation)
D=0

// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@16384
D=A


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@15
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
M=M+D


// Pop into d register
@SP
MA=M-1
D=M


@drawing.vm.0
M=D


@2048
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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@LCL
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

(draw_from_compact_representation$start_loop)

@0
D=A
@LCL
A=M+D
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@2080
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
@$after_set_to_false_12
D;JEQ
@R13
A=M
M=0
($after_set_to_false_12)


// Pop into d register
@SP
MA=M-1
D=M


@draw_from_compact_representation$end_loop
D;JNE


@0
D=A
@LCL
A=M+D
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


// Load return address into D
@$return_point_23
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
@$entry_draw_row_from_compact_representation
0;JMP

// Label for return to caller
($return_point_23)


@SP
M=M-1


@drawing.vm.0
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@512
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
M=M+D


// Pop into d register
@SP
MA=M-1
D=M


@drawing.vm.0
M=D


@0
D=A
@LCL
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
M=M+D


// Pop into d register
@SP
MA=M-1
D=M


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@LCL
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


@draw_from_compact_representation$start_loop
0;JMP

(draw_from_compact_representation$end_loop)

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

($entry_draw_row_from_compact_representation)
D=0

// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@drawing.vm.0
D=M


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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@LCL
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


// Load return address into D
@$return_point_24
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
@$entry_draw_half_row
0;JMP

// Label for return to caller
($return_point_24)


@SP
M=M-1


@drawing.vm.0
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@16
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
M=M+D


// Pop into d register
@SP
MA=M-1
D=M


@drawing.vm.0
M=D


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
M=M+D


// Load return address into D
@$return_point_25
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
@$entry_draw_half_row
0;JMP

// Label for return to caller
($return_point_25)


@SP
M=M-1


@0
D=A
@LCL
A=M+D
D=M


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


@drawing.vm.0
M=D


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

($entry_draw_half_row)
D=0

// Push from d register
@SP
A=M
M=D
@SP
M=M+1

D=0

// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@drawing.vm.0
D=M


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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@LCL
D=M

// add index
@1
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


// Pop into d register
@SP
MA=M-1
D=M


@3
M=D


@1
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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@LCL
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

(draw_half_row$start_loop)

@0
D=A
@LCL
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
@$after_set_to_false_13
D;JEQ
@R13
A=M
M=0
($after_set_to_false_13)


// Pop into d register
@SP
MA=M-1
D=M


@draw_half_row$end_loop
D;JNE


@0
D=A
@THIS
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
@LCL
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
M=M&D


@0
D=A
@LCL
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
@$after_set_to_false_14
D;JEQ
@R13
A=M
M=0
($after_set_to_false_14)


// Load return address into D
@$return_point_26
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
@$entry_draw_square
0;JMP

// Label for return to caller
($return_point_26)


@SP
M=M-1


@drawing.vm.0
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


// Pop into d register
@SP
MA=M-1
D=M


@drawing.vm.0
M=D


@0
D=A
@LCL
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
@LCL
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


// Pop into d register
@SP
MA=M-1
D=M


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@LCL
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


@draw_half_row$start_loop
0;JMP

(draw_half_row$end_loop)

@1
D=A
@LCL
A=M+D
D=M


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


@drawing.vm.0
M=D


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

($entry_draw_square)
D=0

// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@drawing.vm.0
D=M


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


// Pop into d register
@SP
MA=M-1
D=M


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@LCL
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

(draw_square$start_loop)

@0
D=A
@LCL
A=M+D
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@16
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
@$after_set_to_false_15
D;JEQ
@R13
A=M
M=0
($after_set_to_false_15)


// Pop into d register
@SP
MA=M-1
D=M


@draw_square$end_loop
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


@3
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@32
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
M=M+D


// Pop into d register
@SP
MA=M-1
D=M


@3
M=D


@0
D=A
@LCL
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
M=M+D


// Pop into d register
@SP
MA=M-1
D=M


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@LCL
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


@draw_square$start_loop
0;JMP

(draw_square$end_loop)

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

($entry_handle_key_change)

// Load return address into D
@$return_point_27
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
@5
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_update_compact_representation
0;JMP

// Label for return to caller
($return_point_27)


@SP
M=M-1


@keyboard_input.vm.1
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
@$after_set_to_false_16
D;JEQ
@R13
A=M
M=0
($after_set_to_false_16)


// Pop into d register
@SP
MA=M-1
D=M


@handle_key_change$initialize_key_update_pointer
D;JNE


@handle_key_change$after_initialize_key_update_pointer
0;JMP

(handle_key_change$initialize_key_update_pointer)

@2048
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


@keyboard_input.vm.1
M=D

(handle_key_change$after_initialize_key_update_pointer)

@keyboard_input.vm.1
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
M=M+D


// Pop into d register
@SP
MA=M-1
D=M


@keyboard_input.vm.1
M=D


@keyboard_input.vm.1
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@2080
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
@$after_set_to_false_17
D;JEQ
@R13
A=M
M=0
($after_set_to_false_17)


// Pop into d register
@SP
MA=M-1
D=M


@handle_key_change$reset_key_update_pointer
D;JNE


@handle_key_change$call_draw_from_compact_representation
0;JMP

(handle_key_change$reset_key_update_pointer)

@2048
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


@keyboard_input.vm.1
M=D

(handle_key_change$call_draw_from_compact_representation)

// Load return address into D
@$return_point_28
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
@5
D=D-A
@ARG
M=D


// Set lcl pointer
@SP
D=M
@LCL
M=D


// Jump to the callee
@$entry_draw_from_compact_representation
0;JMP

// Label for return to caller
($return_point_28)


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

($entry_update_compact_representation)

@keyboard_input.vm.1
D=M


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


@keyboard_input.vm.0
D=M


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

($entry_increment_timer)

@keyboard_input.vm.0
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
M=M+D


// Pop into d register
@SP
MA=M-1
D=M


@keyboard_input.vm.0
M=D


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

($entry_key_changed)
D=0

// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@24576
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


@4
M=D


@0
D=A
@THAT
A=M+D
D=M


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


// stash value from D into R13
@R13
M=D

// put value of pointer in D
@LCL
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


@keyboard_input.vm.2
D=M


// Push from d register
@SP
A=M
M=D
@SP
M=M+1


@0
D=A
@LCL
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
@$after_set_to_false_18
D;JEQ
@R13
A=M
M=0
($after_set_to_false_18)


@SP
A=M-1
M=!M


// Pop into d register
@SP
MA=M-1
D=M


@key_changed$did_change
D;JNE


@key_changed$did_not_change
0;JMP

(key_changed$did_change)

@0
D=A
@LCL
A=M+D
D=M


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


@keyboard_input.vm.2
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

(key_changed$did_not_change)

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
