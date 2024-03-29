//
// Created by lsill on 2024/1/16.
//

#ifndef LC_DP_BASE_H
#define LC_DP_BASE_H
#include <string>
#include <vector>
#ifdef  __linux__
#include <cstring>
#include <functional>
#endif
class DpHard {
private:
    long long solve(std::string num, int m);
    void prepare();
    const int MOD = 10e9 + 7;
    long long f[30][500], g[30][500];
public:
    int countLc(std::string num1, std::string num2, int min_sum, int max_sum);
    int countBest(std::string num1, std::string num2, int min_sum, int max_sum);
    int findRotateSteps(std::string ring, std::string key);
};

#endif //LC_DP_BASE_H
