/// 2679. 矩阵中的和
/// 给你一个下标从 0开始的二维整数数组nums。一开始你的分数为0。你需要执行以下操作直到矩阵变为空：
/// 矩阵中每一行选取最大的一个数，并删除它。如果一行中有多个最大的数，选择任意一个并删除。
/// 在步骤 1 删除的所有数字中找到最大的一个数字，将它添加到你的 分数中。
/// 请你返回最后的 分数
// 自己做的
pub fn matrix_sum(nums: Vec<Vec<i32>>) -> i32 {
    let mut cache = nums;
    let n = cache.len();
    let mut ans = 0;
    for i in 0..n {
        cache[i].sort();
    }

    let m = cache[0].len();
    for i in 0..m {
        let mut max = 0;
        // 可以写成res += nums.iter().map(|x| x[col]).max().unwrap();
        for j in 0..n {
            max = max.max(cache[j][i]);
        }
        ans += max;
    }
    ans
}
// lc 比较优秀的解
pub fn matrix_sum_lc(mut nums: Vec<Vec<i32>>) -> i32 {
    for list in nums.iter_mut() {
        list.sort();
    }
    let mut res :i32 = 0;
    let (n, m) = (nums.len(), nums[0].len());
    for col in 0..m {
        res += nums.iter().map(|x| x[col]).max().unwrap();
    }
    res
}
// lc 写的很优雅的解
pub fn matrix_sum_lc_1(mut nums: Vec<Vec<i32>>) -> i32 {
    for row in nums.iter_mut() {
        row.sort();
    }
    (0..nums[0].len()).fold(0, |mut a,c|{a+=nums.iter().map(|x|x[c]).max().unwrap();a})
}