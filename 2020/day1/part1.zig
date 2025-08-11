const std = @import("std");
const input = @embedFile("data/input");

fn inputToIntList(comptime buf: []const u8, al: *std.ArrayList(u32)) !void {
    var tokenizer = std.mem.tokenizeAny(u8, buf, " \n");
    while (tokenizer.next()) |tok| {
        const val = try std.fmt.parseInt(u32, tok, 10);
        try al.append(val);
    }
}

fn contains(comptime T: type, haystack: []const T, needle: T) bool {
    for (haystack) |item| {
        if (item == needle) {
            return true;
        }
    }

    return false;
}

pub fn main() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const alloc = gpa.allocator();
    var nums = std.ArrayList(u32).init(alloc);
    defer nums.deinit();

    inputToIntList(input, &nums) catch @panic("Failed to parse list");
    for (nums.items) |num| {
        const look_for = 2020 - num;
        if (contains(u32, nums.items, look_for)) {
            std.debug.print("We found em: {} x {} = {}\n", .{ num, look_for, num * look_for });
            std.posix.exit(0);
        }
    }
}
