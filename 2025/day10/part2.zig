const std = @import("std");
const input = @embedFile("data/input");

pub fn main() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const alloc = gpa.allocator();

    var lines = std.mem.tokenizeAny(u8, input, "\n");

    var total_min: u64 = 0;

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

        const current_state = alloc.alloc(bool, end.len) catch @panic("Failed to alloc");
        defer alloc.free(current_state);

        const combinations = allCombinations(alloc, solve.len) catch @panic("Failed to alloc");
        defer freeCombinations(alloc, combinations);

        var curr_min = solve.len + 1;

        for (combinations) |combo| {
            for (current_state) |*val| val.* = false;
            pressAll(current_state, solve, combo);

            if (std.mem.eql(bool, current_state, end)) {
                const presses = trueCount(combo);
                if (presses < curr_min) {
                    curr_min = presses;
                }
            }
        }

        total_min += curr_min;
    }

    std.debug.print("Total min presses: {}\n", .{total_min});
}

pub fn trueCount(combo: []const bool) u64 {
    var res: u64 = 0;
    for (combo) |c| {
        if (c) res += 1;
    }

    return res;
}

pub fn freeCombinations(alloc: std.mem.Allocator, combos: [][]const bool) void {
    for (combos) |combo| {
        alloc.free(combo);
    }

    alloc.free(combos);
}

pub fn allCombinations(alloc: std.mem.Allocator, len: usize) ![][]const bool {
    const possibilities = try std.math.powi(usize, 2, len);

    const buf = try alloc.alloc([]const bool, possibilities);
    var idx: usize = 0;

    for (0..possibilities) |val| {
        const combo = try alloc.alloc(bool, len);
        for (0..len) |i| {
            combo[i] = val >> @truncate(i) & 0x01 == 1;
        }

        buf[idx] = combo;
        idx += 1;
    }

    return buf;
}

pub fn pressAll(state: []bool, buttons: [][]const usize, selections: []const bool) void {
    for (0..buttons.len) |i| {
        if (selections[i]) {
            press(state, buttons[i]);
        }
    }
}

pub fn press(state: []bool, button: []const usize) void {
    for (button) |i| {
        state[i] = !state[i];
    }
}
