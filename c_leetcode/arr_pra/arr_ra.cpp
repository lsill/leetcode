//
// Created by lsill on 2022/8/16.
//

#include "arr_ra.h"

OrderedStream::OrderedStream(int n) {
    arr = vector<string>(n+1);
    ptr = 1;
}

vector<string> OrderedStream::insert(int idKey, string value) {
    arr[idKey] = value;
    vector<string> res;
    while (ptr < arr.size() && !arr[ptr].empty()) {
        res.push_back(arr[ptr]);
        ++ptr;
    }
    return res;
}

int MaxEqualFreqClass::maxEqualFreq(vector<int> &nums) {
    unordered_map<int, int> freq, count;
    int res = 0,maxFreq = 0;
    for(int i = 0;i<nums.size();i++){
        if (count[nums[i]] > 0) {
            freq[count[nums[i]]]--;
        }
        count[nums[i]]++;
        maxFreq = max(maxFreq, count[nums[i]]);
        freq[count[nums[i]]]++;
        bool ok = maxFreq == 1 || freq[maxFreq] * maxFreq + freq[maxFreq - 1] * (maxFreq - 1) == i + 1 && freq[maxFreq] == 1 ||
                                  freq[maxFreq] * maxFreq + 1 == i + 1 && freq[1] == 1;
        if (ok){
            res = max(res, i +1);
        }
    }
    return res;
}

int BusyStudent::busyStudent(vector<int> &startTime, vector<int> &endTime, int queryTime) {
    int res = 0;
    for (int i = 0;i < startTime.size();i++){
        if (queryTime >= startTime[i]&& queryTime <= endTime[i]) {
            res++;
        }
    }
    return res;
}
int BusyStudent::maxProduct(vector<int> &nums) {
    int max_0 = 0, max_1 =0;
    for (auto num : nums) {
        if (num > max_0) {
            max_1 = max_0;
            max_0 = num;
            continue;
        }
        if (num > max_1) {
            max_1 = num;
        }
    }
    return (max_0 - 1) * (max_1 - 1);
}