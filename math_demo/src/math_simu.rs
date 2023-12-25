/// 2240. 买钢笔和铅笔的方案数
/// 给你一个整数 total ，表示你拥有的总钱数。同时给你两个整数 cost1 和 cost2 ，分别表示一支钢笔和一支铅笔的价格。你可以花费你部分或者全部的钱，去买任意数目的两种笔。
/// 请你返回购买钢笔和铅笔的 不同方案数目 。
// lc 自己解
pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
    let mut ans:i64 = 0;
    let mut pen:i32 = 0;
    let c1 = cost1.max(cost2);
    let c2 = cost1.min(cost2);
    while total >= c1 * pen {
        let pencil = (total - pen * c1) / c2;
        ans += pencil as i64 + 1;
        pen += 1;
    }
    ans
}

// lc 比较优秀的解
pub fn ways_to_buy_pens_pencils_1(total: i32, cost1: i32, cost2: i32) -> i64 {
    if cost1 < cost2 {
        return ways_to_buy_pens_pencils(total, cost2, cost1)
    }
    let mut res = 0_i64;
    let mut total = total;
    while total >= 0 {
        res += (total / cost2 + 1) as i64;
        total -= cost1;
    }
    res
}

/// 2651. 计算列车到站时间
/// 给你一个正整数 arrivalTime 表示列车正点到站的时间（单位：小时），另给你一个正整数 delayedTime 表示列车延误的小时数。
/// 返回列车实际到站的时间。
/// 注意，该问题中的时间采用 24 小时制。

pub fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32 {
    (arrival_time + delayed_time) % 24
}

/// 2582. 递枕头
/// n 个人站成一排，按从 1 到 n 编号。
/// 最初，排在队首的第一个人拿着一个枕头。每秒钟，拿着枕头的人会将枕头传递给队伍中的下一个人。一旦枕头到达队首或队尾，传递方向就会改变，队伍会继续沿相反方向传递枕头。
/// 例如，当枕头到达第 n 个人时，TA 会将枕头传递给第 n - 1 个人，然后传递给第 n - 2 个人，依此类推。
/// 给你两个正整数 n 和 time ，返回 time 秒后拿着枕头的人的编号。
pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
    let round = time / (n - 1);
    let step = time % (n - 1);

    if round & 1 == 1 {
        n - step
    } else {
        1 + step
    }
}

/// 2562. 找出数组的串联值
///给你一个下标从 0 开始的整数数组 nums 。
/// 现定义两个数字的 串联 是由这两个数值串联起来形成的新数字。
/// 例如，15 和 49 的串联是 1549 。
/// nums 的 串联值 最初等于 0 。执行下述操作直到 nums 变为空：
/// 如果 nums 中存在不止一个数字，分别选中 nums 中的第一个元素和最后一个元素，将二者串联得到的值加到 nums 的 串联值 上，然后从 nums 中删除第一个和最后一个元素。
/// 如果仅存在一个元素，则将该元素的值加到 nums 的串联值上，然后删除这个元素。
/// 返回执行完所有操作后 nums 的串联值。
/// 示例 1：
/// 输入：nums = [7,52,2,4]
/// 输出：596
/// 解释：在执行任一步操作前，nums 为 [7,52,2,4] ，串联值为 0 。
///  - 在第一步操作中：
/// 我们选中第一个元素 7 和最后一个元素 4 。
/// 二者的串联是 74 ，将其加到串联值上，所以串联值等于 74 。
/// 接着我们从 nums 中移除这两个元素，所以 nums 变为 [52,2] 。
///  - 在第二步操作中：
/// 我们选中第一个元素 52 和最后一个元素 2 。
/// 二者的串联是 522 ，将其加到串联值上，所以串联值等于 596 。
/// 接着我们从 nums 中移除这两个元素，所以 nums 变为空。
/// 由于串联值等于 596 ，所以答案就是 596 。
/// 示例 2：
/// 输入：nums = [5,14,13,8,12]
/// 输出：673
/// 解释：在执行任一步操作前，nums 为 [5,14,13,8,12] ，串联值为 0 。
/// - 在第一步操作中：
/// 我们选中第一个元素 5 和最后一个元素 12 。
/// 二者的串联是 512 ，将其加到串联值上，所以串联值等于 512 。
/// 接着我们从 nums 中移除这两个元素，所以 nums 变为 [14,13,8] 。
/// - 在第二步操作中：
/// 我们选中第一个元素 14 和最后一个元素 8 。
/// 二者的串联是 148 ，将其加到串联值上，所以串联值等于 660 。
/// 接着我们从 nums 中移除这两个元素，所以 nums 变为 [13] 。
/// - 在第三步操作中：
/// nums 只有一个元素，所以我们选中 13 并将其加到串联值上，所以串联值等于 673 。
/// 接着我们从 nums 中移除这个元素，所以 nums 变为空。
/// 由于串联值等于 673 ，所以答案就是 673 。

// 力扣优秀解
pub fn find_the_array_conc_val(nums: Vec<i32>) -> i64 {
    let mut ans = 0i64;
    let mut i = 0;
    let mut j = nums.len() -1;
    while i < j {
       let mut x = nums[i];
        let mut y = nums[j];
        while y != 0 {
            x *= 10;
            y /= 10;
        }
        ans += (x + nums[j]) as i64;
        i += 1;
        j -= 1;
    }
    if i == j {
        ans += nums[i] as i64;
    }
    ans
}


/// 2652. 倍数求和
/// 给你一个正整数 n ，请你计算在 [1，n] 范围内能被 3、5、7 整除的所有整数之和。
/// 返回一个整数，用于表示给定范围内所有满足约束条件的数字之和。
/// 示例 1：
/// 输入：n = 7
/// 输出：21
/// 解释：在 [1, 7] 范围内能被 3、5、7 整除的所有整数分别是 3、5、6、7 。数字之和为 21 。
/// 示例 2：
/// 输入：n = 10
/// 输出：40
/// 解释：在 [1, 10] 范围内能被 3、5、7 整除的所有整数分别是 3、5、6、7、9、10 。数字之和为 40 。
/// 示例 3：
/// 输入：n = 9
/// 输出：30
/// 解释：在 [1, 9] 范围内能被 3、5、7 整除的所有整数分别是 3、5、6、7、9 。数字之和为 30 。
// 自己做的垃圾
pub fn sum_of_multiples(n: i32) -> i32 {
    let mut ans:i32 = 0;
    for i in 1..n+1 {
        if i % 3 == 0 || i % 5 == 0 || i % 7 == 0 {
            ans += i;
        }
    }
    ans
}

// 力扣优秀数学解 公式
// 在[1,n]中，m的倍数有k = n/m的底，即m,2m,3m...km
// 等差数据求和公式 s(m) = k(k+1)/2*m
pub fn sum_of_multiples_1(n: i32) -> i32 {
    let s = |m:i32| n/m * (n/m + 1) / 2 * m;
    s(3) + s(5) + s(7) - s(15) - s(21) - s(35) + s(105)
}

/// 1276. 不浪费原料的汉堡制作方案
/// 圣诞活动预热开始啦，汉堡店推出了全新的汉堡套餐。为了避免浪费原料，请你帮他们制定合适的制作计划。
/// 给你两个整数 tomatoSlices 和 cheeseSlices，分别表示番茄片和奶酪片的数目。不同汉堡的原料搭配如下：
/// 巨无霸汉堡：4 片番茄和 1 片奶酪
/// 小皇堡：2 片番茄和 1 片奶酪
/// 请你以 [total_jumbo, total_small]（[巨无霸汉堡总数，小皇堡总数]）的格式返回恰当的制作方案，使得剩下的番茄片 tomatoSlices 和奶酪片 cheeseSlices 的数量都是 0。
/// 如果无法使剩下的番茄片 tomatoSlices 和奶酪片 cheeseSlices 的数量为 0，就请返回 []。
/// 示例 1：
/// 输入：tomatoSlices = 16, cheeseSlices = 7
/// 输出：[1,6]
/// 解释：制作 1 个巨无霸汉堡和 6 个小皇堡需要 4*1 + 2*6 = 16 片番茄和 1 + 6 = 7 片奶酪。不会剩下原料。
/// 示例 2：
/// 输入：tomatoSlices = 17, cheeseSlices = 4
/// 输出：[]
/// 解释：只制作小皇堡和巨无霸汉堡无法用光全部原料。
/// 示例 3：
/// 输入：tomatoSlices = 4, cheeseSlices = 17
/// 输出：[]
/// 解释：制作 1 个巨无霸汉堡会剩下 16 片奶酪，制作 2 个小皇堡会剩下 15 片奶酪。
/// 示例 4：
/// 输入：tomatoSlices = 0, cheeseSlices = 0
/// 输出：[0,0]
/// 示例 5：
/// 输入：tomatoSlices = 2, cheeseSlices = 1
/// 输出：[0,1]
/// 提示：
/// 0 <= tomatoSlices <= 10^7
/// 0 <= cheeseSlices <= 10^7

// 自己做的
pub fn num_of_burgers_1(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
    let mut ans:Vec<i32> = Vec::new();
    if tomato_slices & 1 == 1 {
        return ans;
    }
    if cheese_slices == 0 {
        return if tomato_slices == 0 {
            vec![0, 0]
        } else {
            ans
        }
    }
    let x = tomato_slices / 2 - cheese_slices;
    if x < 0 || cheese_slices - x < 0{
        return ans;
    }
    return vec![x, cheese_slices - x]
}

// 写的比较优雅的
pub fn num_of_burgers(tomato_slices: i32, cheese_slices: i32) -> Vec<i32> {
    if tomato_slices < cheese_slices * 2
        || tomato_slices > cheese_slices * 4
        || tomato_slices & 1 == 1 {
        return vec![]
    } else {
        let big = (tomato_slices - cheese_slices * 2) /2;
        return vec![big, cheese_slices - big]
    }
}