const VOWELS: [char; 6] = ['a', 'e', 'i', 'o', 'u', 'y'];
const TEST_STRINGS: [&str; 5] = ["", "first", "apple", "first apple", "this is a test string"];

fn main() {
    for string in TEST_STRINGS {
        println!(
            "\"{}\" in pig latin: \"{}\"",
            string,
            string_to_pig_latin(string)
        )
    }
}

fn string_to_pig_latin(string: &str) -> String {
    let result: Vec<String> = string
        .trim()
        .split_whitespace()
        .map(|word| word_to_pig_latin(word))
        .collect();
    return result.join(" ");
}

fn word_to_pig_latin(string: &str) -> String {
    if string.len() == 0 {
        return "".to_string();
    }
    let first_char = string.chars().collect::<Vec<char>>()[0];
    if VOWELS.contains(&first_char) {
        return format!("{string}-hay");
    }
    let result_chars = &string.chars().collect::<Vec<char>>()[1..];
    let mut result: String = "".to_string();
    for char in result_chars {
        result.push(*char);
    }
    result.push_str(&format!("-{first_char}ay"));
    return result;
}
