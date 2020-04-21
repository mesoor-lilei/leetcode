#[allow(dead_code)]
fn num_identical_pairs(nums: Vec<i32>) -> i32 {
    let mut result = 0;
    for i in 0..nums.len() {
        for j in i + 1..nums.len() {
            if nums[i] == nums[j] {
                result += 1;
            }
        }
    }
    result
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(4, super::num_identical_pairs(vec![1, 2, 3, 1, 1, 3]));
    }
}
