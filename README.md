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

# Usuage

```rust

    // Bring the CPU struct in scope
    use add::CPU;

    //The entry point to our emulator
    fn main(){
        let op = 0x8104;
    
    // Note: the values of both the registers must be in the bounds of u8 as originally implemented in CHIP-8
    
        let register1 = 10; // Initial value of the first register
        let register2 = 15; // Initial value of the second register
    
        // Initialize a cpu with the above Instructions and values
        let cpu = CPU::new(op, register1, register2);
    
        // Evaluate the instructions and add register2 to register1
    
        cpu.run();
    }
    
```

# Why?


- I'm bored lmao

# Features


- It only supports addition for now but work on multiplication is going on.