const std = @import("std");
const data = @embedFile("data/input");

pub fn main() !void {
    const alloc = std.heap.page_allocator;

    var seen = std.AutoHashMap(i32, void).init(alloc);
    defer seen.deinit();
    try seen.put(0, {});

    var total: i32 = 0;

    while (true) {
        var vals = std.mem.split(u8, data, "\n");
        while (vals.next()) |line| {
            const val = std.fmt.parseInt(i32, line, 10) catch continue;
            total += val;

            if (seen.contains(total)) {
                std.debug.print("Frequency Found Twice: {d}\n", .{total});
                return;
            } else {
                try seen.put(total, {});
            }
        }
    }
}
