const std = @import("std");
const input = @embedFile("data/input");

pub fn main() void {
    var lines = std.mem.tokenizeAny(u8, input, "\n");
    var joltage_sum: u64 = 0;

    while (lines.next()) |line| {
        var joltage_places: [12]u8 = undefined;
        var idx: usize = 0;

        for (1..13) |i| {
            const spots_required_after = 12 - i;

            const digit, idx = max(line, idx, line.len - spots_required_after);

            joltage_places[i - 1] = digit;
        }

        var joltage: u64 = 0;

        for (0..12) |i| {
            const curr = joltage_places[11 - i] - '0';

            joltage += @as(u64, curr) * (std.math.powi(u64, 10, i) catch @panic("overflow"));
        }

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
