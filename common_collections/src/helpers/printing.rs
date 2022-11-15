pub fn header(a_string: &str) -> String {
    format!("===== appling to vector function {} =====", a_string)
}
pub fn footer() -> String {
    String::from("===== ending =====")
}
pub fn echo_this(a_string: String) {
    let a = "---->".to_string();
    println!("{} {}", a, a_string);
}
