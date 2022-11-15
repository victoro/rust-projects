// #[derive(Debug)]
mod helpers;
use helpers as hlp;
use hlp::printing;
use hlp::string_playground as sp;
// use typename::TypeName;
use std::collections::HashMap;
fn main() {
    // initialize vector, definening type by annotation because vector is new
    let _v: Vec<i32> = Vec::new();

    // initialize vector by macro with implicit content type [default i32] similar with v1: Vec<i32>
    let _v1 = vec![1, 2, 4];

    // play_with_integers();

    // play_with_strings();

    play_with_hashes();
}

fn play_with_hashes() {
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("green"), 20);
    let a_key = String::from("yellow");

    //update hash if key does not exists
    scores.entry(a_key).or_insert(500);
    //green exists no update done
    scores.entry(String::from("green")).or_insert(500);
    //update if exists
    scores.insert(String::from("green"), 10000);

    let a_key = String::from("blue");
    println!("blue score {}", scores.get(&a_key).copied().unwrap_or(0));

    for (k, v) in &scores {
        println!("{} => {}", k, v);
    }

    dbg!(scores);

    let a_string = "hello world wonderful world of my beautiful world";
    let mut string_map = HashMap::new();
    for word in a_string.split_whitespace() {
        println!("{}", word);
        let count = string_map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("string map {:?}", string_map);
    // dbg!(&a_string.split_whitespace());
}

fn play_with_strings() {
    let mut v: Vec<String> = Vec::new();

    let mut s1 = String::from("tic");
    let s2 = "tac".to_string();
    let s3 = "lorem lipsum et".to_string();
    v.push(String::from("tic"));
    v.push("tac".to_string());
    v.push(s3);

    sp::a_push(&mut s1);

    let hello = "Здравствуйте";
    let a_slice = &hello[0..4];
    println!("a slice {}", a_slice);
    for e in hello.chars() {
        println!("printing 'hello' chars {}", e);
    }
    dbg!(&v);

    println!("{}", s1);

    // println!("s1 => {}, s2 => {}, s3 => {}", s1, s2, s3);
    // let s4 = format!("{}-{}", s1, s2);
    // println!("Final string {}", s4);
    // println!("{}", sp::concatenate_me(s1, s2, Some(0)));
    println!("{}", sp::concatenate_me(v, Some(1)));
    // println!("{}", sp::get_slice(&s3, 100, 1000));
}

fn _play_with_integers() {
    //using annotation to imprint info
    let mut v: Vec<i32> = vec![1, 3, 6, 7, 8];
    let current_fn = stringify!(hlp::mutate::push_value);
    println!("{}", printing::header(current_fn));
    hlp::mutate::push_value(&mut v, 18);
    println!("{}", printing::footer());

    let current_fn = stringify!(hlp::get_values::by_index);
    //TODO: see how to call before and after
    println!("{}", printing::header(current_fn));
    // referencing vector values first method [by index]
    hlp::get_values::by_index(&mut v, 5);
    println!("{}", printing::footer());

    //referencing value by get
    let current_fn = stringify!(hlp::get_values::by_get);
    println!("{}", printing::header(current_fn));
    hlp::get_values::by_get(&mut v, 3);
    println!("{}", printing::footer());

    //referencing value by get with if let
    let current_fn = stringify!(hlp::get_values::by_get_with_if_let);
    println!("{}", printing::header(current_fn));
    hlp::get_values::by_get_with_if_let(&mut v, 100);
    println!("{}", printing::footer());

    //display vector elements by iterating it
    let current_fn = stringify!(hlp::get_values::iterate_and_display);
    println!("{}", printing::header(current_fn));
    hlp::get_values::iterate_and_display(&mut v);
    println!("{}", printing::footer());

    //display vector elements while iterating and altering it
    let current_fn = stringify!(hlp::get_values::iterate_and_display_and_use_mut);
    println!("{}", printing::header(current_fn));
    hlp::get_values::iterate_and_display_and_use_mut(&mut v);
    println!("{}", printing::footer());

    dbg!(v);
    println!("final vector ^^^^");
}
