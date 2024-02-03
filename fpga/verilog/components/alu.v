module alu (in_x,
            in_y,
            zero_x,
            not_x,
            zero_y,
            not_y,
            use_add,
            should_not_output,
            out,
            not_output,
            output_is_zero);
    input [15:0] in_x, in_y;
    input zero_x, not_x, zero_y, not_y, use_add, should_not_output;
    output [15:0] out, not_output;
    output output_is_zero;
    
    wire [15:0] x_maybe_zeroed;
    wire [15:0] y_maybe_zeroed;
    wire [15:0] x_notted;
    wire [15:0] y_notted;
    wire [15:0] x_maybe_notted;
    wire [15:0] y_maybe_notted;
    wire [15:0] anded;
    wire [15:0] added;
    wire [15:0] anded_or_added;
    wire [15:0] notted;
    wire [15:0] final_output;
    wire output_is_non_zero;
    
    mux_16 ZERO_X_MUX (in_x, 16'b0, zero_x, x_maybe_zeroed);
    mux_16 ZERO_Y_MUX (in_y, 16'b0, zero_y, y_maybe_zeroed);
    not_16 NOT_16_X (x_maybe_zeroed, x_notted);
    not_16 NOT_16_Y (y_maybe_zeroed, y_notted);
    mux_16 NOT_X_MUX (x_maybe_zeroed, x_notted, not_x, x_maybe_notted);
    mux_16 NOT_Y_MUX (y_maybe_zeroed, y_notted, not_y, y_maybe_notted);
    and_16 ANDER (x_maybe_notted, y_maybe_notted, anded);
    add_16 ADDER (x_maybe_notted, y_maybe_notted, added);
    mux_16 OP_MUX (anded, added, use_add, anded_or_added);
    not_16 POST_OP_NOT (anded_or_added, notted);
    mux_16 NOT_MUX (anded_or_added, notted, should_not_output, final_output);
    not_16 OUTPUT_NOT_16 (final_output, not_output);
    is_non_zero IS_NON_ZERO (final_output, output_is_non_zero);
    not_gate IS_ZERO_NOT (output_is_non_zero, output_is_zero);
    assign out = final_output;
endmodule
