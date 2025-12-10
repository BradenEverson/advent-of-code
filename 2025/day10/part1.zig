const std = @import("std");
const input = @embedFile("data/test");

pub fn main() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const alloc = gpa.allocator();

    var lines = std.mem.tokenizeAny(u8, input, "\n");

    while (lines.next()) |line| {
        const cols = std.mem.count(u8, line, " ") - 1;
        var solve = alloc.alloc([]const usize, cols) catch unreachable;
        defer {
            for (solve) |elem| {
                alloc.free(elem);
            }

            alloc.free(solve);
        }

        var components = std.mem.tokenizeAny(u8, line, " ");
        const end_state = components.next().?;

        var end_idx: usize = 0;
        const size = end_state.len - 2;
        var end = alloc.alloc(bool, size) catch @panic("Failed to alloc");
        defer alloc.free(end);

        for (end_state) |char| {
            if (char == '.') {
                end[end_idx] = false;
                end_idx += 1;
            } else if (char == '#') {
                end[end_idx] = true;
                end_idx += 1;
            }
        }

        var solve_idx: usize = 0;

        while (components.next()) |comp| {
            if (std.mem.indexOf(u8, comp, "{") == null) {
                const elems = std.mem.count(u8, comp, ",") + 1;
                var indices: []usize = alloc.alloc(usize, elems) catch @panic("Alloc failed");
                var idx: usize = 0;

                const subspace = comp[1 .. comp.len - 1];
                for (subspace) |c| {
                    if (c != ',') {
                        indices[idx] = c - '0';
                        idx += 1;
                    }
                }

                solve[solve_idx] = indices;
                solve_idx += 1;
            }
        }

        // end - end state
        // solve - options we have to click to reach end state
        // time to do a bfs
        const current_state = alloc.alloc(bool, end.len) catch @panic("Failed to alloc");
        defer alloc.free(current_state);
        for (current_state) |*val| val.* = false;

        const min = minPresses(alloc, end, solve, current_state) catch @panic("Failed to find min");
        std.debug.print("Min: {}\n", .{min});
    }
}

pub fn minPresses(alloc: std.mem.Allocator, desired: []const bool, presses: [][]const usize, current_state: []const bool) !u64 {
    _ = alloc;
    _ = desired;
    _ = presses;
    _ = current_state;

    return 0;
}
