/// 1253. 重构 2 行二进制矩阵
/// 给你一个**2**行 **n** 列的**二进制**数组：
/// 矩阵是一个二进制矩阵，这意味着矩阵中的每个元素不是0就是1。
/// 第 0 行的元素之和为upper。
/// 第 1 行的元素之和为 lower。
/// 第 i 列（从 0 开始编号）的元素之和为colsum[i]，colsum是一个长度为n的整数数组。
/// 你需要利用upper，lower和colsum来重构这个矩阵，并以二维整数数组的形式返回它。
/// 如果有多个不同的答案，那么任意一个都可以通过本题。
/// 如果不存在符合要求的答案，就请返回一个空的二维数组。

/// 示例 1：
///
///  输入：upper = 2, lower = 1, colsum = [1,1,1]
///  输出：[[1,1,0],[0,0,1]]
///  解释：[[1,0,1],[0,1,0]] 和 [[0,1,1],[1,0,0]] 也是正确答案。
///  示例 2：
///
/// 输入：upper = 2, lower = 3, colsum = [2,2,1,1]
/// 输出：[]
/// 示例 3：
///
/// 输入：upper = 5, lower = 5, colsum = [2,1,2,0,1,0,1,2,0,1]
/// 输出：[[1,1,1,0,1,0,0,1,0,0],[1,0,1,0,0,0,1,1,0,1]]

// 第一次自己做的提交
pub fn reconstruct_matrix_first(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
    let n = colsum.len();
    let mut t_upper = upper;
    let mut t_lower = lower;
    let mut res: Vec<Vec<i32>> = vec![vec![0; n]; 2];
    for (k, v) in colsum.iter().enumerate() {
        if *v == 2 {
            if t_upper <= 0 || t_lower <= 0 {
                return vec![];
            }
            res[0][k] = 1;
            res[1][k] = 1;
            t_upper -= 1;
            t_lower -= 1;
        }
    }
    for (k, v) in colsum.iter().enumerate() {
        if *v != 1 {
            continue;
        }
        if t_upper > 0 {
            res[0][k] = 1;
            t_upper -= 1;
        } else if t_lower > 0 {
            res[1][k] = 1;
            t_lower -= 1;
        } else {
            return vec![];
        }
    }
    if t_upper > 0 || t_lower > 0 {
        return vec![];
    }
    res
}

// lc最优解
pub fn reconstruct_matrix_try(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
    let s:i32 = colsum.iter().sum();
    if s != upper + lower {
        return vec![];
    }
    let c1 = colsum.iter().filter(|&a| *a == 2).count() as i32;
    if c1 > lower || c1 > upper {
        return vec![];
    }

    let n = colsum.len();
    let mut res: Vec<Vec<i32>> = vec![vec![0; n]; 2];
    let mut diff = upper - lower;
    for i in 0..n {
        if colsum[i] == 2 {
            ans[0][i] = 1;
            ans[1][i] = 1;
        } else if colsum[i] == 1 {
            if diff < 0 {
                diff += 1;
                ans[1][i] = 1;
            } else {
                diff -= 1;
                ans[0][i] = 1;
            }
        }
    }
    res
}