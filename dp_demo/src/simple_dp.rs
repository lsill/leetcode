use std::cmp::max;
use std::f32::{MIN, MIN_10_EXP, MIN_EXP, MIN_POSITIVE};
use std::i128::MAX;

/// 1186. 删除一次得到子数组最大和
/// 给你一个整数数组，返回它的某个 非空 子数组（连续元素）在执行一次可选的删除操作后，
/// 所能得到的最大元素总和。换句话说，你可以从原数组中选出一个子数组，并可以决定要不要从中删除一个元素（只能删一次哦），
/// （删除后）子数组中至少应当有一个元素，然后该子数组（剩下）的元素总和是所有子数组之中最大的。
/// 注意，删除一个元素后，子数组 不能为空。
/// **示例1**
/// 输入：arr = [1,-2,0,3]
/// 输出：4
/// 解释：我们可以选出 [1, -2, 0, 3]，然后删掉 -2，这样得到 [1, 0, 3]，和最大。
/// **示例2**
/// 输入：arr = [1,-2,-2,3]
/// 输出：3
/// 解释：我们直接选出 [3]，这就是最大和。
/// **示例3**
/// 输入：arr = [-1,-1,-1,-1]
/// 输出：-1
/// 解释：最后得到的子数组不能为空，所以我们不能选择 [-1] 并从中删去 -1 来得到 0。
///      我们应该直接选择 [-1]，或者选择 [-1, -1] 再从中删去一个 -1。

/// [!题解](https://leetcode.cn/problems/maximum-subarray-sum-with-one-deletion/solution/jiao-ni-yi-bu-bu-si-kao-dong-tai-gui-hua-hzz6/)

// lc 的rust题解
pub fn maximum_sum(arr: Vec<i32>) -> i32 {
    let len = arr.len();
    let mut dp_keep = vec![i32::MIN; len];
    let mut dp_delete = vec![i32::MIN; len];
    dp_keep[0] = arr[0];
    dp_delete[0] = arr[0];
    for i in 1..len {
        dp_keep[i] = arr[i].max(dp_keep[i - 1] + arr[i]);
        dp_delete[i] = arr[i].max(dp_keep[i - 1].max(dp_delete[i - 1] + arr[i]));
    }
    return dp_delete.into_iter().max().unwrap()
}

// 大佬优化版
pub fn maximum_sum_bc(arr: Vec<i32>) -> i32 {
    let mut dp0 = arr[0];
    let mut dp1 = 0;
    let mut res = arr[0];
    for val in arr[1..].iter() {
        dp1 = (dp1 + val).max(dp0);
        dp0 = dp0.max(0) + val;
        res = res.max(dp1.max(dp0));
    }
    res
}