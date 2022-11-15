use super::printing;

pub fn concatenate_me(v: Vec<String>, opt_type: Option<usize>) -> String {
    printing::echo_this(v.join(" => "));
    // let s1 = v.
    // match opt_type {
    //     Some(0) => {
    //         printing::echo_this("concatenate string by add operator + ".to_string());
    //         s1 + &s2
    //     }
    //     Some(1) => {
    //         printing::echo_this("concatenate string by push_str ".to_string());
    //         let mut new_s = s1;
    //         new_s.push_str(&s2);
    //         new_s
    //     }
    //     _ => String::from("No option provided"),
    // }

    //flattening b4 concat
    v.concat()
}

pub fn get_slice(lorem: &str, start: usize, end: usize) -> &str {
    println!("taking slice from `{}`", &lorem);
    if start > end {
        let start: usize = 0;
    }
    if end > lorem.len() {
        let end = lorem.len();
    }
    printing::echo_this(format!(" getting slice from {} to {}", start, end));
    let slice: &str = &lorem[start..end];
    println!("a slice {}", &slice);
    slice
}

pub fn a_push(s1: &mut String) {
    s1.push_str("-");
    s1.push_str("t0e");
}

//vector of strings
pub fn a_join() {
    // mutable vector
    let mut mut_v = vec!["mama", "are", "mere"];
    println!("{}", mut_v.join(" "));

    mut_v.push("si");
    mut_v.push("pere");
    println!("{}", mut_v.join(" "));
}
