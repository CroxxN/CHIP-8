#![allow(non_snake_case)]

mod executor;

use executor::CPU;

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        pos_in_memory: 0,
        memory: [0; 4096],
    };

    // Sets values in the register
    cpu.registers[0] = 5;
    cpu.registers[1] = 10;
    cpu.registers[2] = 23;
    cpu.registers[3] = 10;
    cpu.registers[5] = 2;

    // sets instructions in the memory
    // as each memory is u8 type, it can hold 8-bit value
    // so we split the instructions across two memory address

    // reads "add the value of register 1 to that of register 0"
    // and save it to register 0
    cpu.memory[0] = 0x80;
    cpu.memory[1] = 0x14;

    //reads "add the value of register 5 to that of register 2"
    // and save it in register 2
    cpu.memory[2] = 0x82;
    cpu.memory[3] = 0x54;

    cpu.run();
}
