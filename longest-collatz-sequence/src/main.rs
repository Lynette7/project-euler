use std::collections::HashMap;

/// Compute the Collatz sequence length for a given number
fn collatz_sequence_length(n: u64, memo: &mut HashMap<u64, u64>) -> u64 {
    // If we've already computed this number's sequence length, return it
    if let Some(&length) = memo.get(&n) {
        return length;
    }

    let length = match n {
        1 => 1,
        _ if n % 2 == 0 => 1 + collatz_sequence_length(n / 2, memo),
        _ => 1 + collatz_sequence_length(3 * n + 1, memo),
    };

    memo.insert(n, length);
    length
}

fn find_longest_collatz_chain(limit: u64) -> (u64, u64) {
    let mut memo = HashMap::new();
    let mut max_length = 0;
    let mut max_start = 0;

    for start in 1..limit {
        let current_length = collatz_sequence_length(start, &mut memo);
        
        if current_length > max_length {
            max_length = current_length;
            max_start = start;
        }
    }

    (max_start, max_length)
}

fn main() {
    let limit = 1_000_000;
    let (starting_number, chain_length) = find_longest_collatz_chain(limit);
    
    println!("Limit: {}", limit);
    println!("Starting number: {}", starting_number);
    println!("Longest chain length: {}", chain_length);
}
