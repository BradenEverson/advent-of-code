const std = @import("std");
const data = @embedFile("data/input");

const StrResult = enum {
    Two,
    Three,
    Both,
    None,
};

pub fn main() void {
    var vals = std.mem.split(u8, data, "\n");

    var twos: usize = 0;
    var threes: usize = 0;

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
    }

    const total = twos * threes;
    std.debug.print("Checksum: {d}\n", .{total});
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
