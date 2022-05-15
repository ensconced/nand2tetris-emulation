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
- static: stores static variables shared by all functions within the same .vm file. shared by all functions in the same .vm file (is it really just that it's namespaced by file, using dots?? - JB)
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

0 : R0 / SP / stack pointer - POINTS TO the NEXT topmost location in the stack
1 : R1 / LCL - POINTS TO the base of the current VM function's local segment
2 : R2 / ARG - POINTS TO the base of the current VM function's argument segment
3 : R3 / pointer 0 - POINTS TO the base of the current THIS segment within the heap
4 : R4 / pointer 1 - POINTS TO the base of the current THAT segment within the heap
5-12 : R5-R12 - holds the contents of the TEMP segment
13-15 : R13-R15 - can be used by the VM implementation as general-purpose registers
16-255 : static variables
256-2047 : stack
2048-16383 : heap
16384-24575 : memory-mapped screen
24576 : memory-mapped keyboard
24577-32767 : unused

### static variables

each static variable number j in a VM file f is turned into the assembly symbol f.j. for example, suppose that the file Xxx.vm contains the command `push static 3`. This command can be translated to the Hack assembly commands @Xxx.3 and D=M, followed by additional assembly code that pushes D's value to the stack.

### function scoping

it should be assumed that each Xxx.vm file has the functions named as Xxx.function_name. This will be the case for the vm files compiled by the jack compiler. i.e. the scoping of functions within vm files is already acheived by the jack compiler.

### function labels

all labels are local within a function. how this is acheived: each `label b` command in a VM function `f` should cause the VM implementation to generate a globally unique symbol `f$b` where `f` is the function name and `b` is the label symbol within the VM function's code. when translating `goto b` and `if-goto b` VM commands into the target language, the full label specification `f$b` must be used instead of b. this means that all gotos, if-gotos etc stay within the same function. recall that vm function labels cannot contain the $ character, which is what makes this safe.

### design of VM implementation

the vm translator should accept a single command-line parameter, for either a file name of the form Xxx.vm, or a directory name containing one or more vm files. The result of the translation is always a single assembly file named Xxx.asm, created in the same directory as the input Xxx.

### initialization / bootstrap code

when the vm implementation starts running or is reset, the convention is that it always executes an argument-less vm function called Sys.init. typically, this function then calls the main function in the user's program. thus, compilers that generate vm code must ensure that each translated program will have one such Sys.init function.

So the bootstrap code will need to do the following:

- set SP to 256
- call main function, in the normal way (it has no arguments)

### how to define a function

- create label
- push zeros for each of its local variables

### how to call a function

- push the arguments for the function to the stack (NB this is not done by the call command itself, it's assumed to have already been done)
- push the return address to the stack. this is simply the ROM address following the jump into the subroutine code
- save the caller's frame to the stack (i.e. push the caller's LCL, ARG, THIS and THAT (well, actually the pointers to THIS/THAT, I guess?))
- reposition ARG pointer
- reposition LCL pointer
- jump to execute the code of the subroutine

### how to return from a function

- when function has finished executing, there should be a single return value left at the top of the stack
- copy this return value to ARG, so that after we returning it will be at the top of the stack for the caller
- set SP to point to ARG+1
- restore LCL, ARG, THIS, THAT from the caller's saved state
- jump execution to the return address
