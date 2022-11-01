use std::io;

fn main() {
    let mut nth_number = String::new();
    println!("Please insert the nth number of Fibonacci series");
    io::stdin()
        .read_line(&mut nth_number)
        .expect("An integer value is required");

    let nth_number: u32 = nth_number
        .trim()
        .parse()
        .expect("A positive number is required.");
    let mut counter: u32 = 0;
    let mut nth_f_number: u32 = 0;
    let mut prev_nth_f_number: u32 = 0;
    println!("Fibonacci series until nth {}", nth_number);
    while counter < nth_number {
        if counter == 1 {
            nth_f_number = 1;
        } else {
            (prev_nth_f_number, nth_f_number) = fibonacci(prev_nth_f_number, nth_f_number);
        }

        println!(" {}  => {}", counter + 1, nth_f_number);
        counter += 1;
    }

    println!("{}th fibonacci number => {}", nth_number, nth_f_number);
}

fn fibonacci(x: u32, y: u32) -> (u32, u32) {
    (y, y + x)
}
