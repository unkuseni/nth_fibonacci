use std::io;
fn main() {
    println!("Generate a nth fibonacci number");
    loop {
        let mut a: u128 = 0;
        let mut b: u128  = 1;
        println!("Enter a number: ");
        let mut n: String = String::new();
        io::stdin().read_line(&mut n).expect("Failed to read line");
        let  nth: u128 = n.trim().parse().expect("Not a number");
        let mut n = nth - 1;
        if n == 0 {
            println!("The {}th fibonacci number is {}.", nth, a);
        } else if n == 1 {
            println!("The {}th fibonacci number is {}.", nth, b);
        } else if n > 1 {
            while n >= 2 {
                let c = a.wrapping_add(b);
                a = b;
                b = c;
                n -= 1;
            }
            println!("The {}th fibonacci number is {}.", nth, b);
        }
    }
}
