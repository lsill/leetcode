//
// Created by lsill on 2022/8/15.
//

#ifndef C_LEETCODE_DEGISN1_H
#define C_LEETCODE_DEGISN1_H
#include<vector>
using namespace std;

class MyCircularDeque {
private:
    vector<int> elements;
    int tail, front;
    int capacity;
public:
    MyCircularDeque(int k);

    bool insertFront(int value);

    bool insertLast(int value);

    bool deleteFront();

    bool deleteLast();

    int getFront();

    int getRear();

    bool isEmpty();

    bool isFull();
};

#endif //C_LEETCODE_DEGISN1_H
