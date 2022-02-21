use std::io;
use std::f64;

fn main() {
    let mut c1 = String::new();
    let mut c2 = String::new();

    println!("Insira o cateto 1");

    io::stdin()
    .read_line(&mut c1)
    .expect("Failed to read line");

    println!("Insira o cateto 2");
    io::stdin()
    .read_line(&mut c2)
    .expect("Failed to read line");

    let c1: f64 = c1.trim().parse().expect("Not a number!");
    let c2: f64 = c2.trim().parse().expect("Not a number!");

    let hp = c1*c1 + c2*c2;

    println!("A hipotenusa é {}", f64::sqrt(hp));
}
