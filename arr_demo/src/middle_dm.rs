use std::arch::aarch64::{vaba_s8, veor_s8};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter::from_fn;
use std::panic::resume_unwind;
use std::process::abort;

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

/// 2682. 找出转圈游戏输家
/// n 个朋友在玩游戏。这些朋友坐成一个圈，按 顺时针方向 从 1 到 n 编号。
/// 从第 i 个朋友的位置开始顺时针移动 1 步会到达第 (i + 1) 个朋友的位置（1 <= i < n），而从第 n 个朋友的位置开始顺时针移动 1 步会回到第 1 个朋友的位置。
/// 游戏规则如下：
/// 第 1 个朋友接球。
/// 接着，第 1 个朋友将球传给距离他顺时针方向 k 步的朋友。
/// 然后，接球的朋友应该把球传给距离他顺时针方向 2 * k 步的朋友。
/// 接着，接球的朋友应该把球传给距离他顺时针方向 3 * k 步的朋友，以此类推。
/// 换句话说，在第 i 轮中持有球的那位朋友需要将球传递给距离他顺时针方向 i * k 步的朋友。
/// 当某个朋友第 2 次接到球时，游戏结束。
/// 在整场游戏中没有接到过球的朋友是 输家 。
/// 给你参与游戏的朋友数量 n 和一个整数 k ，请按升序排列返回包含所有输家编号的数组 answer 作为答案。
// 自己做的
pub fn circular_game_losers(n: i32, k: i32) -> Vec<i32> {
    let mut kv:HashMap<i32,i32> = HashMap::new();
    kv.insert(1, 1);
    let mut next = 1;
    let mut step  = k;
    loop {
        let mut tmp = (next + step) % n;
        if tmp == 0 {
            tmp = n;
        }
        next = tmp;
        step += k;
        if kv.contains_key(&tmp) {
            break;
        }
        kv.insert(tmp, 1);
    }
    let mut ans = Vec::new();
    for i in 1..n+1 {
        if kv.contains_key(&i) {
            continue;
        }
        ans.push(i);
    }
    ans
}

// lc 好的符合rust的题解
pub fn circular_game_losers_0(n: i32, k: i32) -> Vec<i32> {
    let k = k as usize;
    let n = n as usize;
    let mut i = 0;
    let mut j = 1;
    let mut v = vec![false; n];
    loop {
        if v[i] == true {
            break;
        } else {
            v[i] = true;
        }
        i+=j*k;
        i %= n;
        j += 1;
    }
    v.into_iter().enumerate().filter(|(i, f)|!*f).map(|(i, f)| (i+1) as i32).collect()
}

/// 849. 到最近的人的最大距离
/// 给你一个数组 seats 表示一排座位，其中 seats[i] = 1 代表有人坐在第 i 个座位上，seats[i] = 0 代表座位 i 上是空的（下标从 0 开始）。
/// 至少有一个空座位，且至少有一人已经坐在座位上。
/// 亚历克斯希望坐在一个能够使他与离他最近的人之间的距离达到最大化的座位上。
/// 返回他到离他最近的人的最大距离。

// lc 双百解 很符合rust写法的解
pub fn max_dist_to_closest(seats: Vec<i32>) -> i32 {
    let n = seats.len() as i32;
    let ans = seats.iter()
        .enumerate()
        .fold(
            (0,n,-1),
            |(max, first, last), (seat, man)| {
                let s = seat as i32;
                if *man == 1 { (max, s.min(first), s)} else { (max.max(s - last), first, last) }
            },
        );
    ans.1.max(n-1 - ans.2).max((ans.0 + 1) / 2)
}
/// 1333. 餐厅过滤器
/// 给你一个餐馆信息数组 restaurants，其中  restaurants[i] = [idi, ratingi, veganFriendlyi, pricei, distancei]。你必须使用以下三个过滤器来过滤这些餐馆信息。
/// 其中素食者友好过滤器 veganFriendly 的值可以为 true 或者 false，
/// 如果为 true 就意味着你应该只包括 veganFriendlyi 为 true 的餐馆，为 false 则意味着可以包括任何餐馆。
/// 此外，我们还有最大价格 maxPrice 和最大距离 maxDistance 两个过滤器，它们分别考虑餐厅的价格因素和距离因素的最大值。
/// 过滤后返回餐馆的 id，按照 rating 从高到低排序。如果 rating 相同，那么按 id 从高到低排序。
/// 简单起见， veganFriendlyi 和 veganFriendly 为 true 时取值为 1，为 false 时，取值为 0 。
pub fn filter_restaurants(restaurants:Vec<Vec<i32>>, vegan_friendly: i32, max_price: i32, max_distance: i32) -> Vec<i32> {
    let mut restaurants = restaurants
        .iter()
        .filter(|res| res[2] >= vegan_friendly && res[3] <= max_price && res[4] <= max_distance)
        .collect::<Vec<_>>();
    restaurants.sort_unstable_by_key(|r| (-r[1], -r[0]));
    restaurants.iter().map(|r| r[0]).collect()
}