### main program

- vm - enable parsing of whole files/directories, and prepending filenames to function names etc correctly
- vm codegen!
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
