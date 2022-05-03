@R0 // word_offset
M=0

(BEGIN_MAIN_LOOP)
  @R1 // word_idx
  M=0

  // kill some time
  @R2
  M=0
  (BEGIN_KILL_TIME_LOOP)
  @32767
  D=A
  @R2
  D=D-M
  @END_KILL_TIME_LOOP
  D;JEQ

    @R3
    M=0
    (BEGIN_INNER_KILL_TIME_LOOP)
    @5
    D=A
    @R3
    D=D-M
    @END_INNER_KILL_TIME_LOOP
    D;JEQ

      @R3
      M=M+1

    @BEGIN_INNER_KILL_TIME_LOOP
    0;JMP
    (END_INNER_KILL_TIME_LOOP)

    @R2
    M=M+1
  @BEGIN_KILL_TIME_LOOP
  0;JMP
  (END_KILL_TIME_LOOP)


  (BEGIN_CLEAR_LOOP)
   @8192
   D=A
  @R1
  D=D-M
  @END_CLEAR_LOOP
  D;JEQ

    @R1
    D=M
    @SCREEN
    A=A+D
    M=0

    @R1
    M=M+1

  @BEGIN_CLEAR_LOOP
  0;JMP
  (END_CLEAR_LOOP)

  @R0
  D=M
  @R1
  M=D

  (BEGIN_DRAW_LOOP)
  @R1
  D=M
  @R0
  D=D-M
  @512
  D=D-A
  @END_DRAW_LOOP
  D;JEQ

    @R1
    D=M
    @SCREEN
    A=D+A
    M=0
    M=!M

    @32
    D=A
    @R1
    M=M+D

  @BEGIN_DRAW_LOOP
  0;JMP
  (END_DRAW_LOOP)

  @R0
  D=M
  @32
  D=D+A
  @8191
  D=D&A
  @R0
  M=D

@BEGIN_MAIN_LOOP
0;JMP