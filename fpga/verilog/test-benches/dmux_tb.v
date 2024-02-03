`timescale 1 ns/10 ps

module dmux_gate_tb;
    reg in, sel;
    localparam period = 20;
    dmux UUT (.in(in), .sel(sel), .out_a(out_a), .out_b(out_b));
    
    initial
    begin
        for (int i=0; i<4; i=i+1) begin
            {in, sel} = i;
            #period;
  
            assert(out_a == (sel ? 0 : in)) else begin
                $display("in: ", in, ", sel: ", sel, ", out_a: ", out_a, ", out_b: ", out_b);
                $fatal(1);
            end;

            assert(out_b == (sel ? in : 0)) else begin
                $display("in: ", in, ", sel: ", sel, ", out_a: ", out_a, ", out_b: ", out_b);
                $fatal(1);
            end;
        end
    end
endmodule
