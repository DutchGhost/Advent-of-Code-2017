//zig.exe --release-fast build-exe

const std = @import("std");
const warn = std.debug.warn;

fn solve() [2]u64 {
    
    const f = @embedFile("Input.txt");

    var part1: u64 = 0;
    var part2: u64 = 0;

    var start: usize = f.len - 1;
    var next: usize = 0;
    var half = f.len >> 1;

    while (half != f.len): ({start = next; next += 1; half += 1;}) {

        part1 += @intCast(u64, (f[start] - 48) & @bitCast(u8, -@intCast(i8, @boolToInt(f[start] == f[next]))));
        part2 += @intCast(u64, (f[next]  - 48) & @bitCast(u8, -@intCast(i8, @boolToInt(f[next]  == f[half]))));

    }
    
    while (next != f.len): ({start += 1; next += 1;}) {
        part1 += @intCast(u64, (f[start] - 48) & @bitCast(u8, -@intCast(i8, @boolToInt(f[start] == f[next]))));
    }

    return [2]u64 {part1, part2 * 2};
}

pub fn main() void {
    comptime @setEvalBranchQuota(2200);
    const solved = comptime solve();

    warn("part1: {}\n", solved[0]);
    warn("part2: {}\n", solved[1]);
}