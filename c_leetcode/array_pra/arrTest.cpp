//
// Created by lsill on 2022/10/27.
//

#include "arrTest.h"

int arrTest::arraySign(vector<int>& nums)
{
    int product = 1;
    for (int i = 0; i < nums.size();i++)
    {
        if (nums[i] < 0) {
            product = -product;
        }
        if (nums[i] == 0)
        {
            return 0;
        }
    }
    return product;
}

int arrTest::arraySignOther(vector<int>& nums)
{
    int product=1,i=0;
    while(i<nums.size())
    {
        if(nums[i]==0)
            return 0;
        product*=nums[i++]>0?1:-1;
    }
    return product;
}

int arrTest::sumSubarrayMins(vector<int> &arr)
{
    int n = arr.size();
    vector<int> monoStack;
    vector<int> left(n), right(n);
    for (int i = 0; i < n; i++) {
        while (!monoStack.empty() && arr[i] <= arr[monoStack.back()]) {
            monoStack.pop_back();
        }
        left[i] = i - (monoStack.empty() ? -1 : monoStack.back());
        monoStack.emplace_back(i);
    }
    monoStack.clear();
    for (int i = n - 1; i >= 0; i--) {
        while (!monoStack.empty() && arr[i] < arr[monoStack.back()]) {
            monoStack.pop_back();
        }
        right[i] = (monoStack.empty() ? n : monoStack.back()) - i;
        monoStack.emplace_back(i);
    }
    long long ans = 0;
    long long mod = 1e9 + 7;
    for (int i = 0; i < n; i++) {
        ans = (ans + (long long)left[i] * right[i] * arr[i]) % mod;
    }
    return ans;
}

