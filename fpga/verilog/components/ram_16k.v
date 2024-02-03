module ram_16k(input [15:0] in,
               input [13:0] addr,
               input load,
               input clock,
               output [15:0] out);
    reg [15:0] memory [16383:0];
    integer i;
    initial begin
        for(i = 0; i < 16384; i=i+1) 
            memory[i] = 16'b0;
    end
    always @(posedge clock) begin
        if (load == 1) begin
            memory[addr] <= in;
        end
    end
    assign out = memory[addr];
endmodule
