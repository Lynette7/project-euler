use std::collections::HashSet;

fn is_palindrome_base_10(num: u32) -> bool {
    let num_str = num.to_string();
    num_str == num_str.chars().rev().collect::<String>()
}

fn is_palindrome_base_2(num: u32) -> bool {
    let binary = format!("{:b}", num);
    binary == binary.chars().rev().collect::<String>()
}

fn palindrome_sum() -> u32 {
    let mut palindrome_set = HashSet::new();

    for num in 1..1000000 {
        if is_palindrome_base_10(num) && is_palindrome_base_2(num) {
            palindrome_set.insert(num);
        }
    }

    println!("Palindromic numbers found: {:?}", palindrome_set);
    palindrome_set.iter().sum()
}

fn main() {
    let result = palindrome_sum();
    println!("The sum of palindromic numbers in base 10 and base 2 is {}", result);
}
