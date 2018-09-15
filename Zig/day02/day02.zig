const std = @import("std");
const warn = std.debug.warn;

fn solve() [2] usize {
    comptime var f = comptime @embedFile("Input.txt");
    var len = f.len;

    var a: [16] usize = [1]usize{0} ** 16;
    var a_idx: usize = 0;

    var n: usize = 0;

    var min: usize = @maxValue(usize);
    var max: usize = 0;

    var quotient: usize = 0;

    var part1: usize = 0;
    var part2: usize = 0;

    for(f) |c| {
        if (c >= '0') {
            n = n * 10 + c - '0';
        } else {
            if (c == '\r') { continue; }
            
            if (n < min) { min = n; }
            if (n > max) { max = n; }

            if (quotient == 0) {
                for(a[0..a_idx]) |*item| {
                    if (n % item.* == 0) {
                        quotient = n / item.*;
                        break;
                    }

                    if(item.* % n == 0) {
                        quotient = item.* / n;
                        break;
                    }
                }
            }
            
            if (c != '\n') {
                a[a_idx] = n;
                a_idx += 1;
            }

            else {
                part1 += max - min;
                part2 += quotient;

                min = @maxValue(usize);
                max = 0;
                a_idx = 0;
                quotient = 0;
            }

            n = 0;
        }
    }

    return [2] usize {part1, part2};
}

pub fn main() void {
    comptime @setEvalBranchQuota(5000);

    const arr = comptime solve();

    warn("{} {}", arr[0], arr[1]);
}