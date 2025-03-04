const std = @import("std");
const data = @embedFile("data/input");

const StrResult = enum {
    Two,
    Three,
    Both,
    None,
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer {
        _ = gpa.deinit();
    }

    var vals = std.mem.split(u8, data, "\n");

    var twos: usize = 0;
    var threes: usize = 0;

    var list = std.ArrayList([]const u8).init(allocator);
    defer list.deinit();

    while (vals.next()) |val| {
        if (val.len == 0) continue;
        const tag = findDupes(val);

        switch (tag) {
            StrResult.Two => twos += 1,
            StrResult.Three => threes += 1,
            StrResult.Both => {
                twos += 1;
                threes += 1;
            },
            StrResult.None => {},
        }

        try list.append(val);
    }

    const total = twos * threes;
    std.debug.print("Checksum: {d}\n", .{total});

    // Part 2
    for (list.items, 0..) |val, i| {
        for (list.items, 0..) |cmp, j| {
            if (i == j) continue;

            const differences = numDifferences(val, cmp);
            if (differences == 1) {
                std.debug.print("{s}\n{s}\n", .{ val, cmp });
                return;
            }
        }
    }

    // srijafjzloguvlntqmphenbkd
}

pub fn numDifferences(cmp: []const u8, to: []const u8) u32 {
    var differences: u32 = 0;
    for (cmp, 0..) |val, idx| {
        if (val != to[idx]) {
            differences += 1;
        }
    }

    return differences;
}

pub fn findDupes(str: []const u8) StrResult {
    var charCounts: [26]u32 = std.mem.zeroes([26]u32);

    for (str) |char| {
        charCounts[char - 'a'] += 1;
    }

    var three: bool = false;
    var two: bool = false;

    for (charCounts) |count| {
        if (count == 3) {
            three = true;
        } else if (count == 2) {
            two = true;
        }
    }

    if (three and !two) {
        return StrResult.Three;
    } else if (!three and two) {
        return StrResult.Two;
    } else if (three and two) {
        return StrResult.Both;
    } else {
        return StrResult.None;
    }
}
