// Adds 1 + ... + 100
  M=1    // i = 1
  M=0    // sum = 0
  D=M    // D=i
  @100
  D=D-A  // D=i-100
  D;JGT  // if (i-100)>0 goto END
  D=M    // D=i
  M=D+M  // sum=sum+i
  M=M+1  // i=i+1
  0;JMP  // goto LOOP
  0;JMP  // infinite loop
