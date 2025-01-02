fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn permute(digits: &mut Vec<u64>, start: usize, result: &mut Vec<u64>) {
    if start == digits.len() {
        let number = digits.iter().fold(0, |acc, &digit| acc * 10 + digit);
        result.push(number);
    } else {
        for i in start..digits.len() {
            digits.swap(start, i);
            permute(digits, start + 1, result);
            digits.swap(start, i);
        }
    }
}

fn generate_pandigital_numbers(n: u64) -> Vec<u64> {
    let mut digits: Vec<u64> = (1..=n).collect();
    let mut pandigital_numbers = Vec::new();

    permute(&mut digits, 0, &mut pandigital_numbers);

    pandigital_numbers
}

fn main() {
    let mut largest_pandigital_prime = 0;

    for n in (1..=9).rev() {
        let pandigital_numbers = generate_pandigital_numbers(n);

        for &number in &pandigital_numbers {
            if is_prime(number) {
                largest_pandigital_prime = largest_pandigital_prime.max(number);
            }
        }

        if largest_pandigital_prime > 0 {
            break;
        }
    }

    println!("The largest n-digit pandigital prime is: {}", largest_pandigital_prime);
}
