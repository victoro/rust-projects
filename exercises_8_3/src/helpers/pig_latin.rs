static VOWELS: [&str; 5] = ["a", "e", "i", "o", "u"];

pub fn convert(a_string: &str) -> String {
    let mut new_string: Vec<String> = vec![];
    for word in a_string.split_whitespace() {
        new_string.push(process(word, has_vowel(word)));
    }
    println!("{:?}", new_string.join(" "));
    new_string.join(" ")
}

fn has_vowel(word: &str) -> bool {
    if VOWELS
        .iter()
        .any(|vowel| word.to_lowercase().starts_with(vowel))
    {
        return true;
    }
    false
}

fn process(word: &str, has_vowel: bool) -> String {
    match has_vowel {
        true => handle_vowel(word),
        false => handle_consonant(word),
    }
}

fn handle_vowel(word: &str) -> String {
    let (new_word, _) = drop_first_letter_of(word);
    format!("{}-{}", new_word, "hay")
}

fn handle_consonant(word: &str) -> String {
    let (new_word, letter) = drop_first_letter_of(word);
    format!("{}-{}{}", new_word, letter, "ay")
}

fn drop_first_letter_of(word: &str) -> (&str, char) {
    let (_, letter) = word.char_indices().next().unwrap();
    let mut chars = word.chars();
    chars.next();
    (chars.as_str(), letter)
}
