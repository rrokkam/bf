struct Program {
    tape: Vec<u8>,
    head: usize,
    program: Vec<u8>,
    pc: usize,
}

impl Program {
    pub fn new(program: Vec<u8>) -> Self {
        Program {
            tape: Vec::with_capacity(30_000),
            head: 0,
            program: program,
            pc: 0,
        }
    }

    pub fn run(mut self) {
        while !self.done() {
            self.step()
        }
    }

    fn step(&mut self) {
        println!("READING: {}", self.program[self.pc] as char);
        self.pc += 1;
    }

    pub fn done(&self) -> bool {
        self.pc == self.program.len()
    }
}

fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("args should have a filename as the first argument");
    let program = std::fs::read(filename).expect("first argument in args should point to a file");

    Program::new(program).run();
}
