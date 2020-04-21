#[allow(dead_code)]
fn number_of_steps(mut num: i32) -> i32 {
    let mut result = 0;
    while num != 0 {
        if num & 1 == 0 {
            num >>= 1;
        } else {
            num -= 1;
        }
        result += 1;
    }
    result
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(6, super::number_of_steps(14));
    }
}
