//
// Created by lsill on 2024/1/25.
//
#include "binary_base.h"
#include <iostream>
#include <vector>

using  namespace std;

int main(){
    BinarySimple* dp = new BinarySimple;
    vector<int> nums{5,10,1,5,2};
    cout << dp->sumIndicesWithSetBits(nums, 1) << endl;
    delete dp;
}