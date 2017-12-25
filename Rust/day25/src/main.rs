mod state;
use state::*;

fn main() {
    let mut cpu = CPU::new();
    for _ in 0..12794428 {
        cpu.run();
    }
    println!("{:?}", cpu.count_one());
}