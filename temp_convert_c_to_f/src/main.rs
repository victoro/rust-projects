use std::io;

fn main() {
    let mut counter = 0;
    loop {
        if counter == 3 {
            break;
        }
        println!("Please insert Celsius value");
        let mut temp_c = String::new();
        io::stdin().read_line(&mut temp_c).expect("Pick a number");
        let temp_c: f64 = temp_c.trim().parse().expect("A numeric value is required");
        println!(
            "Celsius temperature {}, Fahrenheit temperature {:.3}",
            temp_c,
            calculate_celsius(temp_c)
        );
        counter += 1;
    }
}

fn calculate_celsius(temp_c: f64) -> f64 {
    temp_c * (9.0 / 5.0) + 32.00
}
