pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0
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
    if ans == i32::MAX {0} else {ans}
}

pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
    let n = nums.len();
    if nums.len() == 1 {
        if nums[0] > k {
            return 1
        }
        return 0;
    }
    let mut sum = 0;
    let mut left:usize = 0;
    let mut tmp_mul:i32 = nums[left];
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
                tmp_mul  = tmp_mul / nums[left];
                left +=1;
            }
        }
    }
    sum
}