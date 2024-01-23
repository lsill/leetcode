//
// Created by lsill on 2024/1/23.
//
#include "arr_base.h"

// 2765. 最长交替子数组
//给你一个下标从 0 开始的整数数组 nums 。如果 nums 中长度为 m 的子数组 s 满足以下条件，我们称它是一个 交替子数组 ：
//m 大于 1 。
//s1 = s0 + 1 。
//下标从 0 开始的子数组 s 与数组 [s0, s1, s0, s1,...,s(m-1) % 2] 一样。也就是说，s1 - s0 = 1 ，s2 - s1 = -1 ，s3 - s2 = 1 ，s4 - s3 = -1 ，以此类推，直到 s[m - 1] - s[m - 2] = (-1)m 。
//请你返回 nums 中所有 交替 子数组中，最长的长度，如果不存在交替子数组，请你返回 -1 。
//子数组是一个数组中一段连续 非空 的元素序列。
//示例 1：
//输入：nums = [2,3,4,3,4]
//输出：4
//解释：交替子数组有 [3,4] ，[3,4,3] 和 [3,4,3,4] 。最长的子数组为 [3,4,3,4] ，长度为4 。
//示例 2：
//输入：nums = [4,5,6]
//输出：2
//解释：[4,5] 和 [5,6] 是仅有的两个交替子数组。它们长度都为 2 。
//提示：
//2 <= nums.length <= 100
//1 <= nums[i] <= 104

// lc的解法 很精妙 一次循环
int ArraySimple::alternatingSubarray(std::vector<int> &nums) {
    int ans = -1;
    int i = 0, n = nums.size();
    while (i < n - 1) {
        if (nums[i + 1] - nums[i] != 1) {
            i++; // 直接跳过
            continue;
        }
        int i0 = i; // 记录这一组的开始位置
        i += 2; // i 和 i+1 已经满足要求，从 i+2 开始判断
        while (i < n && nums[i] == nums[i0] + (i - i0) % 2) {
            i++;
        }
        // 从 i0 到 i-1 是满足题目要求的（并且无法再延长的）子数组
        ans = std::max(ans, i - i0);
        i--;
    }
    return ans;
}

// 自己做的
int ArraySimple::alternatingSubarraySelf(std::vector<int> &nums) {
    int ans = -1;
    int  i = 1;
    const int n = nums.size();
    for (;i<n;i++) {
        int sub = nums[i] - nums[i-1];
        if (sub == 1) {
            int tmp = 2;
            for (int j = i+1;j < n;j++) {
                if (nums[j] - nums[j-1] == -sub) {
                    tmp++;
                    sub = -sub;
                } else{
                    break;
                }
            }
            ans = std::max(ans, tmp);
        }
    }
    return ans;
}