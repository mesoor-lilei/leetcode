use std::cmp::min;

#[allow(dead_code)]
fn min_path_sum(grid: Vec<Vec<i32>>) -> i32 {
    let row_size = grid.len();
    let grid_size = grid[0].len();
    let mut result = vec![vec![1; grid_size]; row_size];
    result[0][0] = grid[0][0];
    for i in 1..row_size {
        result[i][0] = grid[i][0] + result[i - 1][0];
    }
    for j in 1..grid_size {
        result[0][j] = grid[0][j] + result[0][j - 1];
    }
    for i in 1..row_size {
        for j in 1..grid_size {
            result[i][j] = min(result[i - 1][j], result[i][j - 1]) + grid[i][j];
        }
    }
    result[row_size - 1][grid_size - 1]
}

#[cfg(test)]
mod test {
    #[test]
    fn test() {
        assert_eq!(
            7,
            super::min_path_sum(vec![vec![1, 3, 1], vec![1, 5, 1], vec![4, 2, 1]])
        );
    }
}
