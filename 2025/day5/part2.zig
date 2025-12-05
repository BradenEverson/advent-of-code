const std = @import("std");
const input = @embedFile("data/input");

pub fn main() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const alloc = gpa.allocator();

    var ranges = std.ArrayList(Range){};
    defer ranges.deinit(alloc);

    var lines = std.mem.tokenizeAny(u8, input, "\n");

    while (lines.next()) |line| {
        if (!std.mem.containsAtLeast(u8, line, 1, "-")) {
            std.debug.print("Finished reading ranges\n", .{});
            break;
        }

        var split = std.mem.splitAny(u8, line, "-");
        const start = std.fmt.parseInt(u64, split.next().?, 10) catch @panic("Failed to parse");
        const end = std.fmt.parseInt(u64, split.next().?, 10) catch @panic("Failed to parse");

        ranges.append(alloc, .{ .start = start, .end = end }) catch @panic("Failed to push");
    }

    std.mem.sort(Range, ranges.items, {}, Range.compareRanges);

    var reduced = std.ArrayList(Range){};
    defer reduced.deinit(alloc);

    reduced.append(alloc, ranges.items[0]) catch @panic("Failed to push");

    for (ranges.items[1..]) |range| {
        if (reduced.items[reduced.items.len - 1].overlap(range)) {
            reduced.items[reduced.items.len - 1] = reduced.items[reduced.items.len - 1].combine(range);
        } else {
            reduced.append(alloc, range) catch @panic("Failed to push");
        }
    }

    var fresh: u64 = 0;
    for (reduced.items) |range| {
        fresh += range.rangeCount();
    }

    std.debug.print("Fresh: {}\n", .{fresh});
}

pub const Range = struct {
    start: u64,
    end: u64,

    pub fn rangeCount(self: *const Range) u64 {
        return self.end - self.start + 1;
    }

    pub fn inRange(self: *const Range, val: u64) bool {
        return val >= self.start and val <= self.end;
    }

    pub fn combine(self: *const Range, other: Range) Range {
        const start = @min(self.start, other.start);
        const end = @max(self.end, other.end);

        return .{ .start = start, .end = end };
    }

    pub fn overlap(self: *const Range, other: Range) bool {
        return self.inRange(other.start) or self.inRange(other.end);
    }

    pub fn compareRanges(context: void, a: Range, b: Range) bool {
        _ = context;
        return a.start < b.start;
    }
};
