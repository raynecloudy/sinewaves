use std::io::*;

fn main() {
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
        println!("{}\x1b[10D\x1b[4C│", -y);
    }
}
