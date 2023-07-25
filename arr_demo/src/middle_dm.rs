use std::cmp::Ordering;

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

/// 2208. 将数组和减半的最少操作次数
/// 给你一个正整数数组nums。每一次操作中，你可以从nums中选择 任意一个数并将它减小到 恰好一半。（注意，在后续操作中你可以对减半过的数继续执行操作）
/// 请你返回将 nums数组和 至少减少一半的 最少操作数。

// lc贪心+优先队列 rust合适的写法
pub fn halve_array(mut nums: Vec<i32>) -> i32 {
    #[derive(PartialEq, PartialOrd, Clone, Copy)]
    struct NonNanDouble(f64);
    impl Eq for NonNanDouble {}
    impl Ord for NonNanDouble {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.partial_cmp(other).unwrap()
        }
    }
    impl From<f64> for NonNanDouble {
        fn from(value: f64) -> Self {
            NonNanDouble(value)
        }
    }
    impl NonNanDouble {
        fn new(value: f64) -> Self {
            if value.is_nan() {
                panic!("Must build from an `NonNaN` value, but received `NaN`!")
            }
            Self(value)
        }
    }
    use std::collections::BinaryHeap;
    let sum = NonNanDouble::new(nums.iter().map(|num| *num as f64).sum::<f64>());
    let mut used = NonNanDouble::new(0.0);
    let mut steps = 0;
    let mut pq = nums
        .into_iter()
        .map(|num| NonNanDouble::new(num as f64))
        .collect::<BinaryHeap<_>>();
    while used.0 < sum.0 / 2.0 {
        let max = pq.pop().unwrap();
        let half = max.0 as f64 / 2.0;
        used.0 += half;
        pq.push(NonNanDouble::new(half));
        steps += 1;
    }
    steps
}