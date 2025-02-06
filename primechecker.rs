use std::io;

fn main() {
    let mut num = String::new();
    println!("Please enter a number:");

    io::stdin()
        .read_line(&mut num)
        .expect("Failed to read line!");

    let num_u32: u32 = num.trim().parse().expect("Please enter a valid number!");

    if num_u32 < 2 {
        println!("{} is not prime.", num_u32);
        return;
    }

    let num_sqrt = (num_u32 as f32).sqrt().ceil() as u32;

    let mut is_prime = true;

    for i in 2..=num_sqrt {
        if num_u32 % i == 0 {
            println!("{} is not prime.", num_u32);
            is_prime = false;
            break;
        }
    }

    if is_prime {
        println!("{} is prime.", num_u32);
    }
}
