pub struct CPU {
    pub registers: [u8; 16],
    pub pos_in_memory: usize,
    pub memory: [u8; 0x1000],
}

impl CPU {
    pub fn decode_op_code(&self) -> u16 {
        let op1 = self.memory[self.pos_in_memory] as u16;
        let op2 = self.memory[self.pos_in_memory + 1] as u16;
        op1 << 8 | op2
    }

    pub fn add(&mut self, x: u8, y: u8) {
        let register1 = self.registers[x as usize];
        let register2 = self.registers[y as usize];
        let (value, overflow) = register1.overflowing_add(register2);
        self.registers[x as usize] = value;
        if overflow {
            self.registers[0xf] = 1;
        } else {
            self.registers[0xf] = 0;
        }
    }

    pub fn run(&mut self) {
        loop {
            {
                let op = self.decode_op_code();
                self.pos_in_memory += 2;
                let p = ((op & 0xf000) >> 12) as u8;
                let x = ((op & 0x0f00) >> 8) as u8;
                let y = ((op & 0x00f0) >> 4) as u8;
                let z = ((op & 0x000f) >> 0) as u8;

                // test

                match (p, x, y, z) {
                    (0, 0, 0, 0) => {
                        return;
                    }
                    (0x8, _, _, 0x4) => {
                        self.add(x, y);
                        println!("{}", self.registers[x as usize] as u8)
                    }
                    _ => {
                        println!("Unsupported operation")
                    }
                }
            }
        }
    }
}
