# rusty_brain_fcuk
An interpreter for the Brainfu*k programming language written in Rust.
## What is the brainfu*k language?
It is a very simple, in terms of implementation, turing-complete programming language built in 1992 by X. Don't let its simplicity in design fool you into thinking that coding in this language may be simple too. As the name suggests, writing logic in this language is literally going to fuck your brains out. Owing to its highly restrictive set of operations allowed, it makes it excruciatinbly painful to write the simplest of algorithms like insertion sort. 

The operations and constructs supported in this language are: 
- "+": for adding the value pointed to by 1
- "-": for subtracting the value pointed to by 1
- ">": to move one cell to the right 
- "<": to move one cell to the left
- "[ ]": this is the construct for looping, all operations enclosed by the "[ ]" are repeated until terminating condition
- ".": print out the value pointed to by the cell pointer
- ",": set the value inputted at the cell pointer location

### Simple example: 
Code for printing out the string "Hello World!" using brainfuck: `++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.`

### Repository structure
- scr/lib.rs: includes all the logic related to the interpreter
- src/main.rs: main code to run
### How to run
Run the command: `cargo run "path_to_brainfuck_input_file"`      
Example: `cargo run examples/hello_world.bf`       
Output : `Hello World!`   
