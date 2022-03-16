use std::collections::HashMap;
use std::fs;

pub struct Interpreter {
    pub code_block: Vec<char>,
    pub cell_block: Vec<u8>,
    pub value: String,
    pub bracemap: HashMap<usize, usize>,
}

impl Interpreter {
    // Creates a new instance of Interpreter instance and sets the code and memory vectors.
    pub fn new(input_filename: &str) -> Self {
        let code_block = fs::read_to_string(input_filename)
            .expect("Something went wrong reading the file")
            .chars()
            .collect::<Vec<char>>();
        let cell_block = vec![0u8; 65536]; // 2 ^ 16
        let value = String::new();
        let bracemap = Self::build_bracemap(&code_block);

        Interpreter {
            code_block,
            cell_block,
            value,
            bracemap,
        }
    }

    fn build_bracemap(code: &Vec<char>) -> HashMap<usize, usize> {
        let mut bracemap: HashMap<usize, usize> = HashMap::new();
        let mut brace_stack: Vec<usize> = Vec::new();

        for (pos, command) in code.iter().enumerate() {
            // let pos = u16::try_from(pos).unwrap();

            if command == &'[' {
                brace_stack.push(pos);
            }
            if command == &']' {
                let start = brace_stack.pop().unwrap();
                bracemap.insert(start, pos);
                bracemap.insert(pos, start);
            }
        }
        bracemap
    }

    // Includes the core logic for the brainfuck interpreter
    pub fn execute(&mut self) {
        let mut code_ptr = 0;
        let mut cell_ptr = 0;

        while code_ptr < self.code_block.len() {
            let command = self.code_block[code_ptr];

            if command == '>' {
                cell_ptr += 1;
            }
            if command == '<' {
                cell_ptr = if cell_ptr <= 0 { 0 } else { cell_ptr - 1 };
            }
            if command == '+' {
                self.cell_block[cell_ptr] = self.cell_block[cell_ptr] + 1
            }
            if command == '-' {
                self.cell_block[cell_ptr] = self.cell_block[cell_ptr] - 1
            }

            if command == '[' && self.cell_block[cell_ptr] == 0 {
                code_ptr = self.bracemap[&code_ptr];
            }
            if command == ']' && self.cell_block[cell_ptr] != 0 {
                code_ptr = self.bracemap[&code_ptr];
            }

            if command == '.' {
                self.value
                    .push(self.cell_block[cell_ptr].try_into().unwrap());
                // print!("{}", self.cell_block[cell_ptr])
            }

            if command == ',' {}

            code_ptr += 1;
        }
    }

    pub fn get_output(&self) -> &str {
        return &self.value;
    }
}
