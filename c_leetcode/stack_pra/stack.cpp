//
// Created by lsill on 2024/1/11.
//
#include<iostream>
#include "stack_base.h"
using namespace std;

int main()
{
    StringStack* st = new StringStack();
    cout <<st->addMinimum("b") <<endl;
    delete st;
    return 0;
}