### Debug planning

### Phase 1 - output JSON file explaining compiler output

compiler outputs something like this:

{
"jack": [jack modules]
"vm_code": [vm modules] // each vm code line points back to a line of jack code
"asm": [asm lines] // each asm line points back to a line of vm code
}

### Phase 2 - visualise compiler output

- add web frontend to visualize JSON data

### Phase 3 - post hoc runtime debugging

- initial MVP would be post hoc runtime debugging - run and record PC / other state on each tick, import this into debugger
- add stepping ability
  - step line of jack code
  - step line of vm code
  - step asm instruction

### Phase 4 - realtime runtime debugging

- disadvantage of post hoc approach is that might need to collect huge amounts of data if program runs for a long time...
- so instead just do it in realtime

# TODO

- write full stdlib
- implement hashmap module
- implement vector module
- add SCREEN variable for use in jack code
- allow use of e.g. var int[4] foo; to declare fixed-length arrays to be allocated in static section, or on stack. this could make the code in Memory.jack much neater
- make statics and stack sections bigger!
- figure out limits of current algo for two's complement multiplication - is there a simple failing example for a small negative number?
- booth's algo? or...read this: https://pages.cs.wisc.edu/~markhill/cs354/Fall2008/beyond354/int.mult.html ?
- implement proper Sys class - Sys.init should init other stdlib classes then call Main.main
- code to compile entire input directory - reuse what we already have from vm compiler
- typechecking? but will need to allow some coercions - e.g. obj to int for Memory.dealloc, array to obj for constructors.
- check arg count equals param count? might be difficult - would need to look across classes sometimes...
- codegen for subroutines...

### jack extras

- for loops
- pointers
- typechecking? might be tricky...
- break/continue
- more in stdlib - arrays/vectors with push method

### programs

- pong
- mandelbrot
- tetris
- game of life
- asteroids
- snake
- game selector

### hardware

- add timer?
- multiplication / division?
- floating point?
- bitshift?
- ethernet...?
- graphics: fix flickering by assigning a "don't draw" register which programs can use to flag when frame buffer is in an inconsistent state, and which the computer will read to decide whether or not to actually refresh the screen. will need to figure out how to make this work on the fpga too!

### optimizations

- improve emitted code size

### debugging

- add print instruction, only included when compilation is targeting the emulator
- report stack overflows etc in emulator
- improve indentation of emitted vm and asm code
- consolidate various debugging tools

### refactoring

- make error handling and reporting more consistent in parsers
- maybe get rid of clap and parse cli args myself
