//
// Created by lsill on 2024/1/16.
//

#include "dp_base.h"
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
int DpHard::count(std::string num1, std::string num2, int min_sum, int max_sum){
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
