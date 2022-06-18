### Jack codegen

- code to compile entire input directory - reuse what we already have from vm compiler
- testing setup - basically call functions and test what ends up on the stack using similar within_n_ticks code as for vm testing?
- typechecking
- check arg count equals param count? might be difficult - would need to look across classes sometimes...
- codegen for subroutines...
- write barebones versions of key stdlib functions in Jack - e.g. Sys.init, Memory.alloc, String.appendChar, String.new etc to allow testing
- write full stdlib

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
