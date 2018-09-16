const std = @import("std");
const warn = std.debug.warn;

const INPUT: usize = 361527;

const direction = enum {
    UP,
    DOWN,
    LEFT,
    RIGHT,

    fn turn(self: *@This()) void {
        switch (self.*) {
            direction.UP => {
                self.* = direction.LEFT;
            },
            direction.LEFT => {
                self.* = direction.DOWN;
            },
            direction.DOWN => {
                self.* = direction.RIGHT;
            },
            direction.RIGHT => {
                self.* = direction.UP;
            },
            else => {}
        }
    }
};

const position = struct {
    x: i64,
    y: i64,

    fn update(self: *position, direct: *const direction) void {
        switch (direct.*) {
            direction.UP => {
                self.*.y += 1;
            },
            direction.DOWN => {
                self.*.y -= 1;
            },
            direction.RIGHT => {
                self.*.x += 1;
            },
            direction.LEFT => {
                self.*.x -= 1;
            },
        }
    }
};

fn iter(comptime n: usize) [n] void {
    
    return undefined;
}

fn solve() i64 {
    var current: usize = 0;

    var direct = direction.RIGHT;

    var stepsz: usize = 1;

    var pos = position {.x = 0, .y = 0};

    while (true) {
        for(iter(2)) |_| {
            var start: usize = 0;
            
            while(start != stepsz) : ({start += 1;}) {

                current += 1;

                if(current == INPUT) {
                    if (pos.x < 0) {
                        pos.x *= -1;
                    }

                    if (pos.y < 0) {
                        pos.y *= -1;
                    }
                    return pos.x + pos.y;
                }
                
                pos.update(&direct);
            }
            direction.turn(&direct);
        }
        stepsz += 1;
    }
}

pub fn main() void {
    comptime @setEvalBranchQuota(10000000);

    const part1: i64 = comptime solve();

    warn("Part 1 = {}\n", part1);
}