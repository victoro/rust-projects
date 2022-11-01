use std::io;
use typename::TypeName;
fn main() {
    {
        // setting power of
        let base: i32 = 2;
        let eight_bit = i32::pow(base, 8) - 1;
        println!("8 bit value {}", eight_bit);
    }

    {
        let tup: (i32, char, u8, f64) = (500, 'p', 255, 3.14);

        {
            println!("access tuple element directly");
            println!("signed integer i32 => {}", tup.0);
            println!("char type => {}", tup.1);
            println!("unsigned 8 bit => {}", tup.2);
            println!("integer f64 => {}", tup.3);
        }

        {
            println!("assigning tuple data to variables");
            let (a, b, c, d) = tup;
            println!("a => {}", a.type_name_of());
            println!("b => {}", b.type_name_of());
            println!("c => {}", c.type_name_of());
            println!("d => {}", d.type_name_of());
        }
    }

    {
        // array
        // basic assign
        let _arr = [1, 2, 3, 5, 6];

        println!("pulling third element from array => {:?}", _arr[2]);

        // descriptive assign, shadowing _arr
        let _arr: [char; 5] = ['a', 'b', 'c', 'd', 'e'];

        println!("pulling third element from array => {:?}", _arr[2]);

        // initialize array with same value, 5 of 15s
        let _arr = [15; 5];
        println!("initialize array with same value");
        println!("pulling third element from array => {:?}", _arr[2]);
        println!("pulling forth element from array => {:?}", _arr[3]);
        println!("pulling seventh element from array => {:?}", _arr[4]);
    }

    {
        let months = [
            "January",
            "February",
            "March",
            "April",
            "May",
            "June",
            "July",
            "August",
            "September",
            "October",
            "November",
            "December",
        ];

        println!("Please enter a month number");
        let mut month_index = String::new();

        io::stdin()
            .read_line(&mut month_index)
            .expect("please ented a month number");
        let month_index: usize = month_index.trim().parse().expect("month is not a number");

        println!(
            "Display index month {}, {}",
            month_index, months[month_index]
        );
    }
}
