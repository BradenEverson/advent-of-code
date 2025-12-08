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

    var lines = std.mem.tokenizeAny(u8, input, "\n");
    var points: [vals]Point = undefined;

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
        const a = points[i];
        for (i + 1..points.len) |j| {
            const b = points[j];
            dists.append(alloc, Pair.init(a, b)) catch @panic("Failed to push");
        }
    }

    std.mem.sort(Pair, dists.items, {}, Pair.compare);
    var circuit = Circuits{};
    defer circuit.deinit(alloc);

    var connections: usize = 0;

    for (dists.items) |dist| {
        const res = circuit.connect(alloc, dist) catch @panic("Failed to push circuit");
        if (res) {
            connections += 1;
            if (connections == 10) {
                break;
            }
        }
    }

    std.mem.sort(Circuit, circuit.connections.items, {}, Circuit.compare);
    var total: u64 = circuit.connections.items[0].points.items.len;

    for (circuit.connections.items[1..3]) |c| {
        total *= c.points.items.len;
    }

    std.debug.print("Total: {}\n", .{total});
}

pub const Point = struct {
    x: f32,
    y: f32,
    z: f32,

    pub fn dist(self: *const Point, other: Point) f32 {
        const x = std.math.pow(f32, self.x - other.x, 2);
        const y = std.math.pow(f32, self.y - other.y, 2);
        const z = std.math.pow(f32, self.z - other.z, 2);

        return std.math.sqrt(x + y + z);
    }
};

pub const Pair = struct {
    a: Point,
    b: Point,
    dist: f32,
    pub fn init(a: Point, b: Point) Pair {
        return Pair{ .a = a, .b = b, .dist = a.dist(b) };
    }

    pub fn compare(context: void, a: Pair, b: Pair) bool {
        _ = context;
        return a.dist < b.dist;
    }
};

pub const Circuits = struct {
    connections: std.ArrayList(Circuit) = .{},

    pub fn connect(self: *Circuits, alloc: std.mem.Allocator, pair: Pair) !bool {
        var added = false;

        for (self.connections.items) |*circuit| {
            if (circuit.alreadyHas(pair)) {
                return false;
            }

            const try_append = try circuit.add(alloc, pair.a, pair.b) or try circuit.add(alloc, pair.b, pair.a);
            if (try_append) {
                added = true;
                break;
            }
        }

        if (!added) {
            var circuit = Circuit{};
            try circuit.points.append(alloc, pair.a);
            try circuit.points.append(alloc, pair.b);

            try self.connections.append(alloc, circuit);
        }

        return true;
    }

    pub fn deinit(self: *Circuits, alloc: std.mem.Allocator) void {
        for (self.connections.items) |*conn| {
            conn.deinit(alloc);
        }
        self.connections.deinit(alloc);
    }
};

pub const Circuit = struct {
    points: std.ArrayList(Point) = .{},

    pub fn alreadyHas(self: *Circuit, pair: Pair) bool {
        return contains(self.points.items, pair.a) and contains(self.points.items, pair.b);
    }

    pub fn add(self: *Circuit, alloc: std.mem.Allocator, a: Point, b: Point) !bool {
        if (contains(self.points.items, a) and !contains(self.points.items, b)) {
            try self.points.append(alloc, b);
            return true;
        }

        return false;
    }

    pub fn deinit(self: *Circuit, alloc: std.mem.Allocator) void {
        self.points.deinit(alloc);
    }

    pub fn compare(context: void, a: Circuit, b: Circuit) bool {
        _ = context;
        return a.points.items.len > b.points.items.len;
    }
};

pub fn contains(vals: []const Point, find: Point) bool {
    for (vals) |val| {
        if (std.meta.eql(val, find)) {
            return true;
        }
    }

    return false;
}
