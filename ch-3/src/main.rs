fn main() {
    println!("Hello, world!");

    let x = 6.0;

    println!("{} C is {} F", x, to_foreheit(x));
    println!("{}", fibonacci(15));
}

fn to_foreheit(temp: f32) -> f32 {
    return (temp * (9.0 / 5.0)) + 32.0;
}

fn fibonacci(n: u32) -> u64 {
    let mut fib = 1;
    let mut fib_1: u64 = 0;

    for _ in 0..n-1 {
        let old = fib;
        fib = fib_1 + fib;
        fib_1 = old;
    }

    return fib;
}
