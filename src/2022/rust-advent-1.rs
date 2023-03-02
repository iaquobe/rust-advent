use std::fs;
use std::env;

fn main() {
    let args: Vec<String> = env::args()
        .collect(); 
    let file_name: String = args.get(1)
        .unwrap_or(&String::from("input/2022/1.txt"))
        .to_string(); 

    let content: String = fs::read_to_string(file_name)
        .expect("expected file"); 

    let most1 = with_stream(&content); 
    let most2 = with_for(&content); 

    println!("with stream {}", most1); 
    println!("with for {}", most2); 

}

fn with_stream(content: &String) -> i32 {
    let mut elves: Vec<i32> = content.lines()
        .map(|line| line.parse())
        .collect::<Vec<Result<i32,_>>>()
        .split(|res| match res{ 
            Ok(_) => false,
            Err(_) => true})
        .map(|slice| slice.iter().fold(0, |sum, val| sum + val.as_ref().unwrap()))
        .collect::<Vec<i32>>() ;

    elves.sort_by(|a,b| b.cmp(a)); 
    elves[0]
}


fn with_for(content: &String) -> i32 {
    let mut elves: Vec<i32> = Vec::new(); 
    let mut sum = 0; 

    for line in content.lines() {
        match line.parse::<i32>() {
            Ok(num) => sum += num, 
            Err(_) => {elves.push(sum); sum = 0;}
        }
    }

    elves.sort_by(|a,b| b.cmp(a)); 
    elves[0]
}
