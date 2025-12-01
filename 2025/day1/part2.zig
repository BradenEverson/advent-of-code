const std = @import("std");
const input = @embedFile("data/input");

pub fn main() void {
    var lines = std.mem.tokenizeAny(u8, input, "\n");
    var curr: i32 = 50;
    var zero_count: u32 = 0;

    while (lines.next()) |line| {
        const dir = line[0];
        const amount = line[1..];

        var mag: i32 = 1;
        if (dir == 'L') {
            mag = -1;
        }

        const val = std.fmt.parseInt(usize, amount, 10) catch @panic("Malformed Input :(");

        for (0..val) |_| {
            curr += mag;
            curr = @rem(curr, 100);

            if (curr == 0) {
                zero_count += 1;
            }
        }
    }

    std.debug.print("{d} zeros\n", .{zero_count});
}
