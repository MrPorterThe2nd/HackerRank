use std::io

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

    let staircase = numbers[0] as i32;

    for i in 0..staircase {
        for a in 0..staircase{
            if a < staircase-i{
                print!(" ");
            } else {
                print!("#");
            }
        }
        println!("");
    }
}
