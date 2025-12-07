const std = @import("std");
const input = @embedFile("data/input");

pub fn main() void {
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

    var grid: [height][width]i64 = undefined;

    var lines = std.mem.tokenizeAny(u8, input, "\n");

    var idx: usize = 0;

    while (lines.next()) |line| {
        for (line, 0..) |char, c| {
            switch (char) {
                'S' => grid[idx][c] = 1,
                '^' => grid[idx][c] = -1,
                '.' => grid[idx][c] = 0,
                else => unreachable,
            }
        }

        if (idx > 0) {
            eval(&grid[idx - 1], &grid[idx]);
        }

        idx += 1;
    }

    var timelines: i64 = 0;

    for (grid[height - 1]) |val| {
        timelines += val;
    }

    std.debug.print("Timelines: {}\n", .{timelines});
}

pub fn eval(prev: []const i64, curr: []i64) void {
    for (0..prev.len) |i| {
        switch (prev[i]) {
            0 => {},
            -1 => {},
            else => {
                switch (curr[i]) {
                    0 => curr[i] = prev[i],
                    -1 => {
                        curr[i - 1] += prev[i];
                        curr[i + 1] = prev[i];
                    },
                    else => {
                        curr[i] += prev[i];
                    },
                }
            },
        }
    }
}
