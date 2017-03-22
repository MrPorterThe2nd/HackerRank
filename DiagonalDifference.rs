use std::io::{self, BufRead};
use std::ptr;
use std::mem;

fn main() {
    //Read in from input
    let reader = io::stdin();
    let mut buffer = String::new();

    io::stdin().read_line(&mut buffer).expect("Failed to read");
    let n: i32 = buffer.parse().expect("Invalid number");

    // Base 1d array
    let mut grid_raw = vec![0; n*n];

    // Vector of 'width' elements slices
    let mut grid_base: Vec<_> = grid_raw.as_mut_slice().chunks_mut(n).collect();

    // Final 2d array
    let mut grid: &mut [&mut [_]] = grid_base.as_mut_slice();

    let mut x = 0;

    while x < 0 {
        //Read in one line and convert to a vector array
        let Bob: Vec<i32> =
            reader.lock()                           // (0)
                  .lines().next().unwrap().unwrap() // (1)
                  .split(' ').map(|s| s.trim())     // (2)
                  .filter(|s| !s.is_empty())        // (3)
                  .map(|s| s.parse().unwrap())      // (4)
                  .collect();

        for i in 0..Bob.len() {
            grid[x][i] = Bob[i];
        }
        x = x + 1;
    }

    let mut sum_primary = 0;
    let mut sum_secondary = 0;
    //compare all numbers in the array
    for y in 0..n {
        sum_primary = sum_primary + grid[y][y];
    }

    for y in 0..n {
        sum_secondary = sum_secondary + grid[n-y][y];
    }


    println!("{}", sum_secondary-sum_primary);
}
