// Enter your code here
use std::io::{self, BufRead};                   // (a)

fn main() {
    //Read in from input
    let reader = io::stdin();

    //Read in one line and convert to a vector array
    let numbers: Vec<i32> =
        reader.lock()                           // (0)
              .lines().next().unwrap().unwrap() // (1)
              .split(' ').map(|s| s.trim())     // (2)
              .filter(|s| !s.is_empty())        // (3)
              .map(|s| s.parse().unwrap())      // (4)
              .collect();                       // (5)


    let numbers1: Vec<i32> =
        reader.lock()                           // (0)
              .lines().next().unwrap().unwrap() // (1)
              .split(' ').map(|s| s.trim())     // (2)
              .filter(|s| !s.is_empty())        // (3)
              .map(|s| s.parse().unwrap())      // (4)
              .collect();

    //sum all numbers in an array
    let sum = numbers1.iter().fold(0,|a, &b| a + b);

    println!("{}",sum);
}
