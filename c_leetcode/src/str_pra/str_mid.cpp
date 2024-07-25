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

// 2844. 生成特殊数字的最少操作
// 给你一个下标从0开始的字符串 num，表示一个非负整数。
// 在一次操作中，您可以选择 num 的任意一位数字并将其删除。请注意，
// 如果你删除 num 中的所有数字，则 num 变为0。返回最少需要多少次操作可以使 num变成特殊数字。
// 如果整数 x 能被25整除，则该整数 x被认为是特殊数字。
// 示例1：输入：num="2245047"输出：2解释：删除数字 num[5] 和 num[6] ，得到数字
// "22450"，可以被25整除。可以证明要使数字变成特殊数字，最少需要删除2位数字。
// 示例2：输入：num="2908305"输出：3解释：删除 num[3]、 num[4] 和 num[6] ，得到数字
// "2900"，可以被25整除。可以证明要使数字变成特殊数字，最少需要删除3位数字。
// 示例3：输入：num="10"输出：1解释：删除 num[0] ，得到数字"0"，可以被
// 25整除。可以证明要使数字变成特殊数字，最少需要删除1位数字。
// 提示1 <=num.length<=100 num 仅由数字'0'到'9'组成num 不含任何前导零

// lc优秀解（要考虑到个位为'0'和个位为'5'的情况）
int StringMid::minimumOperations(string num) {
    int n = num.length();
    bool found0 = false, found5 = false;
    for (int i = n - 1; i >= 0; i--) {
        char c = num[i];
        if (found0 && (c == '0' || c == '5') ||
            found5 && (c == '2' || c == '7')) {
            return n - i - 2;
            }
        if (c == '0') {
            found0 = true;
        } else if (c == '5') {
            found5 = true;
        }
    }
    return n - found0;
}
