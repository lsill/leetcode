//
// Created by lsill on 2024/1/16.
//

#include "dp_base.h"
#include <functional>
#include <string>
// 2719. 统计整数数目
//给你两个数字字符串 num1 和 num2 ，以及两个整数 max_sum 和 min_sum 。如果一个整数 x 满足以下条件，我们称它是一个好整数：
//num1 <= x <= num2
//min_sum <= digit_sum(x) <= max_sum.
//请你返回好整数的数目。答案可能很大，请返回答案对 109 + 7 取余后的结果。
//注意，digit_sum(x) 表示 x 各位数字之和。
//示例 1：
//输入：num1 = "1", num2 = "12", min_num = 1, max_num = 8
//输出：11
//解释：总共有 11 个整数的数位和在 1 到 8 之间，分别是 1,2,3,4,5,6,7,8,10,11 和 12 。所以我们返回 11 。
//示例 2：
//输入：num1 = "1", num2 = "5", min_num = 1, max_num = 5
//输出：5
//解释：数位和在 1 到 5 之间的 5 个整数分别为 1,2,3,4 和 5 。所以我们返回 5 。
//提示：
//1 <= num1 <= num2 <= 1022
//1 <= min_sum <= max_sum <= 400

// 求区间[num1,num2]中，数位和在[min_sum, max_sum]的数的个数。对于这种区间[l,r]的问题可以
// 转化成[1,r]和[r,l-1]的答案，然后相减即可。
int DpHard::countLc(std::string num1, std::string num2, int min_sum, int max_sum){
    const int mod = 1e9 + 7;
    int f[23][220];
    memset(f, -1, sizeof(f));
    std::string num = num2;

    std::function<int(int,int,bool )> dfs = [&](int pos, int s, bool limit) -> int {
        if (pos >= num.size()) {
            return s >= min_sum && s <= max_sum ? 1 : 0;
        }
        if (!limit && f[pos][s] != -1) {
            return f[pos][s];
        }
        int up = limit ? num[pos] - '0' : 9;
        int ans = 0;
        for (int i = 0; i <= up; ++i) {
            ans += dfs(pos + 1, s + i, limit && i == up);
            ans %= mod;
        }
        if (!limit) {
            f[pos][s] = ans;
        }
        return ans;
    };

    int a = dfs(0, 0, true);
    for (int i = num1.size() - 1; ~i; --i) {
        if (num1[i] == '0') {
            num1[i] = '9';
        } else {
            num1[i] -= 1;
            break;
        }
    }
    num = num1;
    memset(f, -1, sizeof(f));
    int b = dfs(0, 0, true);
    return (a - b + mod) % mod;
}

void DpHard::prepare() {
    static bool done = false;
    if (done) return;
    done = true;
    memset(f, 0, sizeof(f));
    memset(g, 0, sizeof(g));
    f[0][0] = f[1][0] = 1;
    for (int i = 0; i <= 400; i ++) {
        g[0][i] = 1;
    }
    for (int l = 1; l < 25; l ++) {
        for (int i = 0; i <= 400; ++ i) {
            for (int j = 0; j <= 9 && j <= i; j ++) {
                g[l][i] = (g[l][i] + g[l - 1][i - j]) % MOD;
            }
        }
    }
}

long long DpHard::solve(std::string num, int m) {
    int n = num.size();
    long long ans = g[n - 1][m];
    int digit_sum = 0;
    for (int i = 0; i < n; i ++) {
        int digit = num[i] - '0';
        for (int j = 0; j < digit; j ++) {
            if (i == 0 && j == 0) {
                continue;
            }
            if (m - digit_sum - j <= 0) {
                continue;
            }
            ans += g[n - 1 - i][m - digit_sum - j];
        }
        digit_sum += digit;
    }
    return ans % MOD;
}

int DpHard::countBest(std::string num1, std::string num2, int min_sum, int max_sum) {
    prepare();
    long long ans = 0;
    ans += solve(num2, max_sum);
    ans -= solve(num1, max_sum);
    ans -= solve(num2, min_sum - 1);
    ans += solve(num1, min_sum - 1);
    int digit_sum = 0;
    for (int i = 0; i < num1.size(); ++ i) {
        digit_sum += num1[i] - '0';
    }
    if (min_sum <= digit_sum && digit_sum <= max_sum) {
        ans ++;
    }
    return (ans % MOD + MOD) % MOD;
}

// 514. 自由之路
//已解答
//困难
//相关标签
//相关企业
//电子游戏“辐射4”中，任务 “通向自由” 要求玩家到达名为 “Freedom Trail Ring” 的金属表盘，并使用表盘拼写特定关键词才能开门。
//给定一个字符串 ring ，表示刻在外环上的编码；给定另一个字符串 key ，表示需要拼写的关键词。您需要算出能够拼写关键词中所有字符的最少步数。
//最初，ring 的第一个字符与 12:00 方向对齐。您需要顺时针或逆时针旋转 ring 以使 key 的一个字符在 12:00 方向对齐，然后按下中心按钮，以此逐个拼写完 key 中的所有字符。
//旋转 ring 拼出 key 字符 key[i] 的阶段中：
//您可以将 ring 顺时针或逆时针旋转 一个位置 ，计为1步。旋转的最终目的是将字符串 ring 的一个字符与 12:00 方向对齐，并且这个字符必须等于字符 key[i] 。
//如果字符 key[i] 已经对齐到12:00方向，您需要按下中心按钮进行拼写，这也将算作 1 步。按完之后，您可以开始拼写 key 的下一个字符（下一阶段）, 直至完成所有拼写。
//示例 1：
//输入: ring = "godding", key = "gd"
//输出: 4
//解释:
// 对于 key 的第一个字符 'g'，已经在正确的位置, 我们只需要1步来拼写这个字符。
// 对于 key 的第二个字符 'd'，我们需要逆时针旋转 ring "godding" 2步使它变成 "ddinggo"。
// 当然, 我们还需要1步进行拼写。
// 因此最终的输出是 4。
//示例 2:
//输入: ring = "godding", key = "godding"
//输出: 13
//提示：
//1 <= ring.length, key.length <= 100
//ring 和 key 只包含小写英文字母
//保证 字符串 key 一定可以由字符串  ring 旋转拼出

// lc题解
int DpHard::findRotateSteps(std::string ring, std::string key) {
    int n = ring.size(), m = key.size();
    std::vector<int> pos[26];
    for (int i = 0; i < n; ++i) {
        pos[ring[i] - 'a'].push_back(i);
    }
    std::vector<std::vector<int>> dp(m, std::vector<int>(n, 0x3f3f3f3f));
    for (auto& i: pos[key[0] - 'a']) {
        dp[0][i] = std::min(i, n - i) + 1;
    }
    for (int i = 1; i < m; ++i) {
        for (auto& j: pos[key[i] - 'a']) {
            for (auto& k: pos[key[i - 1] - 'a']) {
                dp[i][j] = std::min(dp[i][j], dp[i - 1][k] + std::min(abs(j - k), n - abs(j - k)) + 1);
            }
        }
    }
    return *min_element(dp[m - 1].begin(), dp[m - 1].end());
}

// 3098. 求出所有子序列的能量和
// 给你一个长度为 n 的整数数组 nums 和一个 正 整数 k 。一个子序列的能量定义为子序列中
// 任意 两个元素的差值绝对值的 最小值 。请你返回 nums 中长度 等于 k 的 所有 子序列的
// 能量和 。由于答案可能会很大，将答案对 109 + 7 取余 后返回。
// 示例 1：
// 输入：nums = [1,2,3,4], k = 3
// 输出：4
// 解释：
// nums 中总共有 4 个长度为 3 的子序列：[1,2,3] ，[1,3,4] ，[1,2,4] 和 [2,3,4] 。
// 能量和为 |2 - 3| + |3 - 4| + |2 - 1| + |3 - 4| = 4 。
// 示例 2：
// 输入：nums = [2,2], k = 2
// 输出：0
// 解释：
// nums 中唯一一个长度为 2 的子序列是 [2,2] 。能量和为 |2 - 2| = 0 。
// 示例 3：
// 输入：nums = [4,3,-1], k = 2
// 输出：10
// 解释：
// nums 总共有 3 个长度为 2 的子序列：[4,3] ，[4,-1] 和 [3,-1] 。能量和为 |4 - 3| + |4 - (-1)| + |3 - (-1)| = 10 。
// 提示：
// 2 <= n == nums.length <= 50
// -108 <= nums[i] <= 108
// 2 <= k <= n
// lc 困难题，直接抄的看不懂
int DpHard::sumOfPowers(std::vector<int> &nums, int k) {
        int n = nums.size();
        sort(nums.begin(), nums.end());
        std::vector<int> vals;
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < i; j++) {
                vals.push_back(nums[i] - nums[j]);
            }
        }
        vals.push_back(inf);
        sort(vals.begin(), vals.end());
        vals.erase(unique(vals.begin(), vals.end()), vals.end());

        vector<vector<vector<int>>> d(n, vector(k + 1, vector(vals.size(), 0)));
        vector<vector<int>> border(n, vector(k + 1, 0));
        vector<vector<int>> sum(k + 1, vector(vals.size(), 0));
        vector<vector<int>> suf(n, vector(k + 1, 0));

        for (int i = 0; i < n; i++) {
            for (int j = 0; j < i; j++) {
                int pos = lower_bound(vals.begin(), vals.end(), nums[i] - nums[j]) - vals.begin();
                for (int p = 1; p <= k; p++) {
                    while (border[j][p] < pos) {
                        sum[p][border[j][p]] = (sum[p][border[j][p]] - suf[j][p] + mod) % mod;
                        sum[p][border[j][p]] = (sum[p][border[j][p]] + d[j][p][border[j][p]]) % mod;
                        suf[j][p] = (suf[j][p] - d[j][p][border[j][p]] + mod) % mod;
                        border[j][p]++;
                        sum[p][border[j][p]] = (sum[p][border[j][p]] + suf[j][p]);
                    }
                }
            }

            d[i][1][vals.size() - 1] = 1;
            for (int p = 2; p <= k; p++) {
                for (int v = 0; v < vals.size(); v++) {
                    d[i][p][v] = sum[p - 1][v];
                }
            }
            for (int p = 1; p <= k; p++) {
                for (int v = 0; v < vals.size(); v++) {
                    suf[i][p] = (suf[i][p] + d[i][p][v]) % mod;
                }
                sum[p][0] = (sum[p][0] + suf[i][p]) % mod;
            }
        }

        int res = 0;
        for (int i = 0; i < n; i++) {
            for (int v = 0; v < vals.size(); v++) {
                res = (res + 1ll * vals[v] * d[i][k][v] % mod) % mod;
            }
        }
        return res;
}
