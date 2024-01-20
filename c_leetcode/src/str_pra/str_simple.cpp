//
// Created by lsill on 2024/1/17.
//
#include <sstream>
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


// 2788. 按分隔符拆分字符串
//给你一个字符串数组 words 和一个字符 separator ，请你按 separator 拆分 words 中的每个字符串。
//返回一个由拆分后的新字符串组成的字符串数组，不包括空字符串 。
//注意
//separator 用于决定拆分发生的位置，但它不包含在结果字符串中。
//拆分可能形成两个以上的字符串。
//结果字符串必须保持初始相同的先后顺序。
//示例 1：
//输入：words = ["one.two.three","four.five","six"], separator = "."
//输出：["one","two","three","four","five","six"]
//解释：在本示例中，我们进行下述拆分：
//"one.two.three" 拆分为 "one", "two", "three"
//"four.five" 拆分为 "four", "five"
//"six" 拆分为 "six"
//因此，结果数组为 ["one","two","three","four","five","six"] 。
//示例 2：
//输入：words = ["$easy$","$problem$"], separator = "$"
//输出：["easy","problem"]
//解释：在本示例中，我们进行下述拆分：
//"$easy$" 拆分为 "easy"（不包括空字符串）
//"$problem$" 拆分为 "problem"（不包括空字符串）
//因此，结果数组为 ["easy","problem"] 。
//示例 3：
//输入：words = ["|||"], separator = "|"
//输出：[]
//解释：在本示例中，"|||" 的拆分结果将只包含一些空字符串，所以我们返回一个空数组 [] 。
//提示：
//1 <= words.length <= 100
//1 <= words[i].length <= 20
//words[i] 中的字符要么是小写英文字母，要么就是字符串 ".,|$#@" 中的字符（不包括引号）
//separator 是字符串 ".,|$#@" 中的某个字符（不包括引号）


// 自己做的
vector<string> StringSimple::splitWordsBySeparatorSelf(vector<std::string> &words, char separator) {
    vector<string> ans;
    for (const string& word :words) {
        string tmp = "";
        for (const char& w : word) {
            if (w == separator) {
                if (tmp != "") {
                    ans.push_back(tmp);
                    tmp = "";
                }
            } else{
                tmp.push_back(w);
            }
        }
        if (tmp != "") {
            ans.push_back(tmp);
        }
    }
    return ans;
}

// lc 流处理
vector<string> StringSimple::splitWordsBySeparator(vector<std::string> &words, char separator) {
    vector<string> ans;
    for (const string& word :words) {
        istringstream ss(word);
        string s;
        while (getline(ss, s, separator)) {
            if (!s.empty()) {
                ans.push_back(s);
            }
        }
    }
    return ans;
}