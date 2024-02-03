# nand2tetris, in verilog


### UART setup

- plug in and turn on basys3
- `sudo dmesg | grep tty` should show you which tty corresponds to the board
- `sudo stty -F /dev/ttyUSB1 50` to set baud rate...
- then you can just do stuff like `echo hello > /dev/ttyUSB1`
