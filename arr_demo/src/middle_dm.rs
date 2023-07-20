/// 918. 环形子数组的最大和
/// 给定一个长度为 n 的环形整数数组nums，返回nums的非空 子数组 的最大可能和。
/// 环形数组意味着数组的末端将会与开头相连呈环状。形式上， nums[i] 的下一个元素是 nums[(i + 1) % n] ， nums[i]的前一个元素是 nums[(i - 1 + n) % n] 。
/// 子数组 最多只能包含固定缓冲区nums中的每个元素一次。
/// 形式上，对于子数组nums[i], nums[i + 1], ..., nums[j]，不存在i <= k1, k2 <= j其中k1 % n == k2 % n。

// !! 重点
// lc 比较优秀的解
// 如果不是环，最大子数组就是解
// 如果成环，max(最大子数组，数组总和-最小子数组)
pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
    let (mut max, mut min) = (nums[0], nums[0]);
    let (mut cur_max, mut cur_min) = (0,0);
    let mut sum = 0;
    for n in nums {
        sum += n;
        cur_max = n.max(cur_max + n);
        max = max.max(cur_max);
        cur_min = n.min(cur_min + n);
        min = min.min(cur_min);
    }
    if max > 0 {
        max.max(sum - min)
    } else {
        max
    }
}