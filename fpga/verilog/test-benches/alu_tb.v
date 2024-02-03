`timescale 1 ns/10 ps

module alu_tb;
    reg [15:0] in_x, in_y, out, not_output, expected_out;
    reg zero_x, not_x, zero_y, not_y, use_add, should_not_output, output_is_zero; 
    
    localparam period = 20;
    alu UUT (
        .in_x(in_x), 
        .in_y(in_y), 
        .zero_x(zero_x), 
        .zero_y(zero_y), 
        .not_x(not_x), 
        .not_y(not_y), 
        .use_add(use_add), 
        .should_not_output(should_not_output), 
        .out(out),
        .not_output(not_output),
        .output_is_zero(output_is_zero)
    );
    
    initial
    begin
        for (int i = 0; i < 16; i++) begin
            in_x <= $random();
            for (int j = 0; j < 16; j++) begin
                in_y <= $random();

                // zero
                use_add <= 1;
                should_not_output <= 0;
                zero_x <= 1;
                zero_y <= 1;
                not_x <= 0;
                not_y <= 0;

                #period
                expected_out = 16'b0; 
                assert(out == expected_out) else begin
                    $display("out: ", "%b", out, ", expected out: ", "%b", expected_out);
                    $fatal(1);
                end;

                // one
                use_add <= 1;
                should_not_output <= 1;
                zero_x <= 1;
                zero_y <= 1;
                not_x <= 1;
                not_y <= 1;

                #period
                expected_out = 16'b1; 
                assert(out == expected_out) else begin
                    $display("out: ", "%b", out, ", expected out: ", "%b", expected_out);
                    $fatal(1);
                end;

                // minus one
                use_add <= 1;
                should_not_output <= 0;
                zero_x <= 1;
                zero_y <= 1;
                not_x <= 1;
                not_y <= 0;

                #period
                expected_out = -16'b1; 
                assert(out == expected_out) else begin
                    $display("out: ", "%b", out, ", expected out: ", "%b", expected_out);
                    $fatal(1);
                end;

                // x
                use_add <= 1;
                should_not_output <= 0;
                zero_x <= 0;
                zero_y <= 1;
                not_x <= 0;
                not_y <= 0;

                #period
                expected_out = in_x; 
                assert(out == expected_out) else begin
                    $display("out: ", "%b", out, ", expected out: ", "%b", expected_out);
                    $fatal(1);
                end;

                // y
                use_add <= 1;
                should_not_output <= 0;
                zero_x <= 1;
                zero_y <= 0;
                not_x <= 0;
                not_y <= 0;

                #period
                expected_out = in_y; 
                assert(out == expected_out) else begin
                    $display("out: ", "%b", out, ", expected out: ", "%b", expected_out);
                    $fatal(1);
                end;

                // not x
                use_add <= 1;
                should_not_output <= 0;
                zero_x <= 0;
                zero_y <= 1;
                not_x <= 1;
                not_y <= 0;

                #period
                expected_out = ~in_x; 
                assert(out == expected_out) else begin
                    $display("out: ", "%b", out, ", expected out: ", "%b", expected_out);
                    $fatal(1);
                end;

                // not y
                use_add <= 1;
                should_not_output <= 0;
                zero_x <= 1;
                zero_y <= 0;
                not_x <= 0;
                not_y <= 1;

                #period
                expected_out = ~in_y; 
                assert(out == expected_out) else begin
                    $display("out: ", "%b", out, ", expected out: ", "%b", expected_out);
                    $fatal(1);
                end;

                // minus x
                use_add <= 1;
                should_not_output <= 1;
                zero_x <= 0;
                zero_y <= 1;
                not_x <= 0;
                not_y <= 1;

                #period
                expected_out = -in_x; 
                assert(out == expected_out) else begin
                    $display("out: ", "%b", out, ", expected out: ", "%b", expected_out);
                    $fatal(1);
                end;

                // minus y
                use_add <= 1;
                should_not_output <= 1;
                zero_x <= 1;
                zero_y <= 0;
                not_x <= 1;
                not_y <= 0;

                #period
                expected_out = -in_y; 
                assert(out == expected_out) else begin
                    $display("out: ", "%b", out, ", expected out: ", "%b", expected_out);
                    $fatal(1);
                end;

                // x + 1
                use_add <= 1;
                should_not_output <= 1;
                zero_x <= 0;
                zero_y <= 1;
                not_x <= 1;
                not_y <= 1;

                #period
                expected_out = in_x + 1; 
                assert(out == expected_out) else begin
                    $display("out: ", "%b", out, ", expected out: ", "%b", expected_out);
                    $fatal(1);
                end;

                // y + 1
                use_add <= 1;
                should_not_output <= 1;
                zero_x <= 1;
                zero_y <= 0;
                not_x <= 1;
                not_y <= 1;

                #period
                expected_out = in_y + 1; 
                assert(out == expected_out) else begin
                    $display("out: ", "%b", out, ", expected out: ", "%b", expected_out);
                    $fatal(1);
                end;

                // x - 1
                use_add <= 1;
                should_not_output <= 0;
                zero_x <= 0;
                zero_y <= 1;
                not_x <= 0;
                not_y <= 1;

                #period
                expected_out = in_x - 1; 
                assert(out == expected_out) else begin
                    $display("out: ", "%b", out, ", expected out: ", "%b", expected_out);
                    $fatal(1);
                end;

                // y - 1
                use_add <= 1;
                should_not_output <= 0;
                zero_x <= 1;
                zero_y <= 0;
                not_x <= 1;
                not_y <= 0;

                #period
                expected_out = in_y - 1; 
                assert(out == expected_out) else begin
                    $display("out: ", "%b", out, ", expected out: ", "%b", expected_out);
                    $fatal(1);
                end;

                // x + y
                use_add <= 1;
                should_not_output <= 0;
                zero_x <= 0;
                zero_y <= 0;
                not_x <= 0;
                not_y <= 0;

                #period
                expected_out = in_x + in_y; 
                assert(out == expected_out) else begin
                    $display("out: ", "%b", out, ", expected out: ", "%b", expected_out);
                    $fatal(1);
                end;

                // x - y
                use_add <= 1;
                should_not_output <= 1;
                zero_x <= 0;
                zero_y <= 0;
                not_x <= 1;
                not_y <= 0;

                #period
                expected_out = in_x - in_y; 
                assert(out == expected_out) else begin
                    $display("out: ", "%b", out, ", expected out: ", "%b", expected_out);
                    $fatal(1);
                end;

                // y - x
                use_add <= 1;
                should_not_output <= 1;
                zero_x <= 0;
                zero_y <= 0;
                not_x <= 0;
                not_y <= 1;

                #period
                expected_out = in_y - in_x; 
                assert(out == expected_out) else begin
                    $display("out: ", "%b", out, ", expected out: ", "%b", expected_out);
                    $fatal(1);
                end;

                // x & y
                use_add <= 0;
                should_not_output <= 0;
                zero_x <= 0;
                zero_y <= 0;
                not_x <= 0;
                not_y <= 0;

                #period
                expected_out = in_x & in_y; 
                assert(out == expected_out) else begin
                    $display("out: ", "%b", out, ", expected out: ", "%b", expected_out);
                    $fatal(1);
                end;

                // x | y
                use_add <= 0;
                should_not_output <= 1;
                zero_x <= 0;
                zero_y <= 0;
                not_x <= 1;
                not_y <= 1;

                #period
                expected_out = in_x | in_y; 
                assert(out == expected_out) else begin
                    $display("out: ", "%b", out, ", expected out: ", "%b", expected_out);
                    $fatal(1);
                end;
            end
        end
    end
endmodule
