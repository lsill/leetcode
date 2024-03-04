//
// Created by lsill on 2024/3/4.
//

#ifndef LC_SIMULATION_BASE_H
#define LC_SIMULATION_BASE_H
#include <stack>

class MyQueue {
public:
    MyQueue();
    void push(int x);
    int pop();
    int peek();
    bool empty();
private:
    std::stack<int> inStack, outStack;
    void in2out();
};

#endif //LC_SIMULATION_BASE_H
