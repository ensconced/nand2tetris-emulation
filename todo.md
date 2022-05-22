### vm compiler

- game of life in vm code
- vm - enable parsing of whole files/directories (is it just a matter of parsing then concatenating the commands of all the input files?)
- do something using the keyboard...pong? typing?
- improve emitted code size - use more jumps?
- do static variables as suggested in the book...? (I don't think this is actually necessary - my way should be fine...)

### main program

- maybe get rid of clap and parse cli args myself

### parser

- make error handling and reporting more consistent

### programs

- game of life
- pong
- mandelbrot

### better graphics

- fix flickering by assigning a "don't draw" register which programs can use to flag when frame buffer is in an inconsistent state, and which the computer will read to decide whether or not to actually refresh the screen. will need to figure out how to make this work on the fpga too!

### hardware

- add timer in hardware?
- ethernet...
