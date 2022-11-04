# A CHIP-8 emulator in pure Rust



# What is CHIP-8?



- According to wikipedia, CHIP-8 is an interpreted programming language, developed by Joseph Weisbecker made on his 
 1802 Microprocessor. It was initially used on the COSMAC VIP and Telmac 1800 8-bit microcomputers in the mid-1970s. 
 CHIP-8 programs are run on a CHIP-8 virtual machine. [Learn More](https://en.wikipedia.org/wiki/CHIP-8)
 
# How does it work?


- Instructions are represented as hex value while can be decoded into individual nibble which contains specific instructions.
  For example, the hex value 0x8104 instructions the CPU to add the number in register 0 to register 1; The 8 represents that there are
  two registers we are working with, 1 represents `register 1`, 0 represents `register 0` and 4 represent `addition`. This is, however,
  huge simplification (take the above description with a grain of salt).

# Usage

```rust

    // Bring the CPU struct in scope
    use executor::CPU;

    //The entry point to our emulator
    fn main(){
        let mut cpu = CPU{
            registers: [0;16],
            pos_in_memory: 0,
            memory: [0; 4096],
            stack: [0; 16],
            stack_pointer: 0
        }
        // Note: the values of both the registers must be in the bounds of u8 as originally implemented in CHIP-8
        // Put values in registers 0 and 1 (used for addition in this example)
        cpu.registers[0] = 5;
        cpu.registers[1] = 20;
        // test for multiplication
        cpu.registers[2] = 2;
        cpu.registers[3] = 10;
        
        //for convenience: create a mutable reference to our memory
        let mem = &mut cpu.memory;

        // Set some op codes to test our program
        // Each 2 byte is 1 op code
        
        // For example mem[0x000] and mem[0x001] are one opcode
        mem[0x000] = 0x21;
        mem[0x001] = 0x00;
        mem[002] = 0x21;
        mem[0x003] = 0x00;
        mem[0x004] = 0xf2;
        mem[0x005] = 0x3f;
        // mem[0x004] = 0x00; // Uninitialized memory address are already zero
        // mem[0x005] = 0x00; // No need to do this

        mem[0x100] = 0x80;
        mem[0x101] = 0x14;
        mem[0x102] = 0x80;
        mem[0x103] = 0x14;
        mem[0x104] = 0x00;
        mem[0x105] = 0xEE;

        cpu.run();
    }
    
```

# Why?


- ~I'm bored lmao~ It's interesting


# Features

- Addition
- Multiply (custom op code -> 0xFxyF where: x: Register x, y: Register: y)
- Function call and return

# Op Codes:
    - `0x8xy4` -> Add value of `register y` to `register x` and store in register x
    - `0x2nnn` -> Go to memory address `nnn`
    - `0x00EE` -> Return to function call memory address
    - `0xfxyf` -> Multiply value of `register y` to `register x` and store the result in `register x`
    - `0x6xnn` -> Put value nn in `register x`
    
    Work on supporting other operation is going on.
 
