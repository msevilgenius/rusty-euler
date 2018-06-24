pub fn solve() -> i64{
    sum_even_fibonacci(4000000)
}

fn sum_even_fibonacci(max : i64) -> i64{
    let fib =  Fib::new(max);
    fib.filter(|i| i % 2 == 0).sum()
}

struct Fib {
    previous: i64,
    current: i64,
    max: i64,
}

impl Fib {
    fn new(max: i64) -> Fib {
        Fib {
            previous: 1,
            current: 1,
            max: max
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib() {
        let mut fib = Fib::new(10);
        //assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(1));
        assert_eq!(fib.next(), Some(2));
        assert_eq!(fib.next(), Some(3));
        assert_eq!(fib.next(), Some(5));
        assert_eq!(fib.next(), Some(8));
        assert_eq!(fib.next(), None);
    }
}

impl Iterator for Fib {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        let tmp = self.current;
        self.current += self.previous;
        self.previous = tmp;

        if self.previous < self.max{
            Some(self.previous)
        } else {
            None
        }
    }
}