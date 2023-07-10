pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }
    let (mut l, mut r, mut sum, mut ans) = (0, 0, nums[0], i32::MAX);
    while r < nums.len() - 1 || sum >= target {
        if sum >= target {
            ans = ans.min((r - l + 1) as i32);
            sum -= nums[l];
            l += 1;
        } else {
            r += 1;
            sum += nums[r]
        }
    }
    if ans == i32::MAX {
        0
    } else {
        ans
    }
}

pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    if nums.len() == 1 {
        if nums[0] > k {
            return 1;
        }
        return 0;
    }
    let mut sum = 0;
    let mut left: usize = 0;
    let mut tmp_mul: i32 = nums[left];
    if nums[left] < k {
        sum += 1;
    } else {
        tmp_mul = 1;
    }
    for i in 1..n {
        tmp_mul = tmp_mul * nums[i];
        if tmp_mul < k {
            sum += 1;
        } else {
            while tmp_mul >= k {
                tmp_mul = tmp_mul / nums[left];
                left += 1;
            }
        }
    }
    sum
}

/// 16. 最接近的三数之和
/// 给你一个长度为 n 的整数数组 nums 和 一个目标值 target。请你从 nums 中选出三个整数，使它们的和与 target 最接近。
/// 返回这三个数的和。
/// 假定每组输入只存在恰好一个解。

// lc评价基础解
pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
    nums.sort_unstable();
    let mut ans = 1e7 as i32;
    for i in 0..nums.len() - 2 {
        if i > 0 && nums[i] == nums[i - 1] {
            continue;
        }
        let (mut j, mut k) = (i + 1, nums.len() - 1);
        while j < k {
            let sum = nums[i] + nums[j] + nums[k];
            if sum == target {
                return target;
            }
            if (sum - target).abs() < (ans - target).abs() {
                ans = sum;
            }
            if sum > target {
                k -= 1;
                while k > j && nums[k] == nums[k + 1] {
                    k -= 1;
                }
            } else if sum < target {
                j += 1;
                while j < k && nums[j] == nums[j - 1] {
                    j += 1;
                }
            }
        }
    }
    ans
}

// lc 7月10日最优解
pub fn three_sum_closest_0(nums: Vec<i32>, target: i32) -> i32 {
    let mut nums = nums;
    let len = nums.len();
    if len == 3 {
        return nums.iter().sum::<i32>();
    }

    nums.sort_unstable();

    let m = nums[0..3].iter().sum();
    if m >= target {
        return m;
    }
    let x = nums[nums.len() - 3..nums.len()].iter().sum();
    if x <= target {
        return x;
    }

    let mut diff = i32::MAX;
    for i in 0..len - 2 {
        let t = target - nums[i];
        let mut left = i + 1;
        let mut right = len - 1;
        while left < right {
            if nums[left] + nums[right] < t {
                if (nums[left] + nums[right] - t).abs() < diff.abs() {
                    diff = nums[left] + nums[right] - t;
                }
                left += 1;
            } else if nums[left] + nums[right] > t {
                if (nums[left] + nums[right] - t).abs() < diff.abs() {
                    diff = nums[left] + nums[right] - t;
                }
                right -= 1
            } else {
                return target;
            }
        }
    }
    return target + diff;
}
