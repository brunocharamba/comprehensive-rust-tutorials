fn fibonnaci(n: u32) -> u32 {
    if n <= 2 {
        1
    } else {
        fibonnaci(n-1) + fibonnaci(n-2)
    }
}

fn main() {
    let n = 10;
    println!("fib(n) = {}", fibonnaci(n));
}
