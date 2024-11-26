fn is_palindrome_base_10(num: u32) -> bool {
    let num_str = num.to_string();
    num_str == num_str.chars().rev().collect::<String>()
}

fn is_palindrome_base_2(num: u32) -> bool {
    let binary = format!("{:b}", num);
    binary == binary.chars().rev().collect::<String>()
}

fn main() {
    
}
