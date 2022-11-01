use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut count = 0;
    'make_check: loop {
        println!("count {}", count);
        let mut remaining = 10;

        loop {
            let mut pick_a_number = String::new();
            let generated_number: i32 = generate_number();

            println!("Pick a number");
            println!("Generated number {}", generated_number);
            println!("Remaining {}", remaining);
            println!("Remaining count {}", count);
            if remaining == 8 {
                break;
            }
            if count > 2 {
                break 'make_check;
            }
            io::stdin()
                .read_line(&mut pick_a_number)
                .expect("Invalid number");

            println!("Over here HERE picked number {}", pick_a_number);

            let pick_a_number: i32 = match pick_a_number.trim().parse() {
                Ok(num) => num,
                Err(_) => {
                    println!("it seems we have an error");
                    continue;
                }
            };

            println!("HERE picked number {}", pick_a_number);

            // check_number_by_if_else(plus_one(pick_a_number), generated_number);
            // check_number_by_if_else(plus_ten(pick_a_number), generated_number);
            // check_number_by_switch(plus_one(pick_a_number), generated_number);
            // check_number_by_switch(plus_ten(pick_a_number), generated_number);
            check_number_by_switch(check_ternary(pick_a_number), generated_number);

            remaining -= 1;
        }

        count += 1;
    }
}

fn generate_number() -> i32 {
    rand::thread_rng().gen_range(1..=100)
}

fn _plus_one(number: i32) -> i32 {
    println!("adding 1 to {}", number);
    number + 1
}

fn _plus_ten(number: i32) -> i32 {
    println!("adding 10 to {}", number);
    number + 10
}

fn _check_number_by_if_else(an_integer: i32, generated_number: i32) {
    println!("if else check");
    if an_integer > generated_number {
        println!(
            " the number {} is greater than {}",
            an_integer, generated_number
        );
    } else if an_integer == generated_number {
        println!(
            " the number {} is equal with {}",
            an_integer, generated_number
        );
    } else {
        println!(
            " the number {} is less than {}",
            an_integer, generated_number
        );
    }
}

fn check_number_by_switch(number: i32, generated_number: i32) {
    println!("swtich scheck");
    match number.cmp(&generated_number) {
        Ordering::Less => println!("Number {} is smaller than {}", number, generated_number),
        Ordering::Equal => println!("Number {} is equal with {}", number, generated_number),
        Ordering::Greater => println!("Number {} is greater than {}", number, generated_number),
    };
}

fn check_ternary(number: i32) -> i32 {
    println!("basic ternary");
    let number = if number < 15 { number + 15 } else { number + 5 };
    number
}
