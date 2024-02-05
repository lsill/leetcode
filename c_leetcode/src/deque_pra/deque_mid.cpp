//
// Created by lsill on 2024/2/5.
//
#include "deque_base.h"
#include <functional>
#include <deque>

// 1696. 跳跃游戏 VI
//给你一个下标从 0 开始的整数数组 nums 和一个整数 k 。
//一开始你在下标 0 处。每一步，你最多可以往前跳 k 步，但你不能跳出数组的边界。
// 也就是说，你可以从下标 i 跳到 [i + 1， min(n - 1, i + k)] 包含 两个端点的任意位置。
//你的目标是到达数组最后一个位置（下标为 n - 1 ），你的 得分 为经过的所有数字之和。
//请你返回你能得到的 最大得分 。
//示例 1：
//输入：nums = [1,-1,-2,4,-7,3], k = 2
//输出：7
//解释：你可以选择子序列 [1,-1,4,3] （上面加粗的数字），和为 7 。
//示例 2：
//输入：nums = [10,-5,-2,4,0,3], k = 3
//输出：17
//解释：你可以选择子序列 [10,4,3] （上面加粗数字），和为 17 。
//示例 3：
//输入：nums = [1,-5,-20,4,-1,3,-6,-3], k = 2
//输出：0
//提示：
// 1 <= nums.length, k <= 105
//-104 <= nums[i] <= 104
// lc题解
int DequeMid::maxResult(vector<int> &nums, int k) {
    //递归做法
    /*function<int(int)> dfs = [&](int i) -> int {
        if (i == 0) {
            return nums[0];
        }
        int mx = INT_MIN;
        for (int j = max(i - k, 0); j < i;j++) {
            mx = max(mx, dfs(j));
        }
        return mx + nums[i];
    };
    return dfs(nums.size() - 1);*/
    // 递推做法
    /*int n = nums.size();
    vector<int> f(n);
    f[0] = nums[0];
    for (int i = 1; i < n;i++) {
        f[i] = *max_element(f.begin()+ max(i - k, 0), f.begin() + i) + nums[i];
    }
    return f[n-1];*/
    // 单调队列
   /* int n = nums.size();
    vector<int> f(n);
    f[0] = nums[0];
    deque<int> q = {0};
    for (int i = 1; i < n;i++) {
        // 1.出
        if (q.front() < i- k) {
            q.pop_front();
        }
        // 2. 转移
        f[i] = f[q.front()] + nums[i];
        // 3. 入
        while (!q.empty() && f[i] >= f[q.back()]) {
            q.pop_back();
        }
        q.push_back(i);
    }
    return f[n-1];*/
   // 单调队列空间优化
   deque<int> q = {0};
   for (int i = 1; i < nums.size();i++) {
       // 1. 出
       if (q.front() < i - k) {
           q.pop_front();
       }
       // 2. 转移
       nums[i] += nums[q.front()];
       // 3. 入
       while (!q.empty() && nums[i] >= nums[q.back()]) {
           q.pop_back();
       }
       q.push_back(i);
   }
    return nums.back();
}