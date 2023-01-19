fn main() {
    println!("Hello, world!");

    let sum = add_numbers(10, 20);
    println!("The sum is {}.", sum)
}

fn add_numbers(a: i32, b: i32) -> i32 {
    let sum = a + b;

    return sum;
}
