fn main() {
    println!("{:?}", fizz_buzz(3));
    println!("{:?}", fizz_buzz(5));
    println!("{:?}", fizz_buzz(15));
}

fn fizz_buzz(n: u32) -> Vec<String> {
    let result = vec![String::new(); usize::try_from(n).unwrap()];
    return result
        .into_iter()
        .enumerate()
        .map(|(idx, _item)| {
            let mut string = String::new();
            let idx = idx + 1;
            if idx % 3 == 0 {
                string.push_str("Fizz");
            }
            if idx % 5 == 0 {
                string.push_str("Buzz");
            }
            if string == "" {
                string.push_str(&idx.to_string())
            }
            return string;
        })
        .collect();
}
