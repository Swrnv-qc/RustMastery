fn main(){
    for n in 1..100{
        if n%15 == 0{
            println!("{} : fizzbuzz",n);
        }
        else if n%5 == 0{
            println!("{} : buzz",n);
        }
        else if n%3 == 0{
            println!("{} : fizz",n);
        }
        else{
            println!("{}",n);
        }
    }
}