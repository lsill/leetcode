//
// Created by lsill on 2024/1/17.
//

#include "arr_base.h"
#include <vector>
#include <iostream>

int main()
{
    ArraySimple mid;
    std::vector<int > a {14,30,29,49,3,23,44,21,26,52};
    std::cout<<mid.alternatingSubarray(a) <<std::endl;
}
