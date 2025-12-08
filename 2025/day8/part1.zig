const std = @import("std");
const input = @embedFile("data/input");

const TEST_MAX: usize = 10;
const MAX: usize = 1000;

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

        const x = std.fmt.parseFloat(f32, xyz.next().?) catch @panic("Failed to parse");
        const y = std.fmt.parseFloat(f32, xyz.next().?) catch @panic("Failed to parse");
        const z = std.fmt.parseFloat(f32, xyz.next().?) catch @panic("Failed to parse");

        points[idx] = Point{ .x = x, .y = y, .z = z };
        idx += 1;
    }

    for (0..points.len) |i| {
        for (i + 1..points.len) |j| {
            dists.append(alloc, Pair.init(&points[i], &points[j])) catch @panic("Failed to push");
        }
    }

    std.mem.sort(Pair, dists.items, {}, Pair.compare);

    var connections: usize = 0;

    for (dists.items) |dist| {
        if (!dist.a.pointsTo(dist.b)) {
            dist.a.connections.append(alloc, dist.b) catch @panic("Failed to push");
            dist.b.connections.append(alloc, dist.a) catch @panic("Failed to push");

            connections += 1;
            if (connections == MAX) break;
        }
    }

    var seen = std.ArrayList(*Point){};
    defer seen.deinit(alloc);

    var sizes = std.ArrayList(usize){};
    defer sizes.deinit(alloc);

    for (&points) |*point| {
        if (std.mem.indexOf(*Point, seen.items, &[_]*Point{point}) != null) {
            continue;
        }

        var size: u64 = 0;
        sizeOfCircuit(point, &size, &seen, alloc) catch @panic("Graph traveler is messed up");
        sizes.append(alloc, size) catch @panic("Failed to add size");
    }

    std.mem.sort(u64, sizes.items, {}, compareU64);

    var total = sizes.items[0];

    for (sizes.items[1..3]) |size| {
        total *= size;
    }

    std.debug.print("{}\n", .{total});
}

pub fn compareU64(context: void, a: u64, b: u64) bool {
    _ = context;
    return a > b;
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
    x: f32,
    y: f32,
    z: f32,

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

    pub fn dist(self: *const Point, other: *const Point) f32 {
        const x = std.math.pow(f32, self.x - other.x, 2);
        const y = std.math.pow(f32, self.y - other.y, 2);
        const z = std.math.pow(f32, self.z - other.z, 2);

        return std.math.sqrt(x + y + z);
    }
};

pub const Pair = struct {
    a: *Point,
    b: *Point,
    dist: f32,
    pub fn init(a: *Point, b: *Point) Pair {
        return Pair{ .a = a, .b = b, .dist = a.dist(b) };
    }

    pub fn compare(context: void, a: Pair, b: Pair) bool {
        _ = context;
        return a.dist < b.dist;
    }
};
