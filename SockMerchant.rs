fn main() {

    let mut  input = String::new();
    let T: i32 = std::io::stdin().read_line(&mut input).trim().parse().unwrap();

    std::io::stdin().read_line(&mut input).expect("Could not read stdin!");

    let v: Vec<&str> = input.split(' ').collect();

    println!("{}", v.len());

}
