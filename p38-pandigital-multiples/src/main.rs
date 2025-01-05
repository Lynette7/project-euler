fn is_pandigital(n: &str) -> bool {
    let mut digits: Vec<char> = n.chars().collect();
    digits.sort_unstable();
    digits == ['1', '2', '3', '4', '5', '6', '7', '8', '9']
}

fn main() {
    let mut largest_pandigital = 0;

    for x in 1..=9999 {
        let mut concatenated = String::new();
        let mut n = 1;

        while concatenated.len() < 9 {
            concatenated.push_str(&(x * n).to_string());
            n += 1;
        }

        if concatenated.len() == 9 && is_pandigital(&concatenated) {
            let value = concatenated.parse::<u64>().unwrap();
            largest_pandigital = largest_pandigital.max(value);
        }
    }

    println!("The largest 1 to 9 pandigital number is: {}", largest_pandigital);
}
