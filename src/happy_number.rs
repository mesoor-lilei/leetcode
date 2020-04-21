#[allow(dead_code)]
fn is_happy(mut n: i32) -> bool {
    if n <= 0 {
        return false;
    }
    while n != 1 {
        let mut temp = 0;
        while n > 0 {
            temp += (n % 10) * (n % 10);
            n /= 10;
        }
        n = temp;
        // 当快乐数始终变不了 1，结果为 4 并陷入无限循环时
        if n == 4 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert!(super::is_happy(19));
    }
}
