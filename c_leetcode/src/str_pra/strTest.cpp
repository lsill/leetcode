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