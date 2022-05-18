### main program

- proper initialization of single stack frame for Sys.init, enough at least that ARG etc doesn't just point to address 0 and overwrite SP!
- this should be enough to write tests for basic memory manipulation commands and get them passing
- start with single file...
- vm codegen
- vm - enable parsing of whole files/directories (is it just a matter of parsing then concatenating the commands of all the input files?)
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
