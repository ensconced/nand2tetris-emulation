(START)
@R0 // loop_idx
M=0

// initialize screen_word_location equal to SCREEN
@SCREEN
D=A
@R1 // screen_word_location
M=D

(LOOP_BEGIN)

// if loop_idx > 15, jump to START again
@15
D=A
@R0
D=M-D
@START
D;JGT

// set memory at screen_word_location
@0
A=!A
D=A
@R1
A=M
M=D

// increment screen_word_location
@32
D=A
@R1
M=M+D

// increment loop_idx
@R0
M=M+1

@LOOP_BEGIN
0;JMP