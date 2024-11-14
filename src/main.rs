use std::io::*;

fn main() {
    let (w, _) = termion::terminal_size().unwrap();
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
    let a = a.trim().parse::<f32>().unwrap();
    let b = b.trim().parse::<f32>().unwrap();
    for y in -a.floor() as i32..a.ceil() as i32+1 {
        print!("\x1b[38;2;255;122;200;48;2;42;37;46m{:4}│", -y);
        for x in 0..w-5 {
            print!("\x1b[0;38;2;255;205;125;48;2;42;37;46m");
            if (y.abs() % 2 == 0 && x/2 % 2 == 0) || (y.abs() % 2 == 1 && x/2 % 2 == 1) {
                print!("\x1b[48;2;54;48;59m");
            }
            if -y as f32 == (a * (b * (x as f32) / 2.0).to_radians().sin()).floor() {
                let point = (a * (b * (x as f32) / 2.0).to_radians().sin()) - (a * (b * (x as f32) / 2.0).to_radians().sin()).floor();
                if point < 0.2 {
                    print!(",");
                } else if point < 0.4 {
                    print!(".");
                } else if point < 0.6 {
                    print!("-");
                } else if point < 0.8 {
                    print!("'");
                } else {
                    print!("`");
                }
            } else {
                print!(" ");
            }
        }
        println!("\x1b[0m");
    }
}
