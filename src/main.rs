
struct CPU{
    current_op: u16,
    registers: [u8; 2]
}

impl CPU{
    fn new(current_op: u16, register1: u8, register2: u8) -> Self{
        Self{
            current_op,
            registers: [register1, register2]
        }
    }
    
    fn decode_op_code(&self) -> u16{
        self.current_op
    }
    
    fn add(&mut self, x: u8, y: u8) {
        self.registers[x as usize] += self.registers[y as usize];
    }
    
    fn run(&mut self) {
        let op = self.decode_op_code();
        let p = ((op & 0xf000) >> 12) as u8;
        let x = ((op & 0x0f00) >> 8) as u8;
        let y = ((op & 0x00f0) >> 4) as u8;
        let z = ((op & 0x000f) >> 0) as u8;
        
        
        
        // test
        
        
        assert_eq!(op, 0x8014);
        assert_eq!(p, 0x8);
        assert_eq!(x, 0x0);
        assert_eq!(y, 0x1);
        assert_eq!(z, 0x4);
        match (p, x, y, z) {
            (0x8, _, _, 0x4) => {
                self.add(x, y);
                println!("{}", self.registers[x as usize] as u8)
        },
            _ => {println!("Unsupported operation")}
    }
    }
    
}

fn main() {
    let mut cpu = CPU::new(0x8014, 10, 15);
    cpu.run();
}



