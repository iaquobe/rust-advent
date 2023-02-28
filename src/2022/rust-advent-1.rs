use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args()
        .collect(); 
    let file_name: String = args.get(1)
        .unwrap_or(&String::from("input/2022/1.txt"))
        .to_string(); 

    let mut elves: Vec<i32> = fs::read_to_string(file_name)
        .expect("expected file") 
        .lines()
        .map(|line| line.parse())
        .collect::<Vec<Result<i32,_>>>()
        .split(|res| match res{ 
            Ok(_) => false,
            Err(_) => true})
        .map(|slice| slice.iter().fold(0, |sum, val| sum + val.as_ref().unwrap()))
        .collect::<Vec<i32>>() ;

    elves.sort();
    println!("{:?}", elves);
}
