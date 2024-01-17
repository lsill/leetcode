//
// Created by lsill on 2024/1/17.
//
#include "arr_base.h"
#include <unordered_set>
#include <unordered_map>

// 128. 最长连续序列
//给定一个未排序的整数数组 nums ，找出数字连续的最长序列（不要求序列元素在原数组中连续）的长度。
//请你设计并实现时间复杂度为 O(n) 的算法解决此问题。
//示例 1：
//输入：nums = [100,4,200,1,3,2]
//输出：4
//解释：最长数字连续序列是 [1, 2, 3, 4]。它的长度为 4。
//示例 2：
//输入：nums = [0,3,7,2,5,8,4,6,0,1]
//输出：9
//提示：
//0 <= nums.length <= 105
//-109 <= nums[i] <= 109
// 自己做的超时
int ArrayMid::longestConsecutiveSelf(std::vector<int> &nums) {
    int ans = 0;
    std::unordered_set<int> hash;
    for (const int& num : nums) {
        hash.insert(num);
    }
    for (const int& num : nums) {
        int tmp = num;
        int sum = 0;
        while (hash.contains(tmp)) {
            sum++;
            tmp++;
        }
        ans = std::max(sum, ans);
    }
    return ans;
}

// lc hash表做法
int ArrayMid::longestConsecutiveHash(std::vector<int> &nums) {
    std::unordered_set<int> num_set;
    for (const int& num : nums) {
        num_set.insert(num);
    }
    int longestStreak = 0;
    for (const int& num:num_set) {
        if (!num_set.count(num-1)) {
            int currentNum = num;
            int currentStreak = 1;
            while (num_set.count(currentNum + 1)) {
                currentNum += 1;
                currentStreak += 1;
            }
            longestStreak = std::max(longestStreak, currentStreak);
        }
    }
    return longestStreak;
}

// lc 排序。。。本来以为这样排序不行，结果可以。。
 int ArrayMid::longestConsecutiveSort(std::vector<int> &nums) {
    int n = nums.size();
    if(n==0)
        return 0;

    sort(nums.begin(),nums.end());
    int length=0,temp=1;
    for(int i=0;i<n-1;i++){
        if(nums[i+1]==nums[i]+1){
            temp++;
        }else if(nums[i+1]>nums[i]){
            length=std::max(length,temp);
            temp=1;
        }
    }
    length=std::max(length,temp);
    return length;
}

// lc hash 加上类型动态规划做法
int ArrayMid::longestConsecutiveLikeDp(std::vector<int>& nums) {
    std::unordered_map<int,int> kv;
    int ans = 0;
    int l,r = 0, len;
    for (const int& num : nums) {
       if (!kv[num]) {
           l = kv[num-1];
           r = kv[num + 1];
           len = l +r +1;
           ans = std::max(ans, len);
           kv[num] = len;
           kv[num-l] = len;
           kv[num + r] = len;
       }
    }
    return ans;
}