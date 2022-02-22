use std::io;
use std::cmp::Ordering;

fn main() {
    let mut counter = 2;
    let mut num1 = String::new();
    let mut num2 = String::new();
    let mut max = 0;
    let mut div = 0;

    io::stdin()
        .read_line(&mut num1)
        .expect("Failed to read line");
        io::stdin()
        .read_line(&mut num2)
        .expect("Failed to read line");

    let num1: i32 = num1.trim().parse().expect("Not a number!");
    let num2: i32 = num2.trim().parse().expect("Not a number!");


    loop{
        match num1.cmp(&num2) {
            Ordering::Less => max = num1,
            Ordering::Greater => max = num2,
            Ordering::Equal => break,
                }

        if num1 % counter == 0 && num2 % counter == 0{
            div = counter;
        }
            

        if counter == max{
            println!("O maximo div commum é {}", div);
            break;
        }
        counter +=1;
    }
}