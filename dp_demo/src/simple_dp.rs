use std::cmp::max;

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

/// 1911. 最大子序列交替和
/// 一个下标从0开始的数组的交替和定义为 偶数下标处元素之和减去 奇数下标处元素之和。
/// 比方说，数组[4,2,5,3]的交替和为(4 + 5) - (2 + 3) = 4。
/// 给你一个数组nums，请你返回nums中任意子序列的最大交替和（子序列的下标 重新从 0 开始编号）。
/// 一个数组的 子序列是从原数组中删除一些元素后（也可能一个也不删除）剩余元素不改变顺序组成的数组。
/// 比方说，[2,7,4]是[4,2,3,7,2,1,4]的一个子序列（加粗元素），但是[2,4,2] 不是。
/// [!链接](https://leetcode.cn/problems/maximum-alternating-subsequence-sum/solution/)


// lc 评论解
pub fn max_alternating_sum(nums: Vec<i32>) -> i64 {
    let mut ori = 0;
    let mut res = nums[0] as i64;

    for i in 0..nums.len() {
        res = std::cmp::max(res, ori + nums[i] as i64);
        ori = std::cmp::max(ori, res - nums[i] as i64);
    }
    res
}
// lc 最优解
pub fn max_alternating_sum_1(nums: Vec<i32>) -> i64 {
    let (mut odd, mut even) = (0, nums[0] as i64);

    for i in 1..nums.len() {
        odd = odd.max(even - nums[i] as i64);
        even = even.max(odd + nums[i] as i64);
    }
    even
}

/// 2735. 收集巧克力
/// 给你一个长度为 n 、下标从 0 开始的整数数组 nums ，表示收集不同巧克力的成本。每个巧克力都对应一个不同的类型，最初，位于下标 i 的巧克力就对应第 i 个类型。
/// 在一步操作中，你可以用成本 x 执行下述行为：
///
/// 同时修改所有巧克力的类型，将巧克力的类型 ith 修改为类型 ((i + 1) mod n)th。
/// 假设你可以执行任意次操作，请返回收集所有类型巧克力所需的最小成本。
/// 示例 1：
/// 输入：nums = [20,1,15], x = 5
/// 输出：13
/// 解释：最开始，巧克力的类型分别是 [0,1,2] 。我们可以用成本 1 购买第 1 个类型的巧克力。
/// 接着，我们用成本 5 执行一次操作，巧克力的类型变更为 [1,2,0] 。我们可以用成本 1 购买第 2 个类型的巧克力。
/// 然后，我们用成本 5 执行一次操作，巧克力的类型变更为 [2,0,1] 。我们可以用成本 1 购买第 0 个类型的巧克力。
/// 因此，收集所有类型的巧克力需要的总成本是 (1 + 5 + 1 + 5 + 1) = 13 。可以证明这是一种最优方案。
/// 示例 2：
/// 输入：nums = [1,2,3], x = 4
/// 输出：6
/// 解释：我们将会按最初的成本收集全部三个类型的巧克力，而不需执行任何操作。因此，收集所有类型的巧克力需要的总成本是 1 + 2 + 3 = 6 。
/// 提示：
/// 1 <= nums.length <= 1000
/// 1 <= nums[i] <= 109
/// 1 <= x <= 109
/// ![](https://raw.githubusercontent.com/lsill/gitLink/main/document/photo/leetcode/dp/lc2735.jpg)
// 抄的解1
pub fn min_cost_1(nums: Vec<i32>, x: i32) -> i64 {
    let n = nums.len();
    let mut f = vec![vec![0;n]; n];
    for i in 0..n {
       f[i][0] = nums[i];
        for j in 1..n {
            f[i][j] = f[i][j-1].min(nums[(i+j) % n]);
        }
    }
    let mut ans = i64::MAX;
    for j in 0..n {
        let mut cost = (x as i64) * (j as i64);
        for i in 0..n {
            cost += f[i][j] as i64;
        }
        ans = ans.min(cost);
    }
    ans
}

// 符合rust的解
pub fn min_cost(nums: Vec<i32>, x: i32) -> i64 {
    let n = nums.len();
    let mut f = nums.clone();
    let mut ans: i64 = nums.iter().fold(0, |acc, &cur| acc + cur as i64);

    for k in 1..n {
        for i in 0..n {
            f[i] = f[i].min(nums[(i + k) % n]);
        }
        let sum = f.iter().fold(0, |acc, &cur| acc + cur as i64);
        ans = ans.min((k as i64 * x as i64 + sum) as i64);
    }
    ans
}