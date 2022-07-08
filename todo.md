- stress-test integer division
- stress-test repeated alloc / dealloc with small arrays
- stress-test repeated alloc / dealloc with arrays of decreasing size
- stress-test repeated alloc / dealloc with arrays of increasing size! - requires defragmentation
- don't bother with Array class for now - just use Memory directly?
- figure out limits of current algo for two's complement multiplication - is there a simple failing example for a small negative number?
- booth's algo? or...read this: https://pages.cs.wisc.edu/~markhill/cs354/Fall2008/beyond354/int.mult.html ?
- implement proper Sys class - Sys.init should init other stdlib classes then call Main.main
- code to compile entire input directory - reuse what we already have from vm compiler
- typechecking? but will need to allow some coercions - e.g. obj to array for Memory.dealloc, array to obj for constructors.
- check arg count equals param count? might be difficult - would need to look across classes sometimes...
- codegen for subroutines...
- write full stdlib

### jack extras

- for loops
- pointers
- typechecking? might be tricky...
- break/continue

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

### refactoring

- make error handling and reporting more consistent in parsers
- maybe get rid of clap and parse cli args myself
