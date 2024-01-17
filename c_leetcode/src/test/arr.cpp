//
// Created by lsill on 2024/1/17.
//

#include "arr_base.h"
#include <vector>
#include <iostream>

int main()
{
    ArrayMid mid;
    std::vector<int> v  = {100,4,200,1,3,2};
    std::cout<<mid.longestConsecutiveLikeDp(v) <<std::endl;
}
