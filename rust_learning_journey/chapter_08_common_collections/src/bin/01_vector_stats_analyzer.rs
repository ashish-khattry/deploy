//01_vector_stats_analyzer
use std::collections::HashMap;
use std::io;
fn main() {
    let mut numbers: Vec<i32> = Vec::new();
    println!("Enter numbers like 10 20 30 40 50 60....");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Input error!");
    for word in input.split_whitespace() {
        match word.parse::<i32>() {
            Ok(number) => numbers.push(number),
            Err(_) => {
                println!("Invalid input!");
            }
        }
    }
    let length = numbers.len();
    if length == 0 {
        println!("No numbers are entered");
        return;
    }
    let length = length as i32;
    let mut sum = 0;
    for i in &numbers {
        sum += i;
    }
    let mean = sum / length;
    println!("Mean is ={mean}");
    numbers.sort();
    let medium_index = numbers.len() / 2;
    println!("Medium is ={}", numbers[medium_index]);
    let mut hashmap = HashMap::new();
    for value in &numbers {
        let count = hashmap.entry(value).or_insert(0);
        *count += 1;
    }
    let mut big = 0;
    let mut value = 0;
    for (number, count) in hashmap {
        if big < count {
            big = count;
            value = *number;
        }
    }
    println!("Mode is ={}", value);
}
