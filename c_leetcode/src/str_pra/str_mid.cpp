//
// Created by lsill on 2024/1/19.
//

#include "strTest.h"
#include <unordered_set>
#include <array>
#include <numeric>
#include <iostream>

// 49. 字母异位词分组
//给你一个字符串数组，请你将 字母异位词 组合在一起。可以按任意顺序返回结果列表。
//字母异位词 是由重新排列源单词的所有字母得到的一个新单词。
//示例 1:
//输入: strs = ["eat", "tea", "tan", "ate", "nat", "bat"]
//输出: [["bat"],["nat","tan"],["ate","eat","tea"]]
//示例 2:
//输入: strs = [""]
//输出: [[""]]
//示例 3:
//输入: strs = ["a"]
//输出: [["a"]]
//提示：
//1 <= strs.length <= 104
//0 <= strs[i].length <= 100
//strs[i] 仅包含小写字母

// 还有排序的做法，不过那个简单
// 计数
vector<vector<string>> StringMid::groupAnagrams(vector<std::string> &strs) {
    auto arrayHash = [fn = hash<int>{}] (const array<int ,26>& arr)->size_t {
        return accumulate(arr.begin(), arr.end(), 0u, [&](size_t acc, int num) {
            return (acc << 1) ^ fn(num);
        });
    };

    unordered_map<array<int, 26>, vector<string>, decltype(arrayHash)> mp(0, arrayHash);
    for (const string& str : strs) {
        array<int ,26> count{};
        const int length = str.length();
        for (int i = 0; i < length;i++) {
            count[str[i] - 'a']++;
        }
        mp[count].emplace_back(str);
    }
    vector<vector<string>> ans;
    for (auto it = mp.begin(); it != mp.end();++it) {
        ans.emplace_back(it->second);
    }
    return ans;
}