//
// Created by lsill on 2022/10/31.
//

#ifndef NEW_C_STUDY_MATHPRA_H
#define NEW_C_STUDY_MATHPRA_H

#include <unordered_map>
#include <string>
using namespace std;

class MathPra {
public:
    int magicalString(int n);
    int reachNumber(int target);
    int addMinimum(string word);
    int addMinimum_1(string word);
};

class MathMid {
public:
    bool canMeasureWater(int jug1Capacity, int jug2Capacity, int targetCapacity);
};

class MathSimple {
public:
    bool canWinNim(int n);
};

#endif //NEW_C_STUDY_MATHPRA_H
