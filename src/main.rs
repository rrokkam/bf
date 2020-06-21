struct Program {
    data: Vec<u8>,
    data_pointer: usize,
    instructions: Vec<u8>,
    instruction_pointer: usize,
}

impl Program {
    pub fn new(program: Vec<u8>) -> Self {
        Program {
            data: vec![0; 30_000],
            data_pointer: 0,
            instructions: program,
            instruction_pointer: 0,
        }
    }

    pub fn run(mut self) {
        while !self.done() {
            self.step()
        }
    }

    fn step(&mut self) {
        match self.current_instruction() {
            b'>' => self.data_pointer += 1,
            b'<' => self.data_pointer -= 1,
            b'+' => self.data[self.data_pointer] += 1,
            b'-' => self.data[self.data_pointer] -= 1,
            b'[' => {
                if self.data[self.data_pointer] == 0 {
                    self.jump_instruction_pointer_to_matching_right_brace()
                }
            }
            b']' => {
                if self.data[self.data_pointer] != 0 {
                    self.jump_instruction_pointer_to_matching_left_brace()
                }
            }
            b'.' => print!("{}", self.data[self.data_pointer] as char),
            b',' => unimplemented!(), // self.store_byte_at_data_pointer(),
            _ => (),
        };
        self.instruction_pointer += 1;
    }

    fn current_instruction(&self) -> u8 {
        self.instructions[self.instruction_pointer]
    }

    fn jump_instruction_pointer_to_matching_right_brace(&mut self) {
        let mut rbraces_to_match = 0;
        loop {
            match self.current_instruction() {
                b'[' => rbraces_to_match += 1,
                b']' => rbraces_to_match -= 1,
                _ => (),
            }
            if rbraces_to_match == 0 {
                return;
            }
            self.instruction_pointer += 1;
        }
    }

    fn jump_instruction_pointer_to_matching_left_brace(&mut self) {
        let mut lbraces_to_match = 0;
        loop {
            match self.current_instruction() {
                b'[' => lbraces_to_match -= 1,
                b']' => lbraces_to_match += 1,
                _ => (),
            }
            if lbraces_to_match == 0 {
                return;
            }
            self.instruction_pointer -= 1;
        }
    }

    pub fn done(&self) -> bool {
        self.instruction_pointer >= self.instructions.len()
    }
}

fn main() {
    let filename = std::env::args()
        .nth(1)
        .expect("args should have a filename as the first argument");
    let program = std::fs::read(filename).expect("first argument in args should point to a file");

    Program::new(program).run();
}
