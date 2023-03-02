use std::fs;
use std::env;
use std::io;

fn main(){
    let file_name: String = env::args()
        .collect::<Vec<String>>()
        .get(1)
        .unwrap_or(&String::from("input/2019/5.txt"))
        .to_string(); 

    let file_content: String = fs::read_to_string(file_name)
        .expect("could not open file"); 

    let mut program: Vec<isize> = file_content
        .replace("\n", "")
        .split(',')
        .map(|string| string.parse().expect("expected number"))
        .collect();

    println!("number at 0: {}", run_program(program.clone())); 
}

fn run_program(mut program: Vec<isize>) -> isize {
    let mut pointer: usize = 0; 
    loop {
        match program[pointer] % 100 {
            1 => { 
                let source1 = program[pointer + 1]; 
                let source2 = program[pointer + 2]; 
                let dest = program[pointer + 3]; 

                program[dest as usize] = program[source2 as usize] + program[source1 as usize]; 
                pointer += 4; 
            }
            2 => {
                let source1 = program[pointer + 1]; 
                let source2 = program[pointer + 2]; 
                let dest = program[pointer + 3]; 

                program[dest as usize] = program[source2 as usize] + program[source1 as usize]; 
                pointer += 4; 
            }
            3 => {

                let mut buffer: String = String::new();
                let stdin = io::stdin(); 
                stdin.read_line(&mut buffer).unwrap(); 

                let num: isize = buffer.parse().expect("expected a number"); 
                let dest = program[pointer + 1]; 
                program[dest as usize] = num; 
            },
            4 => {
                let source = program[pointer + 1]; 
                println!("{}", source); 
            },
            99 => break,
            _ => panic!("opcode invalid")
        }
    }

    program[0] 
}
