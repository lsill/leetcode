//
// Created by lsill on 2024/1/17.
//

#ifndef LC_ARR_BASE_H
#define LC_ARR_BASE_H

#include <vector>
#include <unordered_map>
#include <unordered_set>
#include <numeric>

class ArrayMid {
public:
    int longestConsecutiveSelf(std::vector<int>& nums);
    int longestConsecutiveHash(std::vector<int>& nums);
    int longestConsecutiveSort(std::vector<int>& nums);
    int longestConsecutiveLikeDp(std::vector<int>& nums);
    long long minimumRemoval(std::vector<int>& beans);
    int maximumSwap(int num);
    int minimumSeconds(std::vector<int>& nums);
    int stoneGameVI(std::vector<int>& aliceValues, std::vector<int>& bobValues);
    int stoneGameVISelf(std::vector<int>& aliceValues, std::vector<int>& bobValues);
};


class ArraySimple {
public:
    int alternatingSubarraySelf(std::vector<int>& nums);
    int alternatingSubarray(std::vector<int>& nums);
    std::vector<int> distinctDifferenceArraySelf(std::vector<int>& nums);
    std::vector<int> distinctDifferenceArray(std::vector<int>& nums);
};
#endif //LC_ARR_BASE_H
