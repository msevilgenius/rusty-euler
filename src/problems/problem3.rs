pub fn solve() -> i64{
    largest_prime_factor_faster(600851475143)
}

// extremely slow original version
fn largest_prime_factor(num: i64) -> i64 {
    let is_prime = |x : &i64| -> bool { (2..x/2+1).filter(|y| x % y == 0).next() == None };

    let primes = (2..num/2).filter(|x| is_prime(x));
    primes.filter(|p| num % p == 0).max().unwrap()
}

fn largest_prime_factor_faster(num: i64) -> i64 {
    let is_prime = |x : i64| -> bool { (2..x/2+1).filter(|y| x % y == 0).next() == None };

    let mut primes = (2..num/2).filter(|x| is_prime(*x));
    let mut n = num;
    while !is_prime(n){
        let p = primes.next().unwrap();
        while n % p == 0{
            n /= p;
        }
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_factor() {
        assert_eq!(largest_prime_factor(13195), 29);
    }
}