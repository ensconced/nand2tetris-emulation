### vm compiler

- vm - enable parsing of whole files/directories (is it just a matter of parsing then concatenating the commands of all the input files?) need to handle static vars properly
- break game of life out into multiple files

### main program

- maybe get rid of clap and parse cli args myself

### parser

- make error handling and reporting more consistent

### programs

- pong
- mandelbrot

### better graphics

- fix flickering by assigning a "don't draw" register which programs can use to flag when frame buffer is in an inconsistent state, and which the computer will read to decide whether or not to actually refresh the screen. will need to figure out how to make this work on the fpga too!

### hardware

- add timer in hardware?
- ethernet...

### optimizations

- improve emitted code size - use more jumps?

### debugging

- add print instruction, only included when compilation is targeting the emulator
- report stack overflows etc in emulator
