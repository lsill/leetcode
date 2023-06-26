/// 2485. 找出中枢整数
/// 1 和 x 之间的所有元素之和等于 x 和 n 之间所有元素之和。
/// 返回中枢整数 x 。如果不存在中枢整数，则返回 -1 。题目保证对于给定的输入，至多存在一个中枢整数。
/// 示例 1：
/// 输入：n = 8
/// 输出：6
/// 解释：6 是中枢整数，因为 1 + 2 + 3 + 4 + 5 + 6 = 6 + 7 + 8 = 21 。
/// 示例 2：
/// 输入：n = 1
/// 输出：1
/// 解释：1 是中枢整数，因为 1 = 1 。
/// 示例 3：
/// 输入：n = 4
/// 输出：-1
/// 解释：可以证明不存在满足题目要求的整数。
pub fn pivot_integer(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }
    let sum = n * (1+n) /2;
    let mut tmp_sum = 0;
    for i in 1..n+1 {
        if tmp_sum + i == sum - tmp_sum {
            return i;
        }
        tmp_sum += i;
    }
    -1
}

// 力扣最优解
// 等差数列求和 sum(x,y) = (x+y)*(y-x+1)/2
// 等价于sum(1,x) = sum(x,n)
// x = sqrt((n^2 + n)/ 2)
pub fn pivot_integer_lc(n: i32) -> i32 {
    let x = n * (n + 1) / 2;
    let y = (x as f64).sqrt() as i32;
    if y * y == x {
        return y as i32;
    }
    -1
}