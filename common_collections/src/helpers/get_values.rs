use super::printing; //mod in mod scope

pub fn by_index(v: &mut Vec<i32>, index: usize) {
    let a_val: &i32 = &v[index];
    println!("print {index}th vector value {}", a_val);
}

pub fn by_get(v: &mut Vec<i32>, index: usize) {
    printing::echo_this("using get".to_string());
    let el: Option<&i32> = v.get(index);
    match el {
        Some(el) => println!("The {}th element is {}", index, el),
        None => println!("There is no element."),
    }
}

pub fn by_get_with_if_let(v: &mut Vec<i32>, index: usize) {
    printing::echo_this("using get with if let".to_string());
    let el: Option<&i32> = v.get(index);

    if let Some(n) = el {
        println!("The {}th element is {}", index, n);
    } else {
        printing::echo_this("element does not exists".to_string());
    }
}

pub fn iterate_and_display(v: &mut Vec<i32>) {
    printing::echo_this("using iterate and display elements".to_string());
    for (i, el) in v.iter().enumerate() {
        println!("index[{}] ==> {}", i, el);
    }
}

pub fn iterate_and_display_and_use_mut(v: &mut Vec<i32>) {
    printing::echo_this("using iterate and display mut elements".to_string());
    for el in &mut v.iter_mut() {
        *el += 30;
    }
    iterate_and_display(v);
}
