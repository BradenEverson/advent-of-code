const std = @import("std");
const input = @embedFile("data/input");

pub fn main() void {
    var lines = std.mem.tokenizeAny(u8, input, "\n");
    var valid: u64 = 0;

    while (lines.next()) |line| {
        std.testing.expectEqual('x', line[2]) catch @panic("Assert: all dimensions are 2 digits");
        const width = std.fmt.parseInt(u64, line[0..2], 10) catch @panic("Failed to parse");
        const height = std.fmt.parseInt(u64, line[3..5], 10) catch @panic("Failed to parse");

        var spaces: u64 = 0;
        const req = line[7..];

        var reqs = std.mem.tokenizeAny(u8, req, " ");
        while (reqs.next()) |r| {
            const val = std.fmt.parseInt(u64, r, 10) catch @panic("Failed to parse");
            spaces += val;
        }

        if (height / 3 * width / 3 >= spaces) valid += 1;
    }

    std.debug.print("Valid: {}\n", .{valid});
}
