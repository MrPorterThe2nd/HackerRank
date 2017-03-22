use std::io::{self, BufRead};

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

    let mut pos: f32= 0;    //Initilize score for Alice
    let mut neg: f32 = 0;    //Initilize score for Bob
    let mut zero: f32 = 0;    //Initilize score for Bob
    let length: f32 = numbers1.len();
     //compare all numbers in the array
    for b in 0..numbers1.len() {
        if numbers1[b] == 0 {
            zero = zero + 1;
        } else if numbers1[b] > 0 {
            pos = pos+1;
        } else if numbers1[b] < 0 {
            neg = neg+1;
        }
    }


    println!("{}", pos/length);
    println!("{}", neg/length);
    println!("{}", zero/length);
}
