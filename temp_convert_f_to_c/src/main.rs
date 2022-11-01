use std::io;
fn main() {
    let mut counter = 0;
    loop {
        if counter == 3 {
            break;
        }
        println!("Please insert Fahrenheit value");
        let mut temp_f = String::new();
        io::stdin().read_line(&mut temp_f).expect("Pick a number");
        let temp_f: f64 = temp_f.trim().parse().expect("A numeric value is required");
        println!(
            "Fahrenheit temperature {}, Celsius temperature {:.3}",
            temp_f,
            calculate_celsius(temp_f)
        );
        counter += 1;
    }
}

fn calculate_celsius(temp_f: f64) -> f64 {
    (temp_f - 32.00) * (5.0 / 9.0)
}
