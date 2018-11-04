use std::io;
use std::num;

fn main() -> Result<(), num::ParseIntError> {
    loop{
        println!("Please enter a number below:");
        let mut inp = String::new();
        io::stdin().read_line(&mut inp).unwrap();

        let inp: u32 = inp.trim().parse()?;
        println!("Fibbonacci {} is {}.", inp, fib(inp));
    }
}

fn fib(x : u32) -> u32 {
    match x {
        0 => 1,
        1 => 1,
        n => fib(n-1) + fib(n-2)
    }
}
