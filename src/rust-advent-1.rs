use std::fs;
use std::env;
use std::cmp;

fn main(){
    let file_name: String = env::args()
        .collect::<Vec<String>>()
        .get(1)
        .unwrap_or(&String::from("input/1.txt"))
        .to_string(); 

    let content = fs::read_to_string(file_name)
        .expect("could not open file"); 

    let fuel: i32 = content
        .lines()
        .map(|line| {fuel_requirement_1(line.parse().expect("expected number"))})
        .sum();

    let fuel_fuel: i32= content
        .lines()
        .map(|line| {fuel_requirement_2(line.parse().expect("expected number"))})
        .sum();

    println!("total fuel cost: {}", fuel);
    println!("total fuel cost(with fuel costing fuel: {}", fuel_fuel);
    println!("fuel cost: {}", fuel_requirement_2(100756));
}

fn fuel_requirement_1(weight: i32) -> i32 {
    (weight / 3) - 2
}

fn fuel_requirement_2(weight: i32) -> i32 {
    if weight <= 0 { 0 }
    else { 
        let fuel = cmp::max((weight / 3) - 2, 0); 
        fuel + fuel_requirement_2(fuel) 
    }
}
