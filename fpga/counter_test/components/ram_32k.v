module ram_32k(input [15:0] in,
               input [14:0] addr,
               input load,
               input clock,
               output [15:0] out,
               output [15:0] led_out);
    (* ram_style = "block" *)
    reg [15:0] memory [32767:0];
    integer i;
    initial begin
        for(i = 0; i < 32768; i=i+1)
            memory[i] = 16'b0;
    end
    always @(posedge clock) begin
        if (load == 1) begin
            memory[addr] <= in;
        end
    end
    assign out = memory[addr];
    assign led_out = memory[30425];
endmodule
