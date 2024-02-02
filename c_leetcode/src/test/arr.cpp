//
// Created by lsill on 2024/1/17.
//

#include "arr_base.h"
#include <vector>
#include <iostream>

using namespace std;

int main()
{
    ArrayMid mid;
    vector<int> alice {2,4,3};
    vector<int> bob {1,6,7};
    cout << mid.stoneGameVI(alice, bob) << endl;
}
