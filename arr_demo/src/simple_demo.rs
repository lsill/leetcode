use std::os::unix::raw::gid_t;

/// 1253. 重构 2 行二进制矩阵
/// 给你一个**2**行 **n** 列的**二进制**数组：
/// 矩阵是一个二进制矩阵，这意味着矩阵中的每个元素不是0就是1。
/// 第 0 行的元素之和为upper。
/// 第 1 行的元素之和为 lower。
/// 第 i 列（从 0 开始编号）的元素之和为colsum[i]，colsum是一个长度为n的整数数组。
/// 你需要利用upper，lower和colsum来重构这个矩阵，并以二维整数数组的形式返回它。
/// 如果有多个不同的答案，那么任意一个都可以通过本题。
/// 如果不存在符合要求的答案，就请返回一个空的二维数组。

/// 示例 1：
///
///  输入：upper = 2, lower = 1, colsum = [1,1,1]
///  输出：[[1,1,0],[0,0,1]]
///  解释：[[1,0,1],[0,1,0]] 和 [[0,1,1],[1,0,0]] 也是正确答案。
///  示例 2：
///
/// 输入：upper = 2, lower = 3, colsum = [2,2,1,1]
/// 输出：[]
/// 示例 3：
///
/// 输入：upper = 5, lower = 5, colsum = [2,1,2,0,1,0,1,2,0,1]
/// 输出：[[1,1,1,0,1,0,0,1,0,0],[1,0,1,0,0,0,1,1,0,1]]

// 第一次自己做的提交
pub fn reconstruct_matrix_first(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
    let n = colsum.len();
    let mut t_upper = upper;
    let mut t_lower = lower;
    let mut res: Vec<Vec<i32>> = vec![vec![0; n]; 2];
    for (k, v) in colsum.iter().enumerate() {
        if *v == 2 {
            if t_upper <= 0 || t_lower <= 0 {
                return vec![];
            }
            res[0][k] = 1;
            res[1][k] = 1;
            t_upper -= 1;
            t_lower -= 1;
        }
    }
    for (k, v) in colsum.iter().enumerate() {
        if *v != 1 {
            continue;
        }
        if t_upper > 0 {
            res[0][k] = 1;
            t_upper -= 1;
        } else if t_lower > 0 {
            res[1][k] = 1;
            t_lower -= 1;
        } else {
            return vec![];
        }
    }
    if t_upper > 0 || t_lower > 0 {
        return vec![];
    }
    res
}

// lc最优解
pub fn reconstruct_matrix_try(upper: i32, lower: i32, colsum: Vec<i32>) -> Vec<Vec<i32>> {
    let s:i32 = colsum.iter().sum();
    if s != upper + lower {
        return vec![];
    }
    let c1 = colsum.iter().filter(|&a| *a == 2).count() as i32;
    if c1 > lower || c1 > upper {
        return vec![];
    }

    let n = colsum.len();
    let mut res: Vec<Vec<i32>> = vec![vec![0; n]; 2];
    let mut diff = upper - lower;
    for i in 0..n {
        if colsum[i] == 2 {
            res[0][i] = 1;
            res[1][i] = 1;
        } else if colsum[i] == 1 {
            if diff < 0 {
                diff += 1;
                res[1][i] = 1;
            } else {
                diff -= 1;
                res[0][i] = 1;
            }
        }
    }
    res
}

/// 1. 两数之和
/// 给定一个整数数组 nums和一个整数目标值 target，请你在该数组中找出 和为目标值 target 的那两个整数，并返回它们的数组下标。
/// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素在答案里不能重复出现。
/// 你可以按任意顺序返回答案。
///
/// 示例 1：
/// 输入：nums = [2,7,11,15], target = 9
/// 输出：[0,1]
/// 解释：因为 nums[0] + nums[1] == 9 ，返回 [0, 1] 。
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut kv = std::collections::HashMap::new();
    nums.into_iter().
        enumerate().
        find_map(|(k,v)| match kv.get(&(target - v)){
            Some(j) => Some(vec![k as i32, *j]),
            None=>{
                kv.insert(v, k as i32);
                None
            }
        }).unwrap()
}

/// 2178. 拆分成最多数目的正偶数之和
/// 给你一个整数finalSum。请你将它拆分成若干个互不相同 的正偶数之和，且拆分出来的正偶数数目最多。
/// 比方说，给你finalSum = 12，那么这些拆分是符合要求 的（互不相同的正偶数且和为finalSum）
/// ：(2 + 10)，(2 + 4 + 6)和(4 + 8)。它们中，(2 + 4 + 6)包含最多数目的整数。
/// 注意finalSum不能拆分成(2 + 2 + 4 + 4)，因为拆分出来的整数必须互不相同。
/// 请你返回一个整数数组，表示将整数拆分成 最多 数目的正偶数数组。
/// 如果没有办法将finalSum进行拆分，请你返回一个空数组。你可以按 任意顺序返回这些整数。
pub fn maximum_even_split(final_sum: i64) -> Vec<i64> {
    if final_sum & 1 == 1 {
        return vec![]
    }
    let mut tmp = final_sum;
    let mut ans = vec![];
    let mut t = 2;
    while t * 2 < tmp {
        ans.push(t);
        tmp -= t;
        t += 2;
    }
    ans.push(tmp);
    ans
}


/// 2500. 删除每行中的最大值
/// 给你一个 m x n 大小的矩阵 grid ，由若干正整数组成。
/// 执行下述操作，直到 grid 变为空矩阵：
/// 从每一行删除值最大的元素。如果存在多个这样的值，删除其中任何一个。
/// 将删除元素中的最大值与答案相加。
/// 注意 每执行一次操作，矩阵中列的数据就会减 1 。
/// 返回执行上述操作后的答案。
pub fn delete_greatest_value(grid: Vec<Vec<i32>>) -> i32 {
    let mut grid = grid;
    for i in 0..grid.len() {
        grid[i].sort();
    }

    let mut ans = 0;
    for j in 0..grid[0].len() {
        let mut mx = 0;

        for i in 0..grid.len() {
            if grid[i][j] > mx {
                mx = grid[i][j];
            }
        }

        ans += mx;
    }
    ans
}

pub fn delete_greatest_value_0(mut grid: Vec<Vec<i32>>) -> i32 {
    let mut res=0;
    for i in 0..grid.len() {
        grid[i].sort_unstable();

    }
    let mut n=grid[0].len();

    for j in (0..n).rev() {
        let mut v=vec![];
        for i in 0..grid.len() {
            v.push(grid[i][j]);

        }
        res+=v.iter().max().unwrap();

    }
    res
}
/// 2605. 从两个数字数组里生成最小数字
/// 给你两个只包含 1 到 9 之间数字的数组 nums1 和 nums2 ，每个数组中的元素 互不相同 ，请你返回 最小 的数字，两个数组都 至少 包含这个数字的某个数位。
/// 示例 1：
/// 输入：nums1 = [4,1,3], nums2 = [5,7]
/// 输出：15
/// 解释：数字 15 的数位 1 在 nums1 中出现，数位 5 在 nums2 中出现。15 是我们能得到的最小数字。
/// 示例 2：
/// 输入：nums1 = [3,5,2,6], nums2 = [3,1,7]
/// 输出：3
/// 解释：数字 3 的数位 3 在两个数组中都出现了。

// lc 第一种解法 时间复杂度很好
pub fn min_number(nums1: Vec<i32>,nums2: Vec<i32>) -> i32 {
    let mut bucket = [0;10];
    nums1.iter().for_each(|&i|bucket[i as usize] += 1);
    nums2.iter().for_each(|&i|bucket[i as usize] += 2);
    let mut num1 = 10;
    let mut num2 = 20;
    for (i, &count) in bucket.iter().enumerate() {
        match count {
            3 => return i as i32,
            2 => num1 = num1.min(i as i32),
            1 => num2 = num2.min(i as i32),
            _ => continue,
        }
    }
    num1.min(num2) * 10 + num1.max(num2)
}

// lc 另外一种写法，不太好
pub fn min_number_1(nums1: Vec<i32>,nums2: Vec<i32>) -> i32 {
    let d = nums1.iter().filter(|n| nums2.contains(n)).min();
    return if d.is_none() {
        let a = nums1.iter().min().unwrap();
        let b = nums2.iter().min().unwrap();
        if a < b {
            a * 10 + b
        } else {
            b * 10 + a
        }
    } else {
        *d.unwrap()
    }
}

/// 260. 只出现一次的数字 III
/// 给你一个整数数组 nums，其中恰好有两个元素只出现一次，其余所有元素均出现两次。 找出只出现一次的那两个元素。你可以按 任意顺序 返回答案。
/// 你必须设计并实现线性时间复杂度的算法且仅使用常量额外空间来解决此问题。
// 自己做的
pub fn single_number(mut nums: Vec<i32>) -> Vec<i32> {
    let mut ans:Vec<i32> = vec![];
    nums.sort();
    let n = nums.len();
    let mut one = nums[0];
    let mut two = nums[1];
    if one != two {
        ans.push(one)
    }
    for i in 2..n {
       if one != two && two != nums[i] {
           ans.push(two);
       }
       one = two;
       two = nums[i];
    }
    if nums[n-2] != nums[n-1] {
        ans.push(nums[n-1]);
    }
    ans
}

// 力扣符合rust的写法
pub fn single_number_1(mut nums: Vec<i32>) -> Vec<i32> {
    let xor_all = nums.iter().fold(0, |xor, &x| xor ^ x);
    let tz = xor_all.trailing_zeros();
    let mut ans = vec![0, 0];
    for &x in &nums {
        ans[x as usize >> tz & 1] ^= x;
    }
    ans
}

/// 2525. 根据规则将箱子分类
/// 给你四个整数 length ，width ，height 和 mass ，分别表示一个箱子的三个维度和质量，请你返回一个表示箱子 类别 的字符串。
/// 如果满足以下条件，那么箱子是 "Bulky" 的：
/// 箱子 至少有一个 维度大于等于 104 。
/// 或者箱子的 体积 大于等于 109 。
/// 如果箱子的质量大于等于 100 ，那么箱子是 "Heavy" 的。
/// 如果箱子同时是 "Bulky" 和 "Heavy" ，那么返回类别为 "Both" 。
/// 如果箱子既不是 "Bulky" ，也不是 "Heavy" ，那么返回类别为 "Neither" 。
/// 如果箱子是 "Bulky" 但不是 "Heavy" ，那么返回类别为 "Bulky" 。
/// 如果箱子是 "Heavy" 但不是 "Bulky" ，那么返回类别为 "Heavy" 。
/// 注意，箱子的体积等于箱子的长度、宽度和高度的乘积。
///
// 自己做的
pub fn categorize_box(length: i32, width: i32, height: i32, mass: i32) -> String {
    let bFunBulky = |l:i32,w:i32,h:i32|->bool{
        if l >= 10000 || w >= 10000 || h >= 10000 {
            return true
        }
        l as i64* h as i64 *w as i64 >= 1000000000
    };
    let bFunHeavy = |mass:i32|->bool {
        mass >= 100
    };
    let bBulky = bFunBulky(length,width,height);
    let bHeavy = bFunHeavy(mass);
    if bBulky && bHeavy {
        return "Both".to_string()
    }
    if bBulky {
        return "Bulky".to_string()
    }
    if bHeavy {
        return "Heavy".to_string()
    }
    "Neither".to_string()
}

// 写的比较巧的
pub fn categorize_box_1(length: i32, width: i32, height: i32, mass: i32) -> String {
    let x = length >= 10000 || width >= 10000 || height >= 10000 || length as i64 * width as i64 * height as i64 >= 100000000;
    let y = mass >= 100;
    if x && y {
        return "Both".to_string();
    }
    if x {
        return "Bulky".to_string();
    }
    if y {
        return "Heavy".to_string();
    }
    "Neither".to_string()
}


/// 2300. 咒语和药水的成功对数
/// 给你两个正整数数组 spells 和 potions ，长度分别为 n 和 m ，其中 spells[i] 表示第 i 个咒语的能量强度，potions[j] 表示第 j 瓶药水的能量强度。
/// 同时给你一个整数 success 。一个咒语和药水的能量强度 相乘 如果 大于等于 success ，那么它们视为一对 成功 的组合。
/// 请你返回一个长度为 n 的整数数组 pairs，其中 pairs[i] 是能跟第 i 个咒语成功组合的 药水 数目。
/// 示例 1：
/// 输入：spells = [5,1,3], potions = [1,2,3,4,5], success = 7
/// 输出：[4,0,3]
/// 解释：
/// - 第 0 个咒语：5 * [1,2,3,4,5] = [5,10,15,20,25] 。总共 4 个成功组合。
/// - 第 1 个咒语：1 * [1,2,3,4,5] = [1,2,3,4,5] 。总共 0 个成功组合。
/// - 第 2 个咒语：3 * [1,2,3,4,5] = [3,6,9,12,15] 。总共 3 个成功组合。
/// 所以返回 [4,0,3] 。
/// 示例 2：
/// 输入：spells = [3,1,2], potions = [8,5,8], success = 16
/// 输出：[2,0,2]
/// 解释：
/// - 第 0 个咒语：3 * [8,5,8] = [24,15,24] 。总共 2 个成功组合。
/// - 第 1 个咒语：1 * [8,5,8] = [8,5,8] 。总共 0 个成功组合。
/// - 第 2 个咒语：2 * [8,5,8] = [16,10,16] 。总共 2 个成功组合。
/// 所以返回 [2,0,2] 。
/// 提示：
/// n == spells.length
/// m == potions.length
/// 1 <= n, m <= 105
/// 1 <= spells[i], potions[i] <= 105
/// 1 <= success <= 1010
// 自己做的 超时了
pub fn successful_pairs(spells: Vec<i32>, potions: Vec<i32>, success: i64) -> Vec<i32> {
    let mut ans:Vec<i32> = Vec::new();
    for i in 0..spells.len() {
        let mut sum = 0;
        for j in 0..potions.len() {
            let tmp = spells[i] as i64 * potions[j] as i64;
            if  tmp < success {
                continue
            }
            sum += 1;
        }
        ans.push(sum);
    }
    ans
}

pub fn successful_pairs_1(spells: Vec<i32>,mut potions: Vec<i32>, success: i64) -> Vec<i32> {
    let potion_len = potions.len();
    potions.sort_unstable();
    spells.iter().map(|&spell|
        {
            let mut left = 0;
            let mut right = potion_len;

            while left < right {
                let mid = left + (right - left) / 2;
                if potions[mid] as i64 * spell as i64 >= success {
                    right = mid;
                }
                else {
                    left = mid + 1;
                }
            }

            (potion_len - left) as i32
        }
    ).collect()
}

/// 2656. K 个元素的最大和
/// 给你一个下标从 0 开始的整数数组 nums 和一个整数 k 。你需要执行以下操作 恰好 k 次，最大化你的得分：
/// 从 nums 中选择一个元素 m 。
/// 将选中的元素 m 从数组中删除。
/// 将新元素 m + 1 添加到数组中。
/// 你的得分增加 m 。
/// 请你返回执行以上操作恰好 k 次后的最大得分。
/// 示例 1：
/// 输入：nums = [1,2,3,4,5], k = 3
/// 输出：18
/// 解释：我们需要从 nums 中恰好选择 3 个元素并最大化得分。
/// 第一次选择 5 。和为 5 ，nums = [1,2,3,4,6] 。
/// 第二次选择 6 。和为 6 ，nums = [1,2,3,4,7] 。
/// 第三次选择 7 。和为 5 + 6 + 7 = 18 ，nums = [1,2,3,4,8] 。
/// 所以我们返回 18 。
/// 18 是可以得到的最大答案。
/// 示例 2：
/// 输入：nums = [5,5,5], k = 2
/// 输出：11
/// 解释：我们需要从 nums 中恰好选择 2 个元素并最大化得分。
/// 第一次选择 5 。和为 5 ，nums = [5,5,6] 。
/// 第二次选择 6 。和为 6 ，nums = [5,5,7] 。
/// 所以我们返回 11 。
/// 11 是可以得到的最大答案。
///
/// 提示：
/// 1 <= nums.length <= 100
/// 1 <= nums[i] <= 100
/// 1 <= k <= 100

// 自己做的
pub fn maximize_sum(nums: Vec<i32>, k: i32) -> i32 {
    let nums_max = nums.iter().max().unwrap();
    let mut ans = 0;
    for i in 0..k {
        ans += *nums_max + i
    }
    ans
}

// 符合rust的解
pub fn maximize_sum_lc(nums: Vec<i32>, k: i32) -> i32 {
    nums.iter().max().unwrap()*k + k * (k-1) / 2
}

/// 2760. 最长奇偶子数组
/// 给你一个下标从 0 开始的整数数组 nums 和一个整数 threshold 。
/// 请你从 nums 的子数组中找出以下标 l 开头、下标 r 结尾 (0 <= l <= r < nums.length) 且满足以下条件的 最长子数组 ：
/// nums[l] % 2 == 0
/// 对于范围 [l, r - 1] 内的所有下标 i ，nums[i] % 2 != nums[i + 1] % 2
/// 对于范围 [l, r] 内的所有下标 i ，nums[i] <= threshold
/// 以整数形式返回满足题目要求的最长子数组的长度。
///
/// 注意：子数组 是数组中的一个连续非空元素序列。
///
/// 示例 1：
/// 输入：nums = [3,2,5,4], threshold = 5
/// 输出：3
/// 解释：在这个示例中，我们选择从 l = 1 开始、到 r = 3 结束的子数组 => [2,5,4] ，满足上述条件。
/// 因此，答案就是这个子数组的长度 3 。可以证明 3 是满足题目要求的最大长度。
///
/// 示例 2：
/// 输入：nums = [1,2], threshold = 2
/// 输出：1
/// 解释：
/// 在这个示例中，我们选择从 l = 1 开始、到 r = 1 结束的子数组 => [2] 。
/// 该子数组满足上述全部条件。可以证明 1 是满足题目要求的最大长度。
///
/// 示例 3：
/// 输入：nums = [2,3,4,5], threshold = 4
/// 输出：3
/// 解释：
/// 在这个示例中，我们选择从 l = 0 开始、到 r = 2 结束的子数组 => [2,3,4] 。
/// 该子数组满足上述全部条件。
/// 因此，答案就是这个子数组的长度 3 。可以证明 3 是满足题目要求的最大长度。
///
/// 提示：
/// 1 <= nums.length <= 100
/// 1 <= nums[i] <= 100
/// 1 <= threshold <= 100
// 自己做的
pub fn longest_alternating_subarray_1(nums: Vec<i32>, threshold: i32) -> i32 {
    let n = nums.len();
    let mut ans = 0;
    for i in 0..n {
        if nums[i] & 1 == 1 || nums[i] > threshold{
            continue;
        }
        let mut tmp = 1;
        for j in i..(n-1) {
            if nums[j] > threshold || nums[j+1] > threshold {
                break;
            }
            if nums[j] & 1 != nums[j+1] & 1 {
                tmp+=1;
            } else {
                break
            }
        }
        ans = ans.max(tmp);
    }
    ans
}

// 力扣 适合rust的解
pub fn longest_alternating_subarray(nums: Vec<i32>, threshold: i32) -> i32 {
    let n = nums.len();
    let mut ans = 0;
    let mut i = 0;
    while i < n {
        if nums[i] > threshold || nums[i] % 2 != 0 {
            i += 1; // 直接跳过
            continue;
        }
        let start = i; // 记录这一组的开始位置
        i += 1; // 开始位置已经满足要求，从下一个位置开始判断
        while i < n && nums[i] <= threshold && nums[i] % 2 != nums[i - 1] % 2 {
            i += 1;
        }
        // 从 start 到 i-1 是满足题目要求的子数组
        ans = ans.max(i - start);
    }
    ans as i32
}




