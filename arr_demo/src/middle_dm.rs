use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};

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

/// 822. 翻转卡片游戏
/// 在桌子上有 N 张卡片，每张卡片的正面和背面都写着一个正数（正面与背面上的数有可能不一样）。
/// 我们可以先翻转任意张卡片，然后选择其中一张卡片。
/// 如果选中的那张卡片背面的数字 X 与任意一张卡片的正面的数字都不同，那么这个数字是我们想要的数字。
/// 哪个数是这些想要的数字中最小的数（找到这些数中的最小值）呢？如果没有一个数字符合要求的，输出 0。
/// 其中, fronts[i] 和 backs[i] 分别代表第 i 张卡片的正面和背面的数字。
/// 如果我们通过翻转卡片来交换正面与背面上的数，那么当初在正面的数就变成背面的数，背面的数就变成正面的数。

// lc HastSet解法
pub fn flipgame(fronts: Vec<i32>, backs: Vec<i32>) -> i32 {
    let mut good = HashSet::new();
    let mut ban = HashSet::new();
    for i in 0..backs.len() {
        if fronts[i] == backs[i] {
            good.remove(&backs[i]);
            ban.insert(&backs[i]);
        } else {
            if !ban.contains(&fronts[i]) {
                good.insert(fronts[i]);
            }
            if !ban.contains(&backs[i]) {
                good.insert(backs[i]);
            }
        }
    }
    good.into_iter().min().unwrap_or(0)
}