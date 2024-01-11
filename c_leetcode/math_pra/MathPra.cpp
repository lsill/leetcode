//
// Created by lsill on 2022/10/31.
//

#include "MathPra.h"

// 122112
int MathPra::magicalString(int n) {
    if (n < 4) {
        return 1;
    }
    string s(n, '0');
    s[0] = '1', s[1] = '2', s[2] = '2';
    int res = 1;
    int i = 2;
    int j = 3;
    while (j < n) {
        int size = s[i] - '0';
        int num = 3 - (s[j - 1] - '0');
        while (size > 0 && j < n) {
            s[j] = '0' + num;
            if (num == 1) {
                ++res;
            }
            ++j;
            --size;
        }
        ++i;
    }
    return res;
}

int MathPra::reachNumber(int target)
{
    target = abs(target);
    int s = 0, n = 0;
    while (s < target || (s - target) % 2) // 没有到达（越过）终点，或者相距奇数
        s += ++n;
    return n;
}

// 2645. 构造有效字符串的最少插入数
//给你一个字符串 word ，你可以向其中任何位置插入 "a"、"b" 或 "c" 任意次，返回使 word 有效 需要插入的最少字母数。
//如果字符串可以由 "abc" 串联多次得到，则认为该字符串 有效 。
//示例 1：
//输入：word = "b"
//输出：2
//解释：在 "b" 之前插入 "a" ，在 "b" 之后插入 "c" 可以得到有效字符串 "abc" 。
//示例 2：
//输入：word = "aaa"
//输出：6
//解释：在每个 "a" 之后依次插入 "b" 和 "c" 可以得到有效字符串 "abcabcabc" 。
//示例 3：
//输入：word = "abc"
//输出：0
//解释：word 已经是有效字符串，不需要进行修改。
//提示：
//1 <= word.length <= 50
//word 仅由字母 "a"、"b" 和 "c" 组成。

int MathPra::addMinimum(string word) {
    int t = 1;
    for (int i = 1; i < word.length();i++) {
        t += word[i-1] >= word[i];
    }
    return t * 3 - word.length();
}

int MathPra::addMinimum_1(string s) {
    int ans = s[0] + 2 - s.back();
    for (int i = 1; i < s.length(); i++) {
        ans += (s[i] + 2 - s[i - 1]) % 3;
    }
    return ans;
}