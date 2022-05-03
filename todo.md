### main program

- maybe get rid of clap and parse cli args myself

### parser

- add initial lexer stage to assembler to greatly simplify parsing
- make error handling and reporting more consistent

### programs

- pong in asm
- mandelbrot in asm

### better graphics

- fix flickering by assigning a "don't draw" register which programs can use to flag when frame buffer is in an inconsistent state, and which the computer will read to decide whether or not to actually refresh the screen. will need to figure out how to make this work on the fpga too!

### hardware

- add timer in hardware?
