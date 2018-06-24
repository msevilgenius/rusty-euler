pub fn solve() -> i64{
    sum_multiples(1000)
}

fn sum_multiples(max : i64) -> i64{
    (1..max).filter(|i| i % 3 == 0 || i % 5 == 0).sum()
}