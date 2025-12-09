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

            const start_x = @min(prev.x, curr.x);
            const end_x = @max(prev.x, curr.x);

            const start_y = @min(prev.y, curr.y);
            const end_y = @max(prev.y, curr.y);

            const top_left = Point{ .x = start_x, .y = start_y };
            const top_right = Point{ .x = end_x, .y = start_y };
            const bottom_left = Point{ .x = start_x, .y = end_y };
            const bottom_right = Point{ .x = end_x, .y = end_y };

            const line_a = Range.init(top_left, top_right);
            const line_b = Range.init(top_left, bottom_left);
            const line_c = Range.init(bottom_left, bottom_right);
            const line_d = Range.init(top_right, bottom_right);

            valid_points.append(alloc, line_a) catch @panic("Failed to append");
            valid_points.append(alloc, line_b) catch @panic("Failed to append");
            valid_points.append(alloc, line_c) catch @panic("Failed to append");
            valid_points.append(alloc, line_d) catch @panic("Failed to append");
        }

        idx += 1;
    }

    const prev = starting_points[0];
    const curr = starting_points[idx - 1];

    const start_x = @min(prev.x, curr.x);
    const end_x = @max(prev.x, curr.x);

    const start_y = @min(prev.y, curr.y);
    const end_y = @max(prev.y, curr.y);

    const top_left = Point{ .x = start_x, .y = start_y };
    const top_right = Point{ .x = end_x, .y = start_y };
    const bottom_left = Point{ .x = start_x, .y = end_y };
    const bottom_right = Point{ .x = end_x, .y = end_y };

    const line_a = Range.init(top_left, top_right);
    const line_b = Range.init(top_left, bottom_left);
    const line_c = Range.init(bottom_left, bottom_right);
    const line_d = Range.init(top_right, bottom_right);

    valid_points.append(alloc, line_a) catch @panic("Failed to append");
    valid_points.append(alloc, line_b) catch @panic("Failed to append");
    valid_points.append(alloc, line_c) catch @panic("Failed to append");
    valid_points.append(alloc, line_d) catch @panic("Failed to append");

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
    min_x: u64,
    max_x: u64,
    min_y: u64,
    max_y: u64,

    pub fn init(a: Point, b: Point) Range {
        return Range{
            .min_x = @min(a.x, b.x),
            .max_x = @max(a.x, b.x),
            .min_y = @min(a.y, b.y),
            .max_y = @max(a.y, b.y),
        };
    }

    pub fn inRange(self: *const Range, check: *const Range) bool {
        return check.min_x >= self.min_x and
            check.max_x <= self.max_x and
            check.min_y >= self.min_y and
            check.max_y <= self.max_y;
    }
};

pub fn isValid(a: Point, b: Point, valid_points: []const Range) bool {
    const start_x = @min(a.x, b.x);
    const end_x = @max(a.x, b.x);

    const start_y = @min(a.y, b.y);
    const end_y = @max(a.y, b.y);

    const top_left = Point{ .x = start_x, .y = start_y };
    const top_right = Point{ .x = end_x, .y = start_y };
    const bottom_left = Point{ .x = start_x, .y = end_y };
    const bottom_right = Point{ .x = end_x, .y = end_y };

    const line_a = Range.init(top_left, top_right);
    const line_b = Range.init(top_left, bottom_left);
    const line_c = Range.init(bottom_left, bottom_right);
    const line_d = Range.init(top_right, bottom_right);

    var a_valid = false;
    var b_valid = false;
    var c_valid = false;
    var d_valid = false;

    for (valid_points) |range| {
        if (range.inRange(&line_a)) {
            a_valid = true;
        }

        if (range.inRange(&line_b)) {
            b_valid = true;
        }

        if (range.inRange(&line_c)) {
            c_valid = true;
        }

        if (range.inRange(&line_d)) {
            d_valid = true;
        }
    }

    return a_valid and b_valid and c_valid and d_valid;
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
