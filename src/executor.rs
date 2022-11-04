pub struct CPU {
    pub registers: [u8; 16],
    pub pos_in_memory: usize,
    pub memory: [u8; 0x1000],
    pub stack: [u16; 16],
    pub stack_pointer: usize,
}

impl CPU {
    fn decode_op_code(&self) -> u16 {
        let op1 = self.memory[self.pos_in_memory] as u16;
        let op2 = self.memory[self.pos_in_memory + 1] as u16;
        op1 << 8 | op2
    }

    fn add(&mut self, x: u8, y: u8) {
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
    fn mutiply(&mut self, x: u8, y: u8) {
        let register1 = self.registers[x as usize];
        let register2 = self.registers[y as usize];

        let (value, overflow) = register1.overflowing_mul(register2);
        self.registers[x as usize] = value;
        if overflow {
            self.registers[0xf] = 1;
        }
    }
    pub fn call(&mut self, addr: u16) {
        let stack = &mut self.stack;
        let stack_pointer = self.stack_pointer;

        if stack_pointer > stack.len() {
            panic!("Stack Overflow")
        }
        stack[stack_pointer] = self.pos_in_memory as u16;
        self.stack_pointer += 1;
        self.pos_in_memory = addr as usize;
    }

    fn return_func(&mut self) {
        if self.stack_pointer == 0 {
            panic!("Stack Overflow");
        }

        self.stack_pointer -= 1;
        self.pos_in_memory = self.stack[self.stack_pointer] as usize;
    }

    fn set_value(&mut self, x: u8, y: u8, z: u8) {
        let (y, z) = (y as u16, z as u16);
        let value = ((y << 8) | z) as u8;
        self.registers[x as usize] = value;
        println!("Value {} set at register {:#1x}", value, x);
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

                let nnn = op & 0x0fff;
                // test

                match (p, x, y, z) {
                    (0, 0, 0, 0) => {
                        return;
                    }
                    (0x8, _, _, 0x4) => {
                        self.add(x, y);
                        println!("{}", self.registers[x as usize] as u8)
                    }
                    (0xf, _, _, 0xf) => {
                        self.mutiply(x, y);
                        println!("Result: {}", self.registers[x as usize] as u8)
                    }
                    (0x2, _, _, _) => {
                        self.call(nnn);
                    }
                    (0x0, 0x0, 0xE, 0xE) => {
                        self.return_func();
                    }
                    (0x6, _, _, _) => self.set_value(x, y, z),
                    _ => {
                        println!("Unsupported operation")
                    }
                }
            }
        }
    }
}
