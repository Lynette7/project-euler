use itertools::Itertools;

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return  false;;
    }
    let mut i = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return  false;;
        }
        i += 6;
    }
    true
}

fn main() {
    let mut largest_pandigiotal_prime = 0;

    for n in (1..=9).rev() {
        let digits: Vec<usize> = (1..=n).collect();
        let permutations = digits.iter().permutations(n);

    }
}
