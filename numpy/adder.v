module adder(input clock, input reset);

    reg [7:0] a;
    reg [7:0] b;
    reg [7:0] y;

    always @(posedge clock) begin
        if (reset) begin
            a <= 0;
            b <= 0;
            y <= 0;
        end
        else begin
            y <= a + b;
        end
    end

endmodule
