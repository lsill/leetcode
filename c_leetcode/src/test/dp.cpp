//
// Created by lsill on 2024/1/16.
//
#include "dp_base.h"
#include <iostream>
using namespace std;

int main(){
    DpHard* dp = new DpHard;
    vector<int> arr {1,2,3,4};
    std::cout<< dp->sumOfPowers(arr, 3) << std::endl;
    delete dp;
}