//
// Created by lsill on 2022/8/15.
//

#include "strtest1.h"
using namespace std;


int MaxScore::maxScore(string s){
    int max = 0, max_index = 0,tmp = 0;
    for (int i = 1; i <s.length()-1;i++){
        if (s[i] == '0') {
            tmp++;
        } else {
            tmp--;
        }
        if (tmp > max) {
            max = tmp;
            max_index = i;
        }
    }
    int sum = 0;
    for (int i = 0; i <= max_index;i++) {
        if (s[i] == '0') {
            sum++;
        }
    }
    for (int i = max_index+1;i <s.length();i++) {
        if (s[i] == '1') {
            sum++;
        }
    }
    return sum;
}

int MaxScore::maxScore_leetcode(string s) {
    int i = 0,j = 0,ans = 0;
    for (char c : s)
        if (c == '1')
            j++;
    for (int k = 0;k < s.size() - 1;k++)
    {
        if (s[k] == '0')
            i++;
        else
            j--;
        ans = max(ans,i + j);
    }
    return ans;
}