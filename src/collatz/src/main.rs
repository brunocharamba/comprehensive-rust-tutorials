fn collatz_sequence_optimized(mut n: i32) -> u32 {
    let mut count: u32 = 1;

    while n > 1 {
        n = if n % 2 == 1 { 3 * n + 1 } else { n / 2 };
        count += 1;
    }

    count
}

#[test]
fn test_collatz_count() {
    assert_eq!(collatz_sequence_optimized(3), 8);
}

fn main() {
    println!("sequence: {}", collatz_sequence_optimized(3));
}
