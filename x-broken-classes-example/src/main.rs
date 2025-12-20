struct Memory {
    value: i32,
}
impl Memory {
    fn new() -> Self {
        return Self { value: 0 };
    }

    fn set_value(mut self, value: i32) {
        self.value = value;
    }

    fn get_value(self) -> i32 {
        return self.value;
    }
}

struct Cpu {
    memory: &Memory,
}

impl Cpu {
    fn new(memory: &Memory) -> Self {
        return Self { memory: memory };
    }

    fn run(&self) {
        self.memory.set_value(5);
    }
}

struct Gpu {
    memory: &Memory,
}
impl Gpu {
    fn new(memory: &Memory) -> Self {
        return Self { memory: memory };
    }

    fn run(&self) {
        let value = self.memory.get_value();
        println!("value is: {value}");
    }
}

fn main() {
    let memory = Memory::new();
    let cpu = Cpu::new(&memory);
    let gpu = Gpu::new(&memory);

    gpu.run();
    cpu.run();
    gpu.run();
}
