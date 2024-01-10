//
// Created by lsill on 2022/10/31.
//

#include "MathPra.h"

// 122112
int MathPra::magicalString(int n) {
    if (n < 4) {
        return 1;
    }
    string s(n, '0');
    s[0] = '1', s[1] = '2', s[2] = '2';
    int res = 1;
    int i = 2;
    int j = 3;
    while (j < n) {
        int size = s[i] - '0';
        int num = 3 - (s[j - 1] - '0');
        while (size > 0 && j < n) {
            s[j] = '0' + num;
            if (num == 1) {
                ++res;
            }
            ++j;
            --size;
        }
        ++i;
    }
    return res;
}

int MathPra::reachNumber(int target)
{
    target = abs(target);
    int s = 0, n = 0;
    while (s < target || (s - target) % 2) // 没有到达（越过）终点，或者相距奇数
        s += ++n;
    return n;
}