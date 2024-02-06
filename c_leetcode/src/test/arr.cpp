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
    vector<int> alice {-200,-300,400,9};
    cout << mid.magicTower(alice) << endl;
}
