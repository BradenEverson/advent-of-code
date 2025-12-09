const std = @import("std");
const input = @embedFile("data/test");

pub fn main() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const alloc = gpa.allocator();

    const vals: comptime_int = comptime blk: {
        @setEvalBranchQuota(100_000);
        break :blk std.mem.count(u8, input, "\n");
    };

    var starting_points: [vals]Point = undefined;

    var valid_points = std.ArrayList(Range){};
    defer valid_points.deinit(alloc);

    var lines = std.mem.tokenizeAny(u8, input, "\n");

    var idx: usize = 0;

    while (lines.next()) |line| {
        var nums = std.mem.tokenizeAny(u8, line, ",");
        const x = std.fmt.parseInt(u64, nums.next().?, 10) catch @panic("Failed to parse int");
        const y = std.fmt.parseInt(u64, nums.next().?, 10) catch @panic("Failed to parse int");

        starting_points[idx] = Point{ .x = x, .y = y };

        // register valid points
        if (idx > 0) {
            const prev = starting_points[idx - 1];
            const curr = starting_points[idx];

            const l = Range.init(prev, curr);

            valid_points.append(alloc, l) catch @panic("Failed to append");
        }

        idx += 1;
    }

    const prev = starting_points[0];
    const curr = starting_points[idx - 1];

    const l = Range.init(prev, curr);

    valid_points.append(alloc, l) catch @panic("Failed to append");

    var largest: u64 = 0;

    for (0..vals) |i| {
        for (i + 1..vals) |j| {
            const a = starting_points[i];
            const b = starting_points[j];

            const area = a.areaBetween(&b);

            if (area > largest and isValid(a, b, valid_points.items)) {
                largest = area;
            }
        }
    }

    std.debug.print("Area: {d}\n", .{largest});
}

const Range = struct {
    a: Point,
    b: Point,
    horizontal: bool,

    pub fn init(a: Point, b: Point) Range {
        const horizontal = a.y == b.y;

        return Range{
            .a = a,
            .b = b,
            .horizontal = horizontal,
        };
    }

    pub fn containsPoint(self: *const Range, a: Point) bool {
        if (self.horizontal) {
            const min = @min(self.a.x, self.b.x);
            const max = @max(self.a.x, self.b.x);
            return a.y == self.a.y and a.x >= min and a.x <= max;
        } else {
            const min = @min(self.a.y, self.b.y);
            const max = @max(self.a.y, self.b.y);
            return a.x == self.a.x and a.y >= min and a.y <= max;
        }
    }
};

pub fn someRangeContains(point: Point, edges: []const Range) bool {
    for (edges) |edge| {
        if (edge.containsPoint(point)) {
            return true;
        }
    }

    return false;
}

pub fn pointInPolygon(point: Point, edges: []const Range) bool {
    const px = point.x;
    const py = point.y;

    var inside = false;

    for (0..@intCast(px)) |offset| {
        const dx = px - offset;
        const p = Point{ .x = dx, .y = py };

        if (someRangeContains(p, edges)) {
            inside = !inside;
        }
    }

    return inside;
}

pub fn isValid(a: Point, b: Point, valid_points: []const Range) bool {
    const start_x = @min(a.x, b.x);
    const end_x = @max(a.x, b.x);

    const start_y = @min(a.y, b.y);
    const end_y = @max(a.y, b.y);

    for (0..(end_x - start_x)) |dx| {
        const t = Point{ .x = start_x + dx, .y = start_y };
        if (!pointInPolygon(t, valid_points)) {
            return false;
        }
    }

    for (0..(end_x - start_x)) |dx| {
        const t = Point{ .x = start_x + dx, .y = end_y };
        if (!pointInPolygon(t, valid_points)) {
            return false;
        }
    }

    for (0..(end_y - start_y)) |dy| {
        const t = Point{ .x = start_x, .y = start_y + dy };
        if (!pointInPolygon(t, valid_points)) {
            return false;
        }
    }

    for (0..(end_y - start_y)) |dy| {
        const t = Point{ .x = end_x, .y = start_y + dy };
        if (!pointInPolygon(t, valid_points)) {
            return false;
        }
    }

    return true;
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
