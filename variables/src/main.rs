fn main() {
    let mut x = 5;
    println!("This is the value of {x}");
    x = 6;
    println!("THis is another value of {x}");

    let spaces = "     ";

    let spaces = spaces.len();

    {
        let spaces: u32 = 4;
        println!("context spaces = {spaces}");
    }

    println!("Length of spaces is {spaces}");
}
