use std::io;

fn main() {
    println!("Insira o numero!");
    let mut num = String::new();

    io::stdin()
    .read_line(&mut num)
    .expect("Failed to read line");

    let num: u32 = num.trim().parse().expect("Not a number!");


    if num % 2 != 0 && num % 3 == 0 {
        println!("É impar e multiplo de 3.")
    }else if num % 3 == 0 {
        println!("É multiplo de 3");
    }else if num % 2 != 0 {
        println!("É impar")
    }
}
