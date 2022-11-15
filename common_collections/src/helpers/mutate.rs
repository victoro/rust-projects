pub fn push_value(v: &mut Vec<i32>, value: i32) {
    //mod in fn scope
    use super::printing;
    printing::echo_this(format!("pushing {} into vector", value));
    v.push(value);
}
