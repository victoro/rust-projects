// use std::fmt;
// static DAILY_CAROL_DATASET: [(&str, &str); 12] = [
//     ("first", "A partridge in a pear tree"),
//     ("second", "Two turtle doves"),
//     ("third", "Three French hens"),
//     ("forth", "four calling birds"),
//     ("fifth", "five gold rings"),
//     ("sixth", "six geese a-laying"),
//     ("seventh", "seven swans a-swimming"),
//     ("eigth", "eight maids a-milking"),
//     ("ninth", "nine ladies dancing"),
//     ("tenth", "ten lords a-leaping"),
//     ("eleventh", "eleven pipers piping"),
//     ("twelfth", "twelve drummers drumming"),
// ];

static DAILY_CAROL_TEXT: [&str; 12] = [
    "A partridge in a pear tree",
    "Two turtle doves",
    "Three French hens",
    "four calling birds",
    "five gold rings",
    "six geese a-laying",
    "seven swans a-swimming",
    "eight maids a-milking",
    "nine ladies dancing",
    "ten lords a-leaping",
    "eleven pipers piping",
    "twelve drummers drumming",
];

static DAYS: [&str; 12] = [
    "first", "second", "third", "forth", "fifth", "sixth", "seventh", "eighth", "nineth", "tenth",
    "eleventh", "twelfth",
];

fn main() {
    // let mut carol = String::new();

    for (i, carol_day) in DAYS.iter().enumerate() {
        println!("{}", carol_starter(Some(carol_day)));
        print_daily_slice(&DAILY_CAROL_TEXT[0..i + 1]);
        println!("\n");
    }
}

fn carol_starter(day: Option<&str>) -> String {
    match day {
        Some(n) => format!("On the {n} day of Christmas my true love sent to me"),
        None => format!("No text"),
    }
}

fn print_daily_slice(slice: &[&str]) {
    println!("{}", slice.join("\n"));
}
