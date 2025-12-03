const std = @import("std");
const input = @embedFile("data/input");

pub fn main() void {
    var lines = std.mem.tokenizeAny(u8, input, "\n");
    var joltage_sum: u64 = 0;

    while (lines.next()) |line| {
        const first, const idx = max(line, 0, line.len - 1);
        const second, _ = max(line, idx, line.len);

        const first_int = first - '0';
        const second_int = second - '0';

        const joltage: u64 = @as(u64, first_int) * 10 + @as(u64, second_int);

        joltage_sum += joltage;
    }

    std.debug.print("Joltage Sum: {d}\n", .{joltage_sum});
}

pub fn max(buf: []const u8, start: usize, end: usize) struct { u8, usize } {
    var biggest: u8 = '0';
    var idx: usize = 0;

    for (start..end) |i| {
        if (buf[i] > biggest) {
            biggest = buf[i];
            idx = i;
        }
    }

    return .{ biggest, idx + 1 };
}
