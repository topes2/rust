use std::io;
use std::cmp::Ordering;

fn main() {
    let mut counter1 = 1;
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut counter2 = 1;

    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");

    io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");

    let mut num1: i64 = num1.trim().parse().expect("Not a number!");
    let num1o = num1;
    let mut num2: i64 = num2.trim().parse().expect("Not a number!");
    let num2o = num2;

    loop{
        match num1.cmp(&num2) {
            Ordering::Less => {counter1 +=1;
            num1 =num1o * counter1},
            Ordering::Greater => {counter2 +=1;
                num2 =num2o * counter2},
            Ordering::Equal => {println!("O minimo multiplo common é {}",num2);
            break},
                }
}
}