# Setup

If using vscode, you should install the extensions recommended in `.vscode/extensions.json`.

# Running programs

```
./scripts game_of_life
```

### Memory Layout

NB this differs slightly from the layout described in the book - the heap has been expanded to 16k words. This makes the implementation of the buddy heap allocation algorithm much simpler.

Also, my TEMP segment is smaller, and I have more virtual registers.

| address range | use                       |
| ------------- | ------------------------- |
| 0             | SP                        |
| 1             | LCL                       |
| 2             | ARG                       |
| 3             | POINTER TO THIS           |
| 4             | POINTER TO THAT           |
| 5-6           | TEMP - FOR USE IN VM CODE |
| 7-15          | VIRTUAL REGISTERS         |
| 16-255        | STATIC                    |
| 256-2047      | STACK                     |
| 2048-18431    | HEAP                      |
| 18432-26623   | SCREEN                    |
| 26624         | KBD                       |
| 26625-30424   | GLYPHS                    |
| 30425-32767   | FREE FOR FUTURE USE       |

### Stack Frame Layout

ARGUMENTS
RETURN ADDRESS
SAVED CALLER POINTERS
LOCALS
TEMPORARY VALUES / RETURN VALUE
