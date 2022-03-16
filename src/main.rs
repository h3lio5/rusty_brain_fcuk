use rusty_brain_fcuk::*;
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    let mut interpreter_instance = Interpreter::new(&args[1]);
    // Run the interpreter on the brainfuck code
    interpreter_instance.execute();

    // Print out the output of the interpreter
    print!("Output: {}", interpreter_instance.get_output());
}
