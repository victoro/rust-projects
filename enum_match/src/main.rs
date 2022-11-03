use rand::Rng;
use std::cmp::Ordering;
use typename::TypeName;
#[derive(Debug)]
enum IpAddrKind {
    V4(String),
    V6(String),
}
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}
impl Message {
    fn call(&self) {
        dbg!(self);
    }
}
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dimme,
    Quarter(UsState),
}
fn main() {
    let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let loopback = IpAddrKind::V6(String::from("::1"));
    dbg!(loopback);
    dbg!(IpAddrKind::V4(String::from("192.168.0.1")));
    dbg!(IpAddr::V4(192, 168, 0, 1));
    //struct via enum
    let message_struct = Message::Move { x: 11, y: 22 };
    dbg!(message_struct);
    // println!("{}", message_struct.type_name_of());not working
    let message = Message::Write(String::from("this is a string"));
    message.call();

    println!("Value of coin => {} cents", value_in_cents(Coin::Dimme));
    println!("Trying to display enum variant {:?}", Coin::Penny);
    println!(
        "Value of coin => {} cents",
        value_in_cents(Coin::Quarter(UsState::Alaska)),
    );
    let first_number = rand::thread_rng().gen_range(1..=100);
    let second_number = rand::thread_rng().gen_range(1..=100);
    match_int_compare(first_number, second_number);
    println!("Display a string `{}`", match_option(Some("option")));

    let x: Option<i32> = Some(5);
    let y: Option<i32> = match_option_and_return_option(x);
    let z: Option<i32> = match_option_and_return_option(None);
    println!("trying to print an option {:?} and {:?} {:?}", x, y, z);
}

// basic match
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Display penny value");
            1
        }
        Coin::Nickel => {
            println!("Display nickel value");
            5
        }
        Coin::Dimme => {
            println!("Display dimme value");
            10
        }

        Coin::Quarter(state) => {
            println!("Display quarter value of state {:?}", state);
            25
        }
    }
}

fn match_int_compare(a: i32, b: i32) {
    match a.cmp(&b) {
        Ordering::Less => println!("{a} is smaller than {b}"),
        Ordering::Equal => println!("{a} equals {b}"),
        Ordering::Greater => {
            println!("{a} is greater than {b}");
        }
    }
}

fn match_option(an_option: Option<&str>) -> String {
    match an_option {
        Some(o) => format!("this is the match string {}", o),
        None => format!("none is hit"),
    }
}

fn match_option_and_return_option(an_option: Option<i32>) -> Option<i32> {
    match an_option {
        Some(option) => Some(option + 1),
        None => None,
    }
}
