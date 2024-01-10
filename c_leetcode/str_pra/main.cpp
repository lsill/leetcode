//
// Created by lsill on 2024/1/10.
//
#include<iostream>
#include "strTest.h"
using namespace std;

int main()
{
    cout<<"Hello NEW C STUDY"<<endl;
    StringPra* str = new StringPra();
    cout << str->minLength("ABFCACDB") << endl;
    delete str;
    return 0;
}