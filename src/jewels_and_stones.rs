use std::collections::HashSet;

#[allow(dead_code)]
fn num_jewels_in_stones(jewels: String, stones: String) -> i32 {
    stones
        .chars()
        .filter(|x| jewels.chars().collect::<HashSet<char>>().contains(x))
        .count() as i32
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(
            3,
            super::num_jewels_in_stones(String::from("aA"), String::from("aAAbbbb"))
        );
    }
}
