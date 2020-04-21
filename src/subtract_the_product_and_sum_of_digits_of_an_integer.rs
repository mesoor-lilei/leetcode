#[allow(dead_code)]
fn subtract_product_and_sum(mut n: i32) -> i32 {
    let mut sum = 0;
    let mut mul = 1;
    while n != 0 {
        let temp = n % 10;
        sum += temp;
        mul *= temp;
        n /= 10;
    }
    mul - sum
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(15, super::subtract_product_and_sum(234));
    }
}
