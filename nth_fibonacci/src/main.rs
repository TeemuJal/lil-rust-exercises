fn main() {
    for i in 0..=30 {
        println!("{}", nth_fibonacci(i));
    }
}

fn nth_fibonacci(n: u32) -> u32 {
    if n < 2 {
        return n;
    }
    let mut prev = 0;
    let mut curr = 1;
    for _i in 0..=(n - 2) {
        let res = prev + curr;
        prev = curr;
        curr = res;
    }
    return curr;
}
