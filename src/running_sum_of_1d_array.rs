#[allow(dead_code)]
fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut result = vec![0; nums.len()];
    result[0] = nums[0];
    for i in 1..nums.len() {
        result[i] = result[i - 1] + nums[i];
    }
    result
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(vec![1, 3, 6, 10], super::running_sum(vec![1, 2, 3, 4]));
    }
}
