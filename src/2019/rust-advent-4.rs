type Number = [u8; 6]; 

struct NumberIterator{
    current: Number,
    max: Number,
}

impl Iterator for NumberIterator{
    type Item = Number;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.max {None}
        else {
            let mut index = 5;
            loop {
                self.current[index] += 1;
                if self.current[index] < 10 {break;}
                self.current[index] = 0;
                index -= 1; 
            }; 
            Some(self.current)
        }    
    }
}

fn number() -> NumberIterator{
    NumberIterator{
        current: [1,3,8,3,0,7], 
        max: [6,5,4,5,0,4]
    } 
}

fn main(){
    let count_1 = number().filter(|number| possible_password_1(number))
        .count();
    let count_2 = number().filter(|number| possible_password_2(number))
        .count();
    println!("found passwords (first condition): {}", count_1); 
    println!("found passwords (first condition): {}", count_2); 
}

fn possible_password_1(number: &Number) -> bool {
    // numbers increasing
    let increasing = number[0] <= number[1] &&
        number[1] <= number[2] &&
        number[2] <= number[3] &&
        number[3] <= number[4] &&
        number[4] <= number[5]; 

    // at least one double 
    let double = number[0] == number[1] ||
        number[1] == number[2] ||
        number[2] == number[3] ||
        number[3] == number[4] ||
        number[4] == number[5];

    return double && increasing; 
}

fn possible_password_2(number: &Number) -> bool {
    // numbers increasing
    let increasing = number[0] <= number[1] &&
        number[1] <= number[2] &&
        number[2] <= number[3] &&
        number[3] <= number[4] &&
        number[4] <= number[5]; 

    // one double 
    let double = 
        (number[0] == number[1] && number[0] != number[2]) ||
        (number[1] == number[2] && number[1] != number[3] && number[1] != number[0]) ||
        (number[2] == number[3] && number[2] != number[4] && number[2] != number[1]) ||
        (number[3] == number[4] && number[3] != number[5] && number[3] != number[2]) ||
        (number[4] == number[5] && number[4] != number[3]); 

    return double && increasing; 
}
