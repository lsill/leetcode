//
// Created by lsill on 2024/1/17.
//

#include "arr_base.h"
#include <vector>
#include <iostream>

int main()
{
    ArrayMid mid;
    std::vector<int> v  = {4,1,6,5};
    std::cout<<mid.minimumRemoval(v) <<std::endl;
}
