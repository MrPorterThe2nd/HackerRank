use std::mem;

fn main() {

    let input = String::from("saveChangesInTheEditor");

    let v: Vec<&str> = input.split(char::is_uppercase).collect();

    println!("{}", v.len());

}
