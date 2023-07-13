use std::cmp;
/// 931. 下降路径最小和
/// 给你一个 n x n 的 方形 整数数组matrix ，请你找出并返回通过 matrix 的下降路径 的 最小和 。
/// 下降路径 可以从第一行中的任何元素开始，并从每一行中选择一个元素。
/// 在下一行选择的元素和当前行所选元素最多相隔一列（即位于正下方或者沿对角线向左或者向右的第一个元素）。
/// 具体来说，位置 (row, col) 的下一个元素应当是 (row + 1, col - 1)、(row + 1, col) 或者 (row + 1, col + 1) 。

/// 提示
/// n == matrix.length == matrix[i].length
/// 1 <= n <= 100
/// -100 <= matrix[i][j] <= 100
/// [来源](https://leetcode.cn/problems/minimum-falling-path-sum/)
// lc看不懂的解
pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (matrix.len(), matrix[0].len());
    for i in (1..m) {
        (0..n).for_each(|j| {
            matrix[i][j] += *matrix[i - 1][(j.max(1) - 1)..(j.min(n - 2) + 2)]
                .iter()
                .min()
                .unwrap()
        });
    }
    *matrix[m - 1].iter().min().unwrap()
}

// lc执行最优解
pub fn min_falling_path_sum_1(matrix: Vec<Vec<i32>>) -> i32 {
    let m=matrix.len();
    let mut res=i32::MAX;
    let mut memo:Vec<Vec<i32>>=vec![vec![66666;m];m];
    for i in 0..m{
        res=cmp::min(res,min_falling_path_sum_1_dp(&matrix,m-1,i,&mut memo));
    }
    res
}

fn min_falling_path_sum_1_dp(matrix:&Vec<Vec<i32>>,i:usize,j:usize,memo:&mut Vec<Vec<i32>>)->i32{
    if i<0 || j<0 || i>=matrix.len() || j>=matrix[0].len(){
        return 99999;
    }
    if i==0{
        return matrix[i][j];
    }
    if memo[i][j]!=66666{
        return memo[i][j];
    }
    memo[i][j]=cmp::min(dp(matrix,i-1,j,memo),cmp::min(dp(matrix,i-1,j-1,memo),dp(matrix,i-1,j+1,memo)))+matrix[i][j];
    return memo[i][j];
}