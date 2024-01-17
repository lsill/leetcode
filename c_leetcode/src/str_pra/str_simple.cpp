//
// Created by lsill on 2024/1/17.
//
#include "strTest.h"

// 2744. 最大字符串配对数目
//给你一个下标从 0 开始的数组 words ，数组中包含 互不相同 的字符串。
//如果字符串 words[i] 与字符串 words[j] 满足以下条件，我们称它们可以匹配：
//字符串 words[i] 等于 words[j] 的反转字符串。
//0 <= i < j < words.length
//请你返回数组 words 中的 最大 匹配数目。
//注意，每个字符串最多匹配一次。
//示例 1：
//输入：words = ["cd","ac","dc","ca","zz"]
//输出：2
//解释：在此示例中，我们可以通过以下方式匹配 2 对字符串：
//- 我们将第 0 个字符串与第 2 个字符串匹配，因为 word[0] 的反转字符串是 "dc" 并且等于 words[2]。
//- 我们将第 1 个字符串与第 3 个字符串匹配，因为 word[1] 的反转字符串是 "ca" 并且等于 words[3]。
//可以证明最多匹配数目是 2 。
//示例 2：
//输入：words = ["ab","ba","cc"]
//输出：1
//解释：在此示例中，我们可以通过以下方式匹配 1 对字符串：
//- 我们将第 0 个字符串与第 1 个字符串匹配，因为 words[1] 的反转字符串 "ab" 与 words[0] 相等。
//可以证明最多匹配数目是 1 。
//示例 3：
//输入：words = ["aa","ab"]
//输出：0
//解释：这个例子中，无法匹配任何字符串。

// 自己做的垃圾
int StringSimple::maximumNumberOfStringPairsSelf(vector<std::string> &words) {
    int n = words.size();
    int ans = 0;
    for (int i = 0;i < n-1;i++) {
        for (int j = i +1; j < n;j++) {
            if (words[i].length() != words[j].length()) {
                continue;
            }
            int index = words[i].length();
            bool bContain = true;
            for (int z = 0; z < index;z++) {
                if (words[i][z] != words[j][index-z-1]) {
                    bContain = false;
                    break;
                }
            }
            if (bContain) {
                ans++;
            }
        }
    }
    return ans;
}

// lc比较优秀的解
int StringSimple::maximumNumberOfStringPairs(vector<std::string> &words) {
    int ans = 0;
    bool seen[26][26]{};
    for (auto &s : words) {
        int x = s[0] - 'a';
        int y = s[1] - 'a';
        if (seen[y][x]) {
            ans++; // s 和 seen中的y+x匹配
        } else {
            seen[x][y] = true;
        }
    }
    return ans;
}