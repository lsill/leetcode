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

// 2670. 找出不同元素数目差数组
//给你一个下标从 0 开始的数组 nums ，数组长度为 n 。
//nums 的 不同元素数目差 数组可以用一个长度为 n 的数组 diff 表示，其中 diff[i]
// 等于前缀 nums[0, ..., i] 中不同元素的数目 减去 后缀 nums[i + 1, ..., n - 1] 中不同元素的数目。
//返回 nums 的 不同元素数目差 数组。
//注意 nums[i, ..., j] 表示 nums 的一个从下标 i 开始到下标 j 结束的子数组（包含下标 i 和 j 对应元素）。特别需要说明的是，如果 i > j ，则 nums[i, ..., j] 表示一个空子数组。
//示例 1：
//输入：nums = [1,2,3,4,5]
//输出：[-3,-1,1,3,5]
//解释：
//对于 i = 0，前缀中有 1 个不同的元素，而在后缀中有 4 个不同的元素。因此，diff[0] = 1 - 4 = -3 。
//对于 i = 1，前缀中有 2 个不同的元素，而在后缀中有 3 个不同的元素。因此，diff[1] = 2 - 3 = -1 。
//对于 i = 2，前缀中有 3 个不同的元素，而在后缀中有 2 个不同的元素。因此，diff[2] = 3 - 2 = 1 。
//对于 i = 3，前缀中有 4 个不同的元素，而在后缀中有 1 个不同的元素。因此，diff[3] = 4 - 1 = 3 。
//对于 i = 4，前缀中有 5 个不同的元素，而在后缀中有 0 个不同的元素。因此，diff[4] = 5 - 0 = 5 。
//示例 2：
//输入：nums = [3,2,3,4,2]
//输出：[-2,-1,0,2,3]
//解释：
//对于 i = 0，前缀中有 1 个不同的元素，而在后缀中有 3 个不同的元素。因此，diff[0] = 1 - 3 = -2 。
//对于 i = 1，前缀中有 2 个不同的元素，而在后缀中有 3 个不同的元素。因此，diff[1] = 2 - 3 = -1 。
//对于 i = 2，前缀中有 2 个不同的元素，而在后缀中有 2 个不同的元素。因此，diff[2] = 2 - 2 = 0 。
//对于 i = 3，前缀中有 3 个不同的元素，而在后缀中有 1 个不同的元素。因此，diff[3] = 3 - 1 = 2 。
//对于 i = 4，前缀中有 3 个不同的元素，而在后缀中有 0 个不同的元素。因此，diff[4] = 3 - 0 = 3 。
//提示：
//1 <= n == nums.length <= 50
//1 <= nums[i] <= 50

// lc 优秀的解，的确用一个集合就够了，不需要用俩hashtable
std::vector<int> ArraySimple::distinctDifferenceArray(std::vector<int> &nums) {
    int n = nums.size();
    std::unordered_set<int> ust;
    std::vector<int> sufCnt(n+1, 0);
    for (int i = n-1;i >= 0; --i) {
        ust.insert(nums[i]);
        sufCnt[i] = ust.size();
    }
    std::vector<int> ans;
    ust.clear();
    for (int i = 0; i < n;i++) {
        ust.insert(nums[i]);
        int cnt = ust.size();
        ans.push_back(cnt - sufCnt[i+1]);
    }
    return ans;
}

// 自己做的
std::vector<int> ArraySimple::distinctDifferenceArraySelf(std::vector<int> &nums) {
    const int n = nums.size();
    std::unordered_map<int, int> begin;
    std::unordered_map<int,int>end;
    for (const int& num : nums) {
        end[num]++;
    }
    std::vector<int> diff;
    for (const int& num : nums) {
        begin[num]++;
        end[num]--;
        if (end[num] <= 0) {
            end.erase(num);
        }
        diff.push_back(begin.size() - end.size());
    }
    return diff;
}