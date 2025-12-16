const std = @import("std");
const input = @embedFile("data/input");

pub fn main() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const alloc = gpa.allocator();

    var graph = Graph{};
    defer graph.deinit(alloc);

    var lines = std.mem.tokenizeAny(u8, input, "\n");

    while (lines.next()) |line| {
        const node = Node.fromStr(line, alloc) catch @panic("Malformed input");
        graph.nodes.put(alloc, node.name, node) catch @panic("Failed to put");
    }

    const dist_svr_to_fft = graph.getPathTo(alloc, "fft", "svr") catch @panic("Failed");
    const dist_fft_to_dac = graph.getPathTo(alloc, "dac", "fft") catch @panic("Failed");
    const dist_dac_to_out = graph.getPathTo(alloc, "out", "dac") catch @panic("Failed");

    std.debug.print("{}\n", .{dist_svr_to_fft * dist_fft_to_dac * dist_dac_to_out});
}

pub const Graph = struct {
    nodes: std.StringHashMapUnmanaged(Node) = .{},
    mem: std.StringHashMapUnmanaged(u64) = .{},

    pub fn getPathTo(self: *Graph, alloc: std.mem.Allocator, to: []const u8, from: []const u8) !u64 {
        if (std.mem.eql(u8, to, from)) {
            return 1;
        }

        const query = try alloc.alloc(u8, 6);

        query[0] = to[0];
        query[1] = to[1];
        query[2] = to[2];
        query[3] = from[0];
        query[4] = from[1];
        query[5] = from[2];

        if (self.mem.get(query)) |mem| {
            alloc.free(query);
            return mem;
        }

        const node = self.nodes.get(from);

        var result: u64 = 0;

        if (node) |n| {
            for (n.connections) |conn| {
                result += try self.getPathTo(alloc, to, conn);
            }
        } else {
            try self.mem.put(alloc, query, 0);
            return 0;
        }

        try self.mem.put(alloc, query, result);
        return result;
    }

    pub fn deinit(self: *Graph, alloc: std.mem.Allocator) void {
        var items = self.nodes.valueIterator();

        while (items.next()) |node| {
            alloc.free(node.connections);
        }

        var keys = self.mem.keyIterator();

        while (keys.next()) |mem| {
            alloc.free(mem.*);
        }

        self.nodes.deinit(alloc);
        self.mem.deinit(alloc);
    }
};

pub const Node = struct {
    name: []const u8,
    connections: [][]const u8,

    pub fn fromStr(str: []const u8, alloc: std.mem.Allocator) !Node {
        const connections = str[5..];
        const len = std.mem.count(u8, connections, " ") + 1;
        const items = try alloc.alloc([]const u8, len);

        var names = std.mem.tokenizeAny(u8, connections, " ");
        var idx: usize = 0;
        while (names.next()) |name| {
            items[idx] = name;
            idx += 1;
        }

        return .{
            .name = str[0..3],
            .connections = items,
        };
    }

    pub fn deinit(self: *Node, alloc: std.mem.Allocator) void {
        alloc.free(self.connections);
    }
};
