const std = @import("std");
const Io = std.Io;

const data = @embedFile("data/input");
const LEN: usize = 12;

pub fn main() !void {
    var iter = std.mem.splitAny(u8, data, "\n");

    var bit_freqs: [LEN]i32 = @splat(0);

    while (iter.next()) |line| {
        for (line, 0..) |l, i| {
            if (l == '1')
                bit_freqs[i] += 1
            else
                bit_freqs[i] -= 1;
        }
    }

    var gamma: usize = 0;
    var epsilon: usize = 0;

    for (bit_freqs) |freq| {
        gamma <<= 1;
        gamma |= if (freq > 0) 1 else 0;

        epsilon <<= 1;
        epsilon |= if (freq > 0) 0 else 1;
    }

    std.debug.print("{} * {} = {}\n", .{ gamma, epsilon, gamma * epsilon });
}
