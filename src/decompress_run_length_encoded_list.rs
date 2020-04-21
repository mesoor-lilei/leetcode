#[allow(dead_code)]
pub fn decompress_rl_elist(nums: Vec<i32>) -> Vec<i32> {
    let mut result = Vec::new();
    for i in (0..nums.len()).step_by(2) {
        result.append(&mut vec![nums[i + 1]; nums[i] as usize]);
    }
    result
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(
            vec![2, 4, 4, 4],
            super::decompress_rl_elist(vec![1, 2, 3, 4])
        );
    }
}
