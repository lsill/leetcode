//
// Created by lsill on 2024/1/17.
//
#include "arr_base.h"
#include <unordered_set>
#include <unordered_map>
#include <numeric>
#include <string>

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

// 2171. 拿出最少数目的魔法
//给定一个 正整数 数组 beans ，其中每个整数表示一个袋子里装的魔法豆的数目。
//请你从每个袋子中 拿出 一些豆子（也可以 不拿出），使得剩下的 非空 袋子中
// （即 至少还有一颗 魔法豆的袋子）魔法豆的数目 相等。一旦把魔法豆从袋子中取出，你不能再将它放到任何袋子中。
//请返回你需要拿出魔法豆的 最少数目。
//示例 1：
//输入：beans = [4,1,6,5]
//输出：4
//解释：
//- 我们从有 1 个魔法豆的袋子中拿出 1 颗魔法豆。
//  剩下袋子中魔法豆的数目为：[4,0,6,5]
//- 然后我们从有 6 个魔法豆的袋子中拿出 2 个魔法豆。
//  剩下袋子中魔法豆的数目为：[4,0,4,5]
//- 然后我们从有 5 个魔法豆的袋子中拿出 1 个魔法豆。
//  剩下袋子中魔法豆的数目为：[4,0,4,4]
//总共拿出了 1 + 2 + 1 = 4 个魔法豆，剩下非空袋子中魔法豆的数目相等。
//没有比取出 4 个魔法豆更少的方案。
//示例 2：
//输入：beans = [2,10,3,2]
//输出：7
//解释：
//- 我们从有 2 个魔法豆的其中一个袋子中拿出 2 个魔法豆。
//  剩下袋子中魔法豆的数目为：[0,10,3,2]
//- 然后我们从另一个有 2 个魔法豆的袋子中拿出 2 个魔法豆。
//  剩下袋子中魔法豆的数目为：[0,10,3,0]
//- 然后我们从有 3 个魔法豆的袋子中拿出 3 个魔法豆。
//  剩下袋子中魔法豆的数目为：[0,10,0,0]
//总共拿出了 2 + 2 + 3 = 7 个魔法豆，剩下非空袋子中魔法豆的数目相等。
//没有比取出 7 个魔法豆更少的方案。
//提示：
//1 <= beans.length <= 105
//1 <= beans[i] <= 105

// 排序+枚举
long long ArrayMid::minimumRemoval(std::vector<int> &beans) {
    const int n = beans.size();
    if (n == 1) {
        return 0;
    }
    std::sort(beans.begin(), beans.end());
    long long s = std::accumulate(beans.begin(), beans.end(), 0ll);
    long long ans = s;
    for (int i = 0; i < n; i++) {
        // 1ll * beans[i] * (n-i) 剩余的
        ans = std::min(ans, s - 1ll * beans[i] * (n-i));
    }
    return ans;
}

// 670. 最大交换
//给定一个非负整数，你至多可以交换一次数字中的任意两位。返回你能得到的最大值。
//示例 1 :
//输入: 2736
//输出: 7236
//解释: 交换数字2和数字7。
//示例 2 :
//输入: 9973
//输出: 9973
//解释: 不需要交换。
//注意:
//给定数字的范围是 [0, 108]

// lc 一次遍历的解
int ArrayMid::maximumSwap(int num) {
    std::string s = std::to_string(num);
    const int n = s.length();
    int max_idx = n-1;
    int p = -1, q;
    for (int i = n-2; i >= 0;i--) {
        if (s[i] > s[max_idx]) {
            max_idx = i;
        } else if (s[i] < s[max_idx]) {
            p = i;
            q = max_idx;
        }
    }
    if (p == -1) {
        return num;
    }
    std::swap(s[p], s[q]);
    return std::stoi(s);
}