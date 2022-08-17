# nand2tetris project

To assemble a program:

```
cargo run -- assemble ./programs/blinky.asm blinky
```

To run the compiled machine code on the emulator:

```
cargo run -- run blinky
```

### memory layout

NB this differs slightly from the layout described in the book - the heap has been expanded to 16k words. This makes the implementation of the buddy heap allocation algorithm much simpler.

| address range | use                       |
| ------------- | ------------------------- |
| 0             | SP                        |
| 1             | LCL                       |
| 2             | ARG                       |
| 3             | POINTER TO THIS           |
| 4             | POINTER TO THAT           |
| 5-12          | TEMP - FOR USE IN VM CODE |
| 13-14         | VIRTUAL REGISTERS         |
| 16-255        | STATIC                    |
| 256-2047      | STACK                     |
| 2048-18431    | HEAP                      |
| 18432-26623   | SCREEN                    |
| 26624         | KBD                       |
| 26625-32767   | FREE FOR FUTURE USE       |
