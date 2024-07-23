//
// Created by lsill on 2024/1/16.
//

#ifndef LC_DP_BASE_H
#define LC_DP_BASE_H
#include <string>
#include <vector>
using namespace std;
#ifdef  __linux__
#include <cstring>
#include <functional>
#endif
class DpHard {
protected:
    static constexpr int mod = 1e9 + 7;
    static constexpr int inf = 0x3f3f3f3f;
private:
    long long solve(std::string num, int m);
    void prepare();
    const int MOD = 10e9 + 7;
    long long f[30][500], g[30][500];
public:
    int countLc(std::string num1, std::string num2, int min_sum, int max_sum);
    int countBest(std::string num1, std::string num2, int min_sum, int max_sum);
    int findRotateSteps(std::string ring, std::string key);
    int sumOfPowers(std::vector<int>& nums, int k);
};

#endif //LC_DP_BASE_H
