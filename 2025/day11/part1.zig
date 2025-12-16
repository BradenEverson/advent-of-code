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

    const dist = graph.getPathTo("out", "you");
    std.debug.print("Paths to `out` from `you`: {}\n", .{dist});
}

pub const Graph = struct {
    nodes: std.StringHashMapUnmanaged(Node) = .{},

    pub fn getPathTo(self: *Graph, to: []const u8, from: []const u8) u64 {
        if (std.mem.eql(u8, to, from)) {
            return 1;
        }

        const node = self.nodes.get(from);

        var result: u64 = 0;

        if (node) |n| {
            for (n.connections) |conn| {
                result += self.getPathTo(to, conn);
            }
        } else {
            return 0;
        }

        return result;
    }

    pub fn deinit(self: *Graph, alloc: std.mem.Allocator) void {
        var items = self.nodes.valueIterator();

        while (items.next()) |node| {
            alloc.free(node.connections);
        }

        self.nodes.deinit(alloc);
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
