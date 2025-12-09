const std = @import("std");
const input = @embedFile("data/input");

pub fn main() void {
    const vals: comptime_int = comptime blk: {
        @setEvalBranchQuota(100_000);
        break :blk std.mem.count(u8, input, "\n");
    };

    var points: [vals]Point = undefined;
    var lines = std.mem.tokenizeAny(u8, input, "\n");

    var idx: usize = 0;

    while (lines.next()) |line| {
        var nums = std.mem.tokenizeAny(u8, line, ",");
        const x = std.fmt.parseInt(u64, nums.next().?, 10) catch @panic("Failed to parse int");
        const y = std.fmt.parseInt(u64, nums.next().?, 10) catch @panic("Failed to parse int");

        points[idx] = Point{ .x = x, .y = y };
        idx += 1;
    }

    var largest: u64 = 0;

    for (0..vals) |i| {
        for (i + 1..vals) |j| {
            const a = points[i];
            const b = points[j];

            const area = a.areaBetween(&b);
            if (area > largest) {
                largest = area;
            }
        }
    }

    std.debug.print("Area: {d}\n", .{largest});
}

const Point = struct {
    x: u64,
    y: u64,

    pub fn areaBetween(self: *const Point, other: *const Point) u64 {
        const height = @max(self.y, other.y) - @min(self.y, other.y) + 1;
        const width = @max(self.x, other.x) - @min(self.x, other.x) + 1;

        return height * width;
    }
};
