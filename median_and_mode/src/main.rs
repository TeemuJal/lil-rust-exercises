use std::{collections::HashMap, num::ParseIntError};

fn main() {
    test_get_median("4 9 3  ");
    test_get_median("-1 2 -3 -4 5 6 0");
    test_get_median("10 9 1 2");
    test_get_median(" 10 1 3 -4 -4 -4 2 3");
    test_get_median("1a 2 3");
    test_get_median("  ");
    test_get_median(" 1    2 ");
    test_get_median("10");

    test_get_mode("4 9 3  ");
    test_get_mode("-1 2 -3 -4 5 6 0");
    test_get_mode("10 9 1 2");
    test_get_mode(" 10 1 3 -4 -4 -4 2 3");
    test_get_mode("1a 2 3");
    test_get_mode("  ");
    test_get_mode(" 1    2 ");
    test_get_mode("10");
}

fn test_get_median(integer_list: &str) {
    println!("Getting median for integer list: {}", integer_list);
    match get_median(integer_list) {
        Ok(val) => println!("Median: {}\n", val),
        Err(err) => println!("Got error: {}\n", err),
    };
}

fn test_get_mode(integer_list: &str) {
    println!("Getting mode for integer list: {}", integer_list);
    match get_mode(integer_list) {
        Ok(val) => println!("Mode: {}\n", val),
        Err(err) => println!("Got error: {}\n", err),
    };
}

fn get_median(integer_list: &str) -> Result<f64, String> {
    let mut integer_vec = match get_int_vec_from_str(integer_list) {
        Ok(vec) => vec,
        Err(err) => return Err(err),
    };
    integer_vec.sort();
    println!("Sorted integer list: {:?}", integer_vec);

    let vec_len = integer_vec.len();
    if vec_len % 2 == 0 {
        let middle_index_left = (vec_len / 2) - 1;
        let middle_index_right = vec_len / 2;
        return Ok((integer_vec[middle_index_left] + integer_vec[middle_index_right]) as f64 / 2.0);
    } else {
        return Ok(integer_vec[vec_len / 2] as f64);
    }
}

fn get_mode(integer_list: &str) -> Result<i32, String> {
    let integer_vec = match get_int_vec_from_str(integer_list) {
        Ok(vec) => vec,
        Err(err) => return Err(err),
    };
    let mut map = HashMap::new();
    for val in integer_vec {
        let count = map.entry(val).or_insert(0);
        *count += 1;
    }
    let mut mode_pair = (0, 0);
    for pair in map {
        if pair.1 > mode_pair.1 {
            mode_pair = pair;
        }           
    }
    return Ok(mode_pair.0);
}

fn get_int_vec_from_str(integer_list: &str) -> Result<Vec<i32>, String> {
    let integer_vec: Result<Vec<i32>, ParseIntError> = integer_list
        .trim()
        .split_whitespace()
        .map(|word| word.parse())
        .collect();

    let integer_vec = match integer_vec {
        Ok(vec) => vec,
        Err(error) => return Err(error.to_string()),
    };
    let vec_len = integer_vec.len();
    if vec_len == 0 {
        return Err("empty list".to_string());
    }
    return Ok(integer_vec);
}
