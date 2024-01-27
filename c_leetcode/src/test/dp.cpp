//
// Created by lsill on 2024/1/16.
//
#include "dp_base.h"
#include <iostream>

int main(){
    DpHard* dp = new DpHard;
    std::cout<< dp->countLc("1", "12", 1,8) << std::endl;
    delete dp;
}