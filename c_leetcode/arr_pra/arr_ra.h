//
// Created by lsill on 2022/8/16.
//

#ifndef C_LEETCODE_ARR_RA_H
#define C_LEETCODE_ARR_RA_H
#include <string>
#include <vector>
#include <unordered_map>
using namespace std;


class OrderedStream {
private:
    int ptr;
    vector<string> arr;
public:
    OrderedStream(int n);

    vector<string> insert(int idKey, string value);
};

class MaxEqualFreqClass {
public:
    int maxEqualFreq(vector<int>& nums);
};

class BusyStudent {
public:
    int busyStudent(vector<int>& startTime, vector<int>& endTime, int queryTime);
    int maxProduct(vector<int>& nums);
};

#endif //C_LEETCODE_ARR_RA_H
