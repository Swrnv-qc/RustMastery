use std::io;

fn main(){
    let mut s = String::new();
    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line!");
    let rev : String = s.chars().rev().collect();
    println!("{}",rev)
}