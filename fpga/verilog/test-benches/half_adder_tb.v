`timescale 1 ns/10 ps

module half_adder_tb;
    reg a, b;
    reg [1:0] in, out, expected_out;
    localparam period = 20;
    half_adder UUT (.in(in), .out(out));
    
    initial
    begin
        for (int i=0; i<4; i=i+1) begin
            in = i;
            #period;
            { a, b } = i;
            expected_out = a + b;
            assert(out == expected_out) else begin
                $display("in: ", "%b", in, ", out: ", "%b", out, ", expected out: ", "%b", expected_out);
                $fatal(1);
            end;
        end
    end
endmodule
