module adder(input clock, input reset);

    reg [31:0] r0;

    always @(posedge clock) begin
        if (reset) begin
            r0 <= 0;
        end
        else begin
            r0 <= r0 + 1;
        end
    end

endmodule
