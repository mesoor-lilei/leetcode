#[allow(dead_code)]
fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
    let mut bottle = num_bottles;
    while bottle >= num_exchange {
        num_bottles += 1;
        bottle -= num_exchange;
        bottle += 1;
    }
    num_bottles
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(13, super::num_water_bottles(9, 3));
    }
}
