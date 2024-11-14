use std::io::*;

fn main() {
    let (w, h) = termion::terminal_size().unwrap();
    println!("welcome to sine wave generator");
    println!("y = a * sin(bx)");
    let mut a = String::new();
    let mut b = String::new();
    println!("enter value for a: ");
    match stdin().read_line(&mut a) {
        Ok(_) => print!(""),
        Err(err) => println!("there was an error: {}", err)
    }
    println!("enter value for b: ");
    match stdin().read_line(&mut b) {
        Ok(_) => print!(""),
        Err(err) => println!("there was an error: {}", err)
    }
    let a = a.trim().parse::<i32>().unwrap();
    let b = b.trim().parse::<i32>().unwrap();
    for y in -a..a+1 {
        print!("\x1b[38;2;255;122;200;48;2;42;37;46m{:4}│", -y);
        for x in 0..(w/2-5) {
            if ((y.abs() % 2 == 0 && x % 2 == 0) || (y.abs() % 2 == 1 && x % 2 == 1)) {
                print!("\x1b[48;2;54;48;59m");
            }
            print!("  \x1b[0m\x1b[48;2;42;37;46m");
        }
        println!();
    }
}
