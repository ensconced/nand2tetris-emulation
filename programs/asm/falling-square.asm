@word_offset
M=0

(BEGIN_MAIN_LOOP)
  @word_idx
  M=0

  // kill some time
  @outer_counter
  M=0
  (BEGIN_KILL_TIME_LOOP)
  @32767
  D=A
  @outer_counter
  D=D-M
  @END_KILL_TIME_LOOP
  D;JEQ

    @inner_counter
    M=0
    (BEGIN_INNER_KILL_TIME_LOOP)
    @5
    D=A
    @inner_counter
    D=D-M
    @END_INNER_KILL_TIME_LOOP
    D;JEQ

      @inner_counter
      M=M+1

    @BEGIN_INNER_KILL_TIME_LOOP
    0;JMP
    (END_INNER_KILL_TIME_LOOP)

    @outer_counter
    M=M+1
  @BEGIN_KILL_TIME_LOOP
  0;JMP
  (END_KILL_TIME_LOOP)


  (BEGIN_CLEAR_LOOP)
   @8192
   D=A
  @word_idx
  D=D-M
  @END_CLEAR_LOOP
  D;JEQ

    @word_idx
    D=M
    @SCREEN
    A=A+D
    M=0

    @word_idx
    M=M+1

  @BEGIN_CLEAR_LOOP
  0;JMP
  (END_CLEAR_LOOP)

  @word_offset
  D=M
  @word_idx
  M=D

  (BEGIN_DRAW_LOOP)
  @word_idx
  D=M
  @word_offset
  D=D-M
  @512
  D=D-A
  @END_DRAW_LOOP
  D;JEQ

    @word_idx
    D=M
    @SCREEN
    A=D+A
    M=0
    M=!M

    @32
    D=A
    @word_idx
    M=M+D

  @BEGIN_DRAW_LOOP
  0;JMP
  (END_DRAW_LOOP)

  @word_offset
  D=M
  @32
  D=D+A
  @8191
  D=D&A
  @word_offset
  M=D

@BEGIN_MAIN_LOOP
0;JMP