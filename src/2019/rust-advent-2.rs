use std::fs;
use std::env;


fn main(){
    let file_name: String = env::args()
        .collect::<Vec<String>>()
        .get(1)
        .unwrap_or(&String::from("input/2019/2.txt"))
        .to_string(); 

    let file_content: String = fs::read_to_string(file_name)
        .expect("could not open file"); 

    let mut program: Vec<usize> = file_content
        .replace("\n", "")
        .split(',')
        .map(|string| string.parse().expect("expected number"))
        .collect();

    program[1] = 12; 
    program[2] = 2; 
    println!("number at 0: {}", run_program(program.clone())); 
    for first in 0..program.len() {
        for second in 0..program.len() {
            program[1] = first; 
            program[2] = second; 
            if run_program(program.clone()) == 19690720 {
                println!("for input {}, {} ({})", first, second, (100*first + second)); 
                return
            }
        }
    }
}

fn run_program(mut program: Vec<usize>) -> usize {
    let mut pointer: usize = 0; 
    loop {
        match program[pointer] {
            1 => { 
                let source1 = program[pointer + 1]; 
                let source2 = program[pointer + 2]; 
                let dest = program[pointer + 3]; 

                program[dest] = program[source2] + program[source1]; 
                pointer += 4; 
            }
            2 => {
                let source1 = program[pointer + 1]; 
                let source2 = program[pointer + 2]; 
                let dest = program[pointer + 3]; 

                program[dest] = program[source2] * program[source1]; 
                pointer += 4; 
            }
            99 => break,
            _ => panic!("opcode invalid")
        }
    }

    program[0] 
}
