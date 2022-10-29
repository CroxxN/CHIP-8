#![allow(non_snake_case)]

mod add;

use add::CPU;

fn main() {
    let mut cpu = CPU::new(0x8014, 10, 15);
    cpu.run();
}

