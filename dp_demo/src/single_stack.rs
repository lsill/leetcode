/// 2866. 美丽塔 II
/// 给你一个长度为 n 下标从 0 开始的整数数组 maxHeights 。
/// 你的任务是在坐标轴上建 n 座塔。第 i 座塔的下标为 i ，高度为 heights[i] 。
/// 如果以下条件满足，我们称这些塔是 美丽 的：
/// 1 <= heights[i] <= maxHeights[i]
/// heights 是一个 山脉 数组。
/// 如果存在下标 i 满足以下条件，那么我们称数组 heights 是一个 山脉 数组：
/// 对于所有 0 < j <= i ，都有 heights[j - 1] <= heights[j]
/// 对于所有 i <= k < n - 1 ，都有 heights[k + 1] <= heights[k]
/// 请你返回满足 美丽塔 要求的方案中，高度和的最大值 。
/// 示例 1：
/// 输入：maxHeights = [5,3,4,1,1]
/// 输出：13
/// 解释：和最大的美丽塔方案为 heights = [5,3,3,1,1] ，这是一个美丽塔方案，因为：
/// - 1 <= heights[i] <= maxHeights[i]
/// - heights 是个山脉数组，峰值在 i = 0 处。
/// 13 是所有美丽塔方案中的最大高度和。
/// 示例 2：
/// 输入：maxHeights = [6,5,3,9,2,7]
/// 输出：22
/// 解释： 和最大的美丽塔方案为 heights = [3,3,3,9,2,2] ，这是一个美丽塔方案，因为：
/// - 1 <= heights[i] <= maxHeights[i]
/// - heights 是个山脉数组，峰值在 i = 3 处。
/// 22 是所有美丽塔方案中的最大高度和。
/// 示例 3：
/// 输入：maxHeights = [3,2,5,5,2,3]
/// 输出：18
/// 解释：和最大的美丽塔方案为 heights = [2,2,5,5,2,2] ，这是一个美丽塔方案，因为：
/// - 1 <= heights[i] <= maxHeights[i]
/// - heights 是个山脉数组，最大值在 i = 2 处。
/// 注意，在这个方案中，i = 3 也是一个峰值。
/// 18 是所有美丽塔方案中的最大高度和。
/// 提示：
/// 1 <= n == maxHeights <= 105
/// 1 <= maxHeights[i] <= 109
/// 动态规划+单调栈
/// 力扣一种写法
fn maximum_sum_of_heights(a: Vec<i32>) -> i64 {
    let mut n = a.len();
    let mut st = Vec::with_capacity(n);
    let mut right = vec![0;n];
    let mut left = vec![0;n];
    let mut res = 0;
    for (i,&x) in a.iter().enumerate().rev() {
        while let Some(&j) = st.last() {
            if a[j] < x { break; }
            st.pop();
        }
        if let Some(&j) = st.last() {
            right[i] = x as i64* (j - i) as i64 + right[j];
        } else {
            right[i] = x as i64* (n - i) as i64;
        }
        st.push(i);
    }
    st.clear();
    for (i,&x) in a.iter().enumerate() {
        let x = x;
        while let Some(&j) = st.last() {
            if a[j] < x { break; }
            st.pop();
        }
        if let Some(&j) = st.last() {
            left[i] = x as i64* (i - j) as i64 + left[j];
        } else {
            left[i] = x as i64* (i + 1) as i64;
        }
        st.push(i);
        res = res.max(left[i] + right[i] - a[i] as i64);
    }
    res
}


// 力扣另外一种差不多的
pub fn maximum_sum_of_heights_1(max_heights: Vec<i32>) -> i64 {
    let n = max_heights.len();
    let (mut stack1, mut stack2) = (Vec::new(), Vec::new());
    let (mut arr1, mut arr2) = (vec![0; n + 1], vec![0; n + 1]);
    for i in 0..n {
        arr1[i + 1] = arr1[i] + max_heights[i] as i64;
        while let Some(&top) = stack1.last() {
            if max_heights[i] > max_heights[top] {
                break;
            }
            stack1.pop();
            arr1[i + 1] -= (max_heights[top] - max_heights[i]) as i64
                * if let Some(&pre) = stack1.last() {
                (top - pre) as i64
            } else {
                top as i64 + 1
            }
        }
        stack1.push(i);
    }
    for i in (0..n).rev() {
        arr2[i] = arr2[i + 1] + max_heights[i] as i64;
        while let Some(&top) = stack2.last() {
            if max_heights[i] > max_heights[top] {
                break;
            }
            stack2.pop();
            arr2[i] -= (max_heights[top] - max_heights[i]) as i64
                * if let Some(&pre) = stack2.last() {
                (pre - top) as i64
            } else {
                (n - top) as i64
            }
        }
        stack2.push(i);
    }
    (0..=n).map(|i| arr1[i] + arr2[i]).max().unwrap_or_default()
}