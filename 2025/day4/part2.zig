const std = @import("std");
const input = @embedFile("data/input");

pub fn main() void {
    const height: comptime_int = comptime blk: {
        @setEvalBranchQuota(20_000);
        break :blk std.mem.count(u8, input, "\n");
    };

    const width: comptime_int = comptime blk: {
        var w = 0;
        while (input[w] != '\n') {
            w += 1;
        }

        break :blk w;
    };

    var total: u64 = 0;
    var this_run: u64 = 1;

    var grid: [height][width]u8 = undefined;
    var l: usize = 0;

    var lines = std.mem.tokenizeAny(u8, input, "\n");
    while (lines.next()) |line| {
        for (line, 0..) |char, c| {
            grid[l][c] = char;
        }

        l += 1;
    }

    while (this_run != 0) {
        this_run = 0;
        for (0..height) |line| {
            for (0..width) |col| {
                if (grid[line][col] != '@') {
                    continue;
                }

                const nearby = getCloseCount(width, height, grid, line, col);

                if (nearby < 4) {
                    this_run += 1;
                    grid[line][col] = '.';
                }
            }
        }

        total += this_run;
    }

    std.debug.print("{} rolls\n", .{total});
}

pub fn get(comptime w: comptime_int, comptime h: comptime_int, grid: [h][w]u8, line: isize, col: isize) u8 {
    if (line >= grid.len or line < 0 or col >= grid[0].len or col < 0) {
        return '.';
    } else {
        return grid[@intCast(line)][@intCast(col)];
    }
}

pub fn getCloseCount(comptime w: comptime_int, comptime h: comptime_int, grid: [h][w]u8, line: usize, col: usize) usize {
    var count: usize = 0;

    const line_s: isize = @intCast(line);
    const col_s: isize = @intCast(col);

    for (0..3) |dl| {
        const dl_s: isize = @intCast(dl);
        for (0..3) |dc| {
            const dc_s: isize = @intCast(dc);

            if (dl_s == 1 and dc_s == 1) {
                continue;
            }

            const l: isize = line_s + (dl_s - 1);
            const c: isize = col_s + (dc_s - 1);

            const at = get(w, h, grid, l, c);
            if (at == '@') {
                count += 1;
            }
        }
    }

    return count;
}
