use std::io::{self, BufRead};
use std::ptr;
use std::mem;

fn main() {
    //Read in from input
    let reader = io::stdin();

    //Read in one line and convert to a vector array
    let Alice: Vec<i32> =
        reader.lock()                           // (0)
              .lines().next().unwrap().unwrap() // (1)
              .split(' ').map(|s| s.trim())     // (2)
              .filter(|s| !s.is_empty())        // (3)
              .map(|s| s.parse().unwrap())      // (4)
              .collect();                       // (5)

    //Read in one line and convert to a vector array
    let Bob: Vec<i32> =
        reader.lock()                           // (0)
              .lines().next().unwrap().unwrap() // (1)
              .split(' ').map(|s| s.trim())     // (2)
              .filter(|s| !s.is_empty())        // (3)
              .map(|s| s.parse().unwrap())      // (4)
              .collect();

    let mut a_score = 0;    //Initilize score for Alice
    let mut b_score = 0;    //Initilize score for Bob

    //compare all numbers in the array
    for x in 0..Alice.len() {
        if Alice[x] > Bob[x] {
            a_score = a_score+1;
        } else if Alice[x] < Bob[x] {
            b_score = b_score+1;
        }
    }


    println!("{} {}", a_score, b_score);
}
