const std = @import("std");
const input = @embedFile("data/input");

pub fn main() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const alloc = gpa.allocator();

    var ranges = std.ArrayList(Range){};
    defer ranges.deinit(alloc);

    var lines = std.mem.tokenizeAny(u8, input, "\n");
    var done = false;

    var fresh: u64 = 0;

    while (lines.next()) |line| {
        if (!done and !std.mem.containsAtLeast(u8, line, 1, "-")) {
            std.debug.print("Finished reading ranges\n", .{});
            done = true;
        }

        if (!done) {
            var split = std.mem.splitAny(u8, line, "-");
            const start = std.fmt.parseInt(u64, split.next().?, 10) catch @panic("Failed to parse");
            const end = std.fmt.parseInt(u64, split.next().?, 10) catch @panic("Failed to parse");

            ranges.append(alloc, .{ .start = start, .end = end }) catch @panic("Failed to push");
        } else {
            const val = std.fmt.parseInt(u64, line, 10) catch @panic("Failed to parse");

            for (ranges.items) |range| {
                if (range.inRange(val)) {
                    fresh += 1;
                    break;
                }
            }
        }
    }

    std.debug.print("Fresh: {d}\n", .{fresh});
}

pub const Range = struct {
    start: u64,
    end: u64,

    pub fn inRange(self: *const Range, val: u64) bool {
        return val >= self.start and val <= self.end;
    }
};
