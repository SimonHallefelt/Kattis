fn gcd(mut n: i64, mut m: i64) -> i64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
          std::mem::swap(&mut m, &mut n);
        }
        m %= n;
    }
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(1, gcd(11, 13));
        assert_eq!(2, gcd(2, 8));
        assert_eq!(3, gcd(6, 9));
        assert_eq!(3, gcd(15, 18));
        assert_eq!(6, gcd(12, 18));
    }
}