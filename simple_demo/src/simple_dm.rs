/// 2544. 交替数字和
/// 给你一个正整数 n 。n 中的每一位数字都会按下述规则分配一个符号：
/// - 最高有效位 上的数字分配到 正 号。
/// - 剩余每位上数字的符号都与其相邻数字相反。
/// 返回所有数字及其对应符号的和。

/// 示例1
/// 输入：n = 521
/// 输出：4
/// 解释：(+5) + (-2) + (+1) = 4

/// 示例2
/// 输入：n = 886996
/// 输出：0
/// 解释：(+8) + (-8) + (+6) + (-9) + (+9) + (-6) = 0

// 自己做的，又臭又长
pub fn alternate_digit_sum(n: i32) -> i32 {
    let mut sum = 0;
    let mut nn = n;
    let mut nums:Vec<i32> = vec![];
    let mut index = 0;
    while nn > 0 {
        let a = nn % 10;
        nums.push(a);
        nn = nn / 10;
        index +=  1;
    }
    let mut odd = false;
    if index & 1 == 1 {
        odd = true;
    }
    for i in 0..nums.len() {
        if odd {
            if i & 1 == 1 {
                sum -= nums[i];
            } else {
                sum += nums[i];
            }
        } else {
            if i & 1 == 1 {
                sum += nums[i];
            } else {
                sum -= nums[i];
            }
        }
    }
    sum
}

// lc最优解
pub fn alternate_digit_sum_0(n: i32) -> i32{
    n.to_string()
        .bytes()
        .enumerate()
        .map(|(idx, b)| {
            if idx % 2 == 0 {
                (b - b'0') as i32
            } else {
                -((b - b'0') as i32)
            }
        })
        .sum()
}