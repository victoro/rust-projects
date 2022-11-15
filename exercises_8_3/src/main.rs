mod helpers;

use helpers::median_and_mode as mm;

fn main() {
    let mut v: Vec<i32> = vec![1, 2, 22, 55, 1, 5, 4, 5, 77, 12, 2, 1, 2, 55, 67];
    mm::vector_median(&mut v);
    println!("Mode of v {:?}", v);
    let (of, max) = mm::mode_variant_1(&v).unwrap();
    println!("We have {} of {}s", max, of);

    println!(
        "mode variant two, numbers with the ocurrences as max  => {:?}",
        mm::mode_variant_2(&v)
    );

    let a_string = " The first consonant of each word is moved to the end of the word and “ay” 
    is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added 
    to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!";
    println!("Converting this string into pig latin => `{}`", a_string);
    helpers::pig_latin::convert(a_string);
}
