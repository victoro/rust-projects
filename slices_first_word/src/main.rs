fn main() {
    let test_string =
        String::from("please get string of words separated by spaces and returns the first word");
    let first_word = print_string_as_bytes(&test_string);
    println!("{}", first_word);
}
// TODO return here to practice strings
fn print_string_as_bytes(a_string: &String) -> &str {
    let string_bytes = a_string.as_bytes();
    for (i, &letter) in string_bytes.iter().enumerate() {
        if letter == b' ' {
            return &a_string[..i];
        }
    }
    &a_string[..]
}
