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

// 2808. 使循环数组所有元素相等的最少秒数
//给你一个下标从 0 开始长度为 n 的数组 nums 。
//每一秒，你可以对数组执行以下操作：
//对于范围在 [0, n - 1] 内的每一个下标 i ，将 nums[i] 替换成 nums[i] ，nums[(i - 1 + n) % n] 或者 nums[(i + 1) % n] 三者之一。
//注意，所有元素会被同时替换。
//请你返回将数组 nums 中所有元素变成相等元素所需要的 最少 秒数。
//示例 1：
//输入：nums = [1,2,1,2]
//输出：1
//解释：我们可以在 1 秒内将数组变成相等元素：
//- 第 1 秒，将每个位置的元素分别变为 [nums[3],nums[1],nums[3],nums[3]] 。变化后，nums = [2,2,2,2] 。
//1 秒是将数组变成相等元素所需要的最少秒数。
//示例 2：
//输入：nums = [2,1,3,3,2]
//输出：2
//解释：我们可以在 2 秒内将数组变成相等元素：
//- 第 1 秒，将每个位置的元素分别变为 [nums[0],nums[2],nums[2],nums[2],nums[3]] 。变化后，nums = [2,3,3,3,3] 。
//- 第 2 秒，将每个位置的元素分别变为 [nums[1],nums[1],nums[2],nums[3],nums[4]] 。变化后，nums = [3,3,3,3,3] 。
//2 秒是将数组变成相等元素所需要的最少秒数。
//示例 3：
//输入：nums = [5,5,5,5]
//输出：0
//解释：不需要执行任何操作，因为一开始数组中的元素已经全部相等。
//提示：
//1 <= n == nums.length <= 105
//1 <= nums[i] <= 109

// lc 假设最终所有元素都变成了 xxx，那么 xxx 一定是数组中的某个元素。
//数字 x 每一秒都可以向左右两边扩展一位，如果有多个相同的 x，那么扩展完整个数组所需要的时间，就取决于相邻两个 x 之间的最大间距。
//因此，枚举每个元素作为最终的 x，计算出每个 x 中相邻两个元素之间的最大间距，记为 t，那么最终答案就是 ans = std::min(ans, t /2);
//著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
int ArrayMid::minimumSeconds(std::vector<int> &nums) {
    std::unordered_map<int, std::vector<int>> kv;
    const int n = nums.size();
    for (int i = 0; i < n;i++) {
        kv[nums[i]].push_back(i);
    }
    int ans = 1 << 30;
    for (auto& [_, v]:kv) {
        int m = v.size();
        int t = v[0] + n - v[m-1];
        for (int i = 1; i < m;++i) {
            t = std::max(t, v[i] - v[i-1]);
        }
        ans = std::min(ans, t /2);
    }
    return ans;
}
// 1686. 石子游戏 VI
//Alice 和 Bob 轮流玩一个游戏，Alice 先手。
//一堆石子里总共有 n 个石子，轮到某个玩家时，他可以 移出 一个石子并得到这个石子的价值。
// Alice 和 Bob 对石子价值有 不一样的的评判标准 。双方都知道对方的评判标准。
//给你两个长度为 n 的整数数组 aliceValues 和 bobValues 。aliceValues[i]
// 和 bobValues[i] 分别表示 Alice 和 Bob 认为第 i 个石子的价值。
//所有石子都被取完后，得分较高的人为胜者。如果两个玩家得分相同，那么为平局。两位玩家都会采用 最优策略 进行游戏。
//请你推断游戏的结果，用如下的方式表示：
//如果 Alice 赢，返回 1 。
//如果 Bob 赢，返回 -1 。
//如果游戏平局，返回 0 。
//
//示例 1：
//输入：aliceValues = [1,3], bobValues = [2,1]
//输出：1
//解释：
//如果 Alice 拿石子 1 （下标从 0开始），那么 Alice 可以得到 3 分。
//Bob 只能选择石子 0 ，得到 2 分。
//Alice 获胜。
//示例 2：
//输入：aliceValues = [1,2], bobValues = [3,1]
//输出：0
//解释：
//Alice 拿石子 0 ， Bob 拿石子 1 ，他们得分都为 1 分。
//打平。
//示例 3：
//输入：aliceValues = [2,4,3], bobValues = [1,6,7]
//输出：-1
//解释：
//不管 Alice 怎么操作，Bob 都可以得到比 Alice 更高的得分。
//比方说，Alice 拿石子 1 ，Bob 拿石子 2 ， Alice 拿石子 0 ，Alice 会得到 6 分而 Bob 得分为 7 分。
//Bob 会获胜。
//
//提示：
//n == aliceValues.length == bobValues.length
//1 <= n <= 105
//1 <= aliceValues[i], bobValues[i] <= 100

// lc 很优雅的写法,写的非常妙啊
int ArrayMid::stoneGameVI(std::vector<int> &a, std::vector<int> &b) {
    int n = a.size();
    std::vector<int> ids(n);
    std::iota(ids.begin(), ids.end(), 0);   // 赋初值然后按1递增
    std::ranges::sort(ids, [&](int i, int j){return a[i] + b[i] > a[j] + b[j];});   // 对索引进行排序
    int diff = 0;
    for (int i = 0; i < n; i++) {
        diff += i % 2 == 0 ? a[ids[i]] : -b[ids[i]];
    }
    return (diff > 0) - (diff < 0);
}

// 自己做的，内存消耗太大
int ArrayMid::stoneGameVISelf(std::vector<int> &aliceValues, std::vector<int> &bobValues) {
    const size_t n = aliceValues.size();
    if (n == 1) {
        return 1;
    }
    int ans = 0;
    std::vector<int> values(n);
    std::vector<std::vector<int>> k_index(202);
    for (size_t i = 0; i < n;i++) {
        values[i] = aliceValues[i] + bobValues[i];
        k_index[values[i]].push_back(i);
    }
    std::sort(values.begin(), values.end(), [](int a, int b){
        return a > b;
    });
    int alice = 0, bod = 0;
    for (size_t i = 0; i < n;i++) {
        int val = values[i];
        if ((i & 1) == 0) {
            if (!k_index[val].empty()) {
                alice += aliceValues[k_index[val].back()];
                k_index[val].pop_back();
            }
        } else {
            if (!k_index[val].empty()) {
                bod += bobValues[k_index[val].back()];
                k_index[val].pop_back();
            }

        }
    }
    if (alice > bod) {
        ans = 1;
    } else if (bod > alice) {
        ans = -1;
    }
    return ans;
}

// 560. 和为 K 的子数组
//给你一个整数数组 nums 和一个整数 k ，请你统计并返回 该数组中和为 k 的子数组的个数 。
//子数组是数组中元素的连续非空序列。
//示例 1：
//输入：nums = [1,1,1], k = 2
//输出：2
//示例 2：
//输入：nums = [1,2,3], k = 3
//输出：2
//提示：
//1 <= nums.length <= 2 * 104
//-1000 <= nums[i] <= 1000
//-107 <= k <= 107

// 力扣 前缀和加哈希表优化
int ArrayMid::subarraySum(std::vector<int> &nums, int k) {
    std::unordered_map<int, int> mp;
    mp[0] = 1;
    int count = 0, pre = 0;
    for (auto& x : nums) {
        pre += x;
        if (mp.find(pre - k) != mp.end()) {
            count += mp[pre - k];
        }
        mp[pre]++;
    }
    return count;
}

// 自己做的暴力解
int ArrayMid::subarraySumSelf(std::vector<int> &nums, int k) {
    int ans = 0;
    int n = nums.size();
    int l = 0;
    while (l < n) {
        int tmp = nums[l];
        if (tmp == k) {
            ans++;
        }
        for (int i = l+1; i < n;i++) {
            tmp += nums[i];
            if (tmp == k) {
                ans++;
            }
        }
        l++;
    }
    return ans;
}
// 2789. 合并后数组中的最大元素
//给你一个下标从 0 开始、由正整数组成的数组 nums 。
//你可以在数组上执行下述操作 任意 次：
//选中一个同时满足 0 <= i < nums.length - 1 和 nums[i] <= nums[i + 1] 的整数 i 。将元素 nums[i + 1] 替换为 nums[i] + nums[i + 1] ，并从数组中删除元素 nums[i] 。
//返回你可以从最终数组中获得的 最大 元素的值。
//示例 1：
//输入：nums = [2,3,7,9,3]
//输出：21
//解释：我们可以在数组上执行下述操作：
//- 选中 i = 0 ，得到数组 nums = [5,7,9,3] 。
//- 选中 i = 1 ，得到数组 nums = [5,16,3] 。
//- 选中 i = 0 ，得到数组 nums = [21,3] 。
//最终数组中的最大元素是 21 。可以证明我们无法获得更大的元素。
//示例 2：
//输入：nums = [5,3,3]
//输出：11
//解释：我们可以在数组上执行下述操作：
//- 选中 i = 1 ，得到数组 nums = [5,6] 。
//- 选中 i = 0 ，得到数组 nums = [11] 。
//最终数组中只有一个元素，即 11 。
//提示：
//1 <= nums.length <= 105
//1 <= nums[i] <= 106

// lc 解
long long ArrayMid::maxArrayValue(std::vector<int> &nums) {
    long long sum = nums.back();
    for (int i = nums.size() - 2; i >= 0; i--) {
        sum = nums[i] <= sum ? sum + nums[i] : nums[i];
    }
    return sum;
}

// 2766. 重新放置石块
//给你一个下标从 0 开始的整数数组 nums ，表示一些石块的初始位置。再给你两个长度 相等 下标从 0 开始的整数数组 moveFrom 和 moveTo 。
//在 moveFrom.length 次操作内，你可以改变石块的位置。在第 i 次操作中，你将位置在 moveFrom[i] 的所有石块移到位置 moveTo[i] 。
//完成这些操作后，请你按升序返回所有 有 石块的位置。
//注意：
//如果一个位置至少有一个石块，我们称这个位置 有 石块。
//一个位置可能会有多个石块。
// 示例 1
//输入：nums = [1,6,7,8], moveFrom = [1,7,2], moveTo = [2,9,5]
//输出：[5,6,8,9]
//解释：一开始，石块在位置 1,6,7,8 。
//第 i = 0 步操作中，我们将位置 1 处的石块移到位置 2 处，位置 2,6,7,8 有石块。
//第 i = 1 步操作中，我们将位置 7 处的石块移到位置 9 处，位置 2,6,8,9 有石块。
//第 i = 2 步操作中，我们将位置 2 处的石块移到位置 5 处，位置 5,6,8,9 有石块。
//最后，至少有一个石块的位置为 [5,6,8,9] 。
//示例 2：
//输入：nums = [1,1,3,3], moveFrom = [1,3], moveTo = [2,2]
//输出：[2]
//解释：一开始，石块在位置 [1,1,3,3] 。
//第 i = 0 步操作中，我们将位置 1 处的石块移到位置 2 处，有石块的位置为 [2,2,3,3] 。
//第 i = 1 步操作中，我们将位置 3 处的石块移到位置 2 处，有石块的位置为 [2,2,2,2] 。
//由于 2 是唯一有石块的位置，我们返回 [2] 。
//提示：
//1 <= nums.length <= 105
//1 <= moveFrom.length <= 105
//moveFrom.length == moveTo.length
//1 <= nums[i], moveFrom[i], moveTo[i] <= 109
//测试数据保证在进行第 i 步操作时，moveFrom[i] 处至少有一个石块。

// lc 比较好的解
std::vector<int> ArrayMid::relocateMarbles(std::vector<int> &nums, std::vector<int> &moveFrom, std::vector<int> &moveTo) {
    std::unordered_set<int> st(nums.begin(), nums.end());
    for (int i = 0; i < moveFrom.size(); i++) {
        st.erase(moveFrom[i]);
        st.insert(moveTo[i]);
    }
     std::vector<int> ans(st.begin(), st.end());
     ranges::sort(ans);
     return ans;
}
