use std::io;

fn main() {
    let mut value = 0;
    loop {
        let mut ks = String::new();

        io::stdin()
        .read_line(&mut ks)
        .expect("Failed to read line");

        let ks: i32 = ks.trim().parse().expect("Not a number!");
        
        if ks == 0 {
            println!("O maior valor é {}", value);
            break;
        }else if ks > value {
            value = ks;
        }
    }
}
