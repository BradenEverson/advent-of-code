const std = @import("std");
const input = @embedFile("data/input");

pub fn main() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const alloc = gpa.allocator();

    const vals: comptime_int = comptime blk: {
        @setEvalBranchQuota(100_000);
        break :blk std.mem.count(u8, input, "\n");
    };

    var lines = std.mem.tokenizeAny(u8, input, "\n");
    var points: [vals]Point = undefined;
    defer {
        for (&points) |*point| {
            point.deinit(alloc);
        }
    }

    var dists = std.ArrayList(Pair){};
    defer dists.deinit(alloc);

    var idx: usize = 0;
    while (lines.next()) |line| {
        var xyz = std.mem.tokenizeAny(u8, line, ",");

        const x = std.fmt.parseInt(i64, xyz.next().?, 10) catch @panic("Failed to parse");
        const y = std.fmt.parseInt(i64, xyz.next().?, 10) catch @panic("Failed to parse");
        const z = std.fmt.parseInt(i64, xyz.next().?, 10) catch @panic("Failed to parse");

        points[idx] = Point{ .x = x, .y = y, .z = z };
        idx += 1;
    }

    for (0..points.len) |i| {
        for (i + 1..points.len) |j| {
            dists.append(alloc, Pair.init(&points[i], &points[j])) catch @panic("Failed to push");
        }
    }

    std.mem.sort(Pair, dists.items, {}, Pair.compare);

    var seen = std.ArrayList(*Point){};
    defer seen.deinit(alloc);

    const need: u64 = @intCast(points.len);

    for (dists.items) |dist| {
        if (!dist.a.pointsTo(dist.b)) {
            dist.a.connections.append(alloc, dist.b) catch @panic("Failed to push");
            dist.b.connections.append(alloc, dist.a) catch @panic("Failed to push");

            var size: u64 = 0;
            sizeOfCircuit(dist.a, &size, &seen, alloc) catch @panic("Failed to get size");

            if (size == need) {
                // 3206508800 - too low
                // .......... it was too low cause i was using floats...
                // that was 2 hours of debugging...

                std.debug.print("Final Connection reached!\n{}\n", .{dist.a.x * dist.b.x});
                break;
            }

            while (seen.pop()) |_| {}
        }
    }
}

pub fn sizeOfCircuit(start: *Point, val: *u64, seen: *std.ArrayList(*Point), alloc: std.mem.Allocator) !void {
    for (start.connections.items) |connection| {
        if (std.mem.indexOf(*Point, seen.items, &[_]*Point{connection}) != null) {
            continue;
        }

        val.* += 1;
        try seen.append(alloc, connection);
        try sizeOfCircuit(connection, val, seen, alloc);
    }
}

pub const Point = struct {
    x: i64,
    y: i64,
    z: i64,

    connections: std.ArrayList(*Point) = .{},

    pub fn pointsTo(self: *Point, other: *Point) bool {
        for (self.connections.items) |p| {
            if (std.meta.eql(p, other)) {
                return true;
            }
        }

        return false;
    }

    pub fn deinit(self: *Point, alloc: std.mem.Allocator) void {
        self.connections.deinit(alloc);
    }

    pub fn dist(self: *const Point, other: *const Point) i64 {
        const x = std.math.pow(i64, self.x - other.x, 2);
        const y = std.math.pow(i64, self.y - other.y, 2);
        const z = std.math.pow(i64, self.z - other.z, 2);

        const res = x + y + z;

        const res_f: f32 = @floatFromInt(res);

        return @intFromFloat(std.math.sqrt(res_f));
    }
};

pub const Pair = struct {
    a: *Point,
    b: *Point,
    dist: i64,
    pub fn init(a: *Point, b: *Point) Pair {
        return Pair{ .a = a, .b = b, .dist = a.dist(b) };
    }

    pub fn compare(context: void, a: Pair, b: Pair) bool {
        _ = context;
        return a.dist < b.dist;
    }
};
