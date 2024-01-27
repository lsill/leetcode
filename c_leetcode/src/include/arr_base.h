//
// Created by lsill on 2024/1/17.
//

#ifndef LC_ARR_BASE_H
#define LC_ARR_BASE_H

#include <vector>

class ArrayMid {
public:
    int longestConsecutiveSelf(std::vector<int>& nums);
    int longestConsecutiveHash(std::vector<int>& nums);
    int longestConsecutiveSort(std::vector<int>& nums);
    int longestConsecutiveLikeDp(std::vector<int>& nums);
    long long minimumRemoval(std::vector<int>& beans);
    int maximumSwap(int num);
};


class ArraySimple {
public:
    int alternatingSubarraySelf(std::vector<int>& nums);
    int alternatingSubarray(std::vector<int>& nums);
};

#endif //LC_ARR_BASE_H
