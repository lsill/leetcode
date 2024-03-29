//
// Created by lsill on 2024/1/11.
//

#ifndef LC_STACK_BASE_H
#define LC_STACK_BASE_H

#include <string>
#include <vector>
#include <stack>
using namespace std;

class StringStack {
public:
    int minLength(string s);
};

class MonotonicStack {
public:
    long long maximumSumOfHeights(vector<int>& maxHeights);
};

#endif //LC_STACK_BASE_H
