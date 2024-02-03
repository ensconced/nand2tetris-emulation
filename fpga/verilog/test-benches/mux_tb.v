`timescale 1 ns/10 ps

module mux_gate_tb;
    reg in_a, in_b, sel;
    localparam period = 20;
    mux UUT (.in_a(in_a), .in_b(in_b), .sel(sel), .out(out));
    
    initial
    begin
        for (int i=0; i<8; i=i+1) begin
            {in_a, in_b, sel} = i;
            #period;
  
            assert(out == (sel ? in_b : in_a)) else begin
                $display("in_a: ", in_a, ", in_b: ", in_b, ", sel: ", sel, ", out: ", out);
                $fatal(1);
            end;
        end
    end
endmodule
