`timescale 1 ns/10 ps

module full_adder_tb;
    reg a, b, c;
    reg [1:0] out, expected_out;
    reg [2:0] in;
    localparam period = 20;
    full_adder UUT (.in(in), .out(out));
    
    initial
    begin
        for (int i=0; i<8; i=i+1) begin
            in = i;
            #period;
            { a, b, c } = i;
            expected_out = a + b + c;
            assert(out == expected_out) else begin
                $display("in: ", "%b", in, ", out: ", "%b", out, ", expected out: ", "%b", expected_out);
                $fatal(1);
            end;
        end
    end
endmodule
