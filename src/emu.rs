struct CPU {
    current_operation: u16,
    register: [u8; 2]
}

impl CPU {
    fn read_opcode(&self) -> u16 {
        self.current_operation
    }

    fn run(&mut self) {
        let opcode = self.read_opcode();
        let c = (opcode & 0xF000) >> 12;
        let x = (opcode & 0x0F00) >> 8;
        let y = (opcode & 0x00F0) >> 4;
        let d = opcode & 0x000F;
        println!("c is {}", c);
        println!("d is {}", d);
        match (c, x, y, d) {
            (0x8, _, _, 0x4) => self.add_xy(x as u8, y as u8),
            _ => todo!("opcode {:04x}", opcode)
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        self.register[x as usize] += self.register[y as usize];
    }
}

pub fn begin_emu() {
    let mut cpu = CPU {
        current_operation: 0,
        register: [0; 2]
    };    
    cpu.current_operation = 0x8014;
    cpu.register[0] = 5;
    cpu.register[1] = 10;

    cpu.run();

    assert_eq!(cpu.register[0], 15);
    println!("the result of 5 + 10 is {}", cpu.register[0]);

    let mut accumulator = Accumulator {
        register : [0; 16],
        memory: [0; 4096],
        position_in_memory: 0
    };

    accumulator.register[0] = 5;
    accumulator.register[1] = 10;
    accumulator.register[2] = 10;
    accumulator.register[3] = 10;

    let mem = &mut accumulator.memory;
    mem[0] = 0x80;
    mem[1] = 0x14;
    mem[2] = 0x80;
    mem[3] = 0x24;
    mem[4] = 0x80;
    mem[5] = 0x34;

    accumulator.run();

    assert_eq!(accumulator.register[0], 35);
    println!("5 + 10 + 10 = 10 = {}", accumulator.register[0]);
}

struct Accumulator { 
    register: [u8; 16],
    position_in_memory: usize,
    memory: [u8; 0x1000],
}

impl Accumulator {
    fn read_opcode(&mut self) -> u16 {
        let opcode1 = self.memory[self.position_in_memory] as u16;
        let opcode2 = self.memory[self.position_in_memory + 1] as u16;
        opcode1 << 8 | opcode2
    }

    fn run(&mut self) {
        loop {
            let opcode = self.read_opcode();
            self.position_in_memory += 2;
            let c = (opcode & 0xF000) >> 12;
            let x = (opcode & 0x0F00) >> 8;
            let y = (opcode & 0x00F0) >> 4;
            let d = opcode & 0x000F;

            match (c, x, y, d) {
                (0, 0, 0, 0) => {return;}
                (0x8, _, _, 0x4) => self.add_xy(x as u8, y as u8),
                _ => todo!("opcode {:04x}", opcode),
            }
        }
    }

    fn add_xy(&mut self, x: u8, y: u8) {
        let (result, overflow) = self.register[x as usize].overflowing_add(self.register[y as usize]);
        self.register[x as usize] = result;
        if overflow {
            self.register[0xF] = 1;
        } else {
            self.register[0xF] = 0;
        }
    }
}

struct StackCPU {
    register: [u8; 16],
    memory_in_position: usize,
    memory: [u8; 4096],
    stack: [u16; 16],
    stack_pointer: usize
}

impl StackCPU {
    
}