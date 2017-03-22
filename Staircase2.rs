fn main() {

    let staircase = 6;

    for i in 1..staircase+1 {
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
