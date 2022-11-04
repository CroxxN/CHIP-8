#![allow(non_snake_case)]

mod executor;

use executor::CPU;

fn main() {
    let mut cpu = CPU {
        registers: [0; 16],
        pos_in_memory: 0,
        memory: [0; 4096],
        stack: [0; 16],
        stack_pointer: 0,
    };

    // Sets values in the register
    // cpu.registers[0] = 5;
    // cpu.registers[1] = 10;
    // cpu.registers[2] = 23;
    // cpu.registers[3] = 10;
    // cpu.registers[5] = 2;

    // // sets instructions in the memory
    // // as each memory is u8 type, it can hold 8-bit value
    // // so we split the instructions across two memory address

    // // reads "add the value of register 1 to that of register 0"
    // // and save it to register 0
    // cpu.memory[0] = 0x80;
    // cpu.memory[1] = 0x14;

    // //reads "add the value of register 5 to that of register 2"
    // // and save it in register 2
    // cpu.memory[2] = 0x82;
    // cpu.memory[3] = 0x54;
    cpu.registers[0] = 5;
    cpu.registers[1] = 20;
    // test for multiplication
    cpu.registers[2] = 2;
    cpu.registers[3] = 10;

    let mem = &mut cpu.memory;

    mem[0x000] = 0x21;
    mem[0x001] = 0x00;
    mem[002] = 0x21;
    mem[0x003] = 0x00;
    mem[0x004] = 0xf2;
    mem[0x005] = 0x3f;
    mem[0x006] = 0x64;
    mem[0x007] = 0x0f;
    // mem[0x004] = 0x00;
    // mem[0x005] = 0x00;

    mem[0x100] = 0x80;
    mem[0x101] = 0x14;
    mem[0x102] = 0x80;
    mem[0x103] = 0x14;
    mem[0x104] = 0x00;
    mem[0x105] = 0xEE;

    cpu.run();
}
