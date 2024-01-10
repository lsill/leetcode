//
// Created by lsill on 2022/11/3.
//

#include "strTest.h"

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

// 2696. 删除子串后的字符串最小长度
// 给你一个仅由 大写 英文字符组成的字符串 s 。
//你可以对此字符串执行一些操作，在每一步操作中，你可以从 s 中删除 任一个 "AB" 或 "CD" 子字符串。
//通过执行操作，删除所有 "AB" 和 "CD" 子串，返回可获得的最终字符串的 最小 可能长度。
//注意，删除子串后，重新连接出的字符串可能会产生新的 "AB" 或 "CD" 子串。
//示例 1：
//输入：s = "ABFCACDB"
//输出：2
//解释：你可以执行下述操作：
//- 从 "ABFCACDB" 中删除子串 "AB"，得到 s = "FCACDB" 。
//- 从 "FCACDB" 中删除子串 "CD"，得到 s = "FCAB" 。
//- 从 "FCAB" 中删除子串 "AB"，得到 s = "FC" 。
//最终字符串的长度为 2 。
//可以证明 2 是可获得的最小长度。
//示例 2：
//输入：s = "ACBBD"
//输出：5
//解释：无法执行操作，字符串长度不变。
//提示：
//1 <= s.length <= 100
//s 仅由大写英文字母组成

// 栈操作
int StringPra::minLength(std::string s) {
    string stk = " ";
    for (char& c : s) {
        if ((c == 'B' && stk.back() == 'A') || (c == 'D' && stk.back() == 'C')) {
            stk.pop_back();
        } else {
            stk.push_back(c);
        }
    }
    return stk.size() - 1;
}