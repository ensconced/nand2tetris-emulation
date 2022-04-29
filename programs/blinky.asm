// IF LED IS ON, JUMP TO 8

(START)
A=16384 // 0100 0000 0000 0000 // load 16384 into A | 0
D=M //  1111 110000 010 000 // load M[A] into D | 1
@LED_OFF // 0000 0000 0000 1000 // load 8 into A | 2
D;JGT //  1110 001100 000 001 // jump if D is positive | 3

// TURN LED ON

A=16384 // 0100 0000 0000 0000 // load 16384 into A | 4
M=1 // 1110 111111 001 000 // load 1 into M[A] | 5

// JUMP TO 10
@WAIT // 0000 0000 0000 1010 | 6
JMP // 1110 101010 000 111 | 7

// TURN LED OFF

(LED_OFF)
@16384 // 0100 0000 0000 0000 // load 16384 into A | 8
M=0 // 1110 101010 001 000 // load 0 into M[A] | 9

// WAIT ONE SECOND
(WAIT)
@2 // 0000 0000 0000 0010 // load 2 into A | 10
M=0 // 1110 101010 001 000 // load 0 into M[A] | 11

(OUTER_INCREMENT)
@2 // 0000 0000 0000 0010 // load 2 into A | 12
M=M+1 // 1111 110111 001 000 // load M[A] + 1 into M[A] | 13

@1 // 0000 0000 0000 0001 // load 1 into A | 14
M=0 // 1110 101010 001 000 // load 0 into M[A] | 15

(INNER_INCREMENT)
@1 // 0000 0000 0000 0001 // load 1 into A | 16
M=M+1 // 1111 110111 001 000 // load M[A] + 1 into M[A] | 17

// if M[1] - 300 < 0, jump to 16
@1 // 0000 0000 0000 0001 // load 1 into A | 18
D=M // 1111 110000 010 000 // load M[1] into D | 19
@16384 // 0100 0000 0000 0000 // load 16384 into A | 20
D=D-A // 1110 010011 010 000 // load D - A into D | 21
@INNER_INCREMENT // 0000 0000 0001 0000 // load 16 into A | 22
D;JLT // 1110 001100 000 100 // if D < 0, jump | 23

// if M[2] - 32767 < 0, jump to 12
@2 // 0000 0000 0000 0010 // load 2 into A | 24
D=M // 1111 110000 010 000 // load M[2] into D | 25
@128 // 0000 0000 1000 0000 // load 128 into A | 26
D=D-A // 1110 010011 010 000 // load D - A into D | 27
@OUTER_INCREMENT // 0000 0000 0000 1100 // load 12 into A | 28
D;JLT // 1110 001100 000 100 // if D < 0, jump | 29

// BACK TO THE START
@(START) // 0000 0000 0000 0000 | 30
JMP // 1110 101010 000 111 | 31