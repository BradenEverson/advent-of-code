const std = @import("std");
const input = @embedFile("data/input");

pub fn main() void {
    var splits: u64 = 0;

    const height: comptime_int = comptime blk: {
        @setEvalBranchQuota(100_000);
        break :blk std.mem.count(u8, input, "\n");
    };

    const width: comptime_int = comptime blk: {
        var w = 0;
        while (input[w] != '\n') {
            w += 1;
        }

        break :blk w;
    };

    var grid: [height][width]u8 = undefined;

    var lines = std.mem.tokenizeAny(u8, input, "\n");

    var idx: usize = 0;

    while (lines.next()) |line| {
        for (line, 0..) |char, c| {
            grid[idx][c] = char;
        }

        if (idx > 0) {
            splits += eval(&grid[idx - 1], &grid[idx]);
        }

        std.debug.print("{s}\n", .{grid[idx]});

        idx += 1;
    }

    std.debug.print("Total Splits: {d}\n", .{splits});
}

pub fn eval(prev: []const u8, curr: []u8) u64 {
    var splits: u64 = 0;

    for (0..prev.len) |i| {
        switch (prev[i]) {
            '.' => {},
            '^' => {},
            '|', 'S' => {
                switch (curr[i]) {
                    '^' => {
                        splits += 1;
                        curr[i - 1] = '|';
                        curr[i + 1] = '|';
                    },
                    '.' => curr[i] = '|',
                    '|' => {},
                    else => unreachable,
                }
            },
            else => unreachable,
        }
    }

    return splits;
}
