const std = @import("std");
const input = @embedFile("data/input");

fn charCount(msg: []const u8, find: u8) u32 {
    var count: u32 = 0;
    for (msg) |c| {
        if (c == find) {
            count += 1;
        }
    }

    return count;
}

fn inputToParsedList(comptime buf: []const u8, parsed: *std.ArrayList(PasswordTest)) !void {
    var tokenizer = std.mem.tokenizeAny(u8, buf, "\n");

    while (tokenizer.next()) |tok| {
        try parsed.append(PasswordTest.parse(tok));
    }
}

const PasswordTest = struct {
    min: u32,
    max: u32,
    char: u8,

    pwrd: []const u8,

    pub fn parse(from: []const u8) PasswordTest {
        var tokenizer = std.mem.tokenizeAny(u8, from, "- :");

        const min = tokenizer.next().?;
        const max = tokenizer.next().?;

        const char = tokenizer.next().?[0];
        const pwrd = tokenizer.next().?;

        return PasswordTest{
            .min = std.fmt.parseInt(u32, min, 10) catch unreachable,
            .max = std.fmt.parseInt(u32, max, 10) catch unreachable,
            .char = char,
            .pwrd = pwrd,
        };
    }

    pub fn passesTest(self: *const PasswordTest) bool {
        const count = charCount(self.pwrd, self.char);
        return count >= self.min and count <= self.max;
    }
};

pub fn main() void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const alloc = gpa.allocator();
    var tests = std.ArrayList(PasswordTest).init(alloc);
    defer tests.deinit();

    inputToParsedList(input, &tests) catch @panic("Failed to parse list");

    var valid: u32 = 0;

    for (tests.items) |pwrd_test| {
        if (pwrd_test.passesTest()) {
            valid += 1;
        }
    }

    std.debug.print("Valid Passwords: {}\n", .{valid});
}
