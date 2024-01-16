//
// Created by lsill on 2022/11/3.
//

#include "strTest.h"
#include <string>
#include <unordered_map>

int StringPra::maxRepeating(std::string sequence, std::string word)
{
    int answer = 0;
    string temp = word;
    while (string::size_type postion = sequence.find(temp) != std::string::npos) {
        temp += word;
        answer++;
    }
    return answer;
}

bool StringPra::halvesAreAlike(string s)
{
    int mid = s.size() / 2 ;
    int res = 0;
    for (int i = 0; i < mid;i++)
    {
        if (s[i] == 'a' || s[i] == 'e' || s[i] == 'i' || s[i] == 'o' || s[i] == 'u'|| s[i] == 'A' ||s[i] == 'E'||s[i] == 'I'||s[i] == 'O'||s[i] == 'U')
        {
            ++res;
        }
    }
    for (int i = mid;i < s.size();i++)
    {
        if (s[i] == 'a' || s[i] == 'e' || s[i] == 'i' || s[i] == 'o' || s[i] == 'u'|| s[i] == 'A' ||s[i] == 'E'||s[i] == 'I'||s[i] == 'O'||s[i] == 'U')
        {
            --res;
        }
    }
    return res == 0;
}

bool StringPra::expand(const string &s, const string &t)
{
    int i = 0, j = 0;
    while(i <s.size() && j < t.size())
    {
        if (s[i] != s[j])
        {
            return false;
        }
        char ch = s[i];
        int cnti = 0;
        while (i < s.size() && s[i] == ch)
        {
            ++cnti;
            ++i;
        }
        int cntj = 0;
        while(j < t.size() && t[j] == ch)
        {
            ++cnti;
            ++j;
        }
        if (cnti < cntj)
        {
            return false;
        }
        if (cnti != cntj && cnti < 3)
        {
            return false;
        }
    }
    return i == s.size() && j == t.size();
}

int StringPra::expressiveWords(string s, vector<string> &words)
{
    int ans = 0;
    for (const string& word: words) 
    {
        if (expand(s, word))
        {
            ++ans;
        }
    }
    return ans;
}

// 2085. 统计出现过一次的公共字符串
// 给你两个字符串数组 words1 和 words2 ，请你返回在两个字符串数组中 都恰好出现一次 的字符串的数目。
//示例 1：
// 输入：words1 = ["leetcode","is","amazing","as","is"], words2 = ["amazing","leetcode","is"]
// 输出：2
// 解释：
// - "leetcode" 在两个数组中都恰好出现一次，计入答案。
// - "amazing" 在两个数组中都恰好出现一次，计入答案。
// - "is" 在两个数组中都出现过，但在 words1 中出现了 2 次，不计入答案。
// - "as" 在 words1 中出现了一次，但是在 words2 中没有出现过，不计入答案。
// 所以，有 2 个字符串在两个数组中都恰好出现了一次。
// 示例 2：
// 输入：words1 = ["b","bb","bbb"], words2 = ["a","aa","aaa"]
// 输出：0
// 解释：没有字符串在两个数组中都恰好出现一次。
// 示例 3：
// 输入：words1 = ["a","ab"], words2 = ["a","a","a","ab"]
// 输出：1
// 解释：唯一在两个数组中都出现一次的字符串是 "ab" 。
// 提示：
// 1 <= words1.length, words2.length <= 1000
// 1 <= words1[i].length, words2[j].length <= 30
// words1[i] 和 words2[j] 都只包含小写英文字母。
int StringPra::countWords(vector<string> &words1, vector<string> &words2)
{
    unordered_map<string, int> kv1;
    unordered_map<string, int> kv2;
    for (const string& word : words1) {
        kv1[word] += 1;
    }
    for (const string& word : words2) {
        kv2[word] += 1;
    }
    int ans = 0;
    for (const auto& pair : kv1) {
        if (pair.second == 1) {
            if (kv2[pair.first] == 1) {
                ans += 1;
            }
        }
    }

    return ans;
}

