# Program and Command structure

a vm program is a collection of one or more files with a .vm extension, each consisting of one or more functions

within a vm file, each vm command appears in a separate line

true is represented as 0xffff

false is represented as 0x0000

## Arithmetic commands

y is the first value popped off the stack, x is the second

add: x + y
sub: x - y
neg: -y
eq: x == y
gt: x > y
lt: x < y
and: x & y
or: x | y
not: !y

## Memory access commands

push|pop segment index

### Memory segments

- argument: stores the function's arguments. one set for each instance of a running function
- local: stores the function's local variables. one set for each instance of a running function
- static: stores static variables shared by all functions within the same .vm file. shared by all functions in the same .vm file
- this/that: general purpose segments on heap, pointed to by pointer 0 and pointer 1 respectively. one set for each instance of a running program
- pointer: a two-entry segment that holds the base addresses of the this and that segments. one set for each instance of a running program.
- temp: fixed eight-entry segment that holds temporary variables for general use. shared by all functions in the program.
- constant: pseudo-segment that holds all the constants in the range 0..32767. shared by all functions in the program

## Program flow commands

label symbol
goto symbol
if-goto symbol

## Function commands

function functionName nLocals
call functionName nLocals
return

## Running functions

When a VM function starts running, it assumes that:

- the stack is empty
- the argument values on which it is supposed to operate are located in the argument segment
- the local variables that it is supposed to use are initialized to 0 and located in the local segment

## RAM layout

0 : R0 / SP / stack pointer - points to the NEXT topmost location in the stack
1 : R1 / LCL - points to the base of the current VM function's local segment
2 : R2 / ARG - points to the base of the current VM function's argument segment
3 : R3 / pointer 0 - points to the base of the current THIS segment within the heap
4 : R4 / pointer 1 - points to the base of the current THAT segment within the heap
5-12 : R5-R12 - holds the contents of the TEMP segment
13-15 : R13-R15 - can be used by the VM implementation as general-purpose registers
16-255 : static variables
256-2047 : stack
2048-16383 : heap
16384-24575 : memory-mapped screen
24576 : memory-mapped keyboard
24577-32767 : unused

### static variables

when a new symbol is encountered for the first time in
