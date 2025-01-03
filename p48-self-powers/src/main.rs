fn main() {
    const MOD: u64 = 10_000_000_000;
    let mut sum: u64 = 0;

    for i in 1..=1000 {
        let mut power = 1u64;
        for _ in 0..i {
            power = power * i % MOD;
        }
        sum = (sum + power) % MOD;
    }

    println!("The last ten digits of the series are: {:010}", sum);
}