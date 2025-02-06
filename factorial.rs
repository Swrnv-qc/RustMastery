use std::io;

fn main(){
    let mut n = String :: new();
    
    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line!");
    
    let n : u32 = n.trim().parse().expect("Please type in a number!");

    let mut fact = 1;
    for i in 1..n+1{
        fact = fact * i;
    }
    println!("Factorial of {0}: {1}",n,fact);
}