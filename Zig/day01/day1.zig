//zig.exe --release-fast build-exe

const std = @import("std");
const warn = std.debug.warn;

fn solve() [2]u64 {
    const f = @embedFile("Input.txt");

    comptime var part1: u64 = 0;
    comptime var part2: u64 = 0;

    comptime var start = 0;
    comptime var next = 1;
    comptime var half = f.len >> 1;

    while (half != f.len): ({start += 1; next += 1; half += 1;}) {

        if (f[start] == f[next]) {
            part1 += f[start] - 48;
        }

        if (f[start] == f[half]) {
            part2 += f[start] - 48;
        }
    }
    
    while (next != f.len): ({start += 1; next += 1;}) {
        if (f[start] == f[next]) {
            part1 += f[start] - 48;
        }
    }

    if (f[start] == f[0]) {
        part1 += f[0] - 48;
    }
    return [2]u64 {part1, part2 * 2};
}

pub fn main() void {
    comptime @setEvalBranchQuota(4500);
    const solved = comptime solve();

    warn("part1: {}\n", solved[0]);
    warn("part2: {}\n", solved[1]);
}