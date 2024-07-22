/*
 * @Author: lsill
 * @Date: 2024-07-22 11:51:57
 */
#include "floyd_base.h"
#include <bitset>
#include <ctime>
#include <numeric>
#include <queue>

// 2101. 引爆最多的炸弹
// 给你一个炸弹列表。一个炸弹的 爆炸范围 定义为以炸弹为圆心的一个圆。
// 炸弹用一个下标从 0 开始的二维整数数组 bombs 表示，其中 bombs[i] = [xi, yi, ri] 。xi 和 yi 表示第 i 个炸弹的 X 和 Y 坐标，ri 表示爆炸范围的 半径 。
// 你需要选择引爆 一个 炸弹。当这个炸弹被引爆时，所有 在它爆炸范围内的炸弹都会被引爆，这些炸弹会进一步将它们爆炸范围内的其他炸弹引爆。
// 给你数组 bombs ，请你返回在引爆 一个 炸弹的前提下，最多 能引爆的炸弹数目。

// lc 优秀解
int FloydMid::maximumDetonation(vector<vector<int>> &bombs) {
        int n = bombs.size();
        vector<bitset<100>> f(n);
        for (int i = 0; i < n; i++) {
            long long x = bombs[i][0], y = bombs[i][1], r = bombs[i][2];
            for (int j = 0; j < n; j++) {
                long long dx = x - bombs[j][0];
                long long dy = y - bombs[j][1];
                if (dx * dx + dy * dy <= r * r) {
                    f[i].set(j); // i 可以到达 j
                }
            }
        }

        for (int k = 0; k < n; k++) {
            for (auto& fi : f) {
                if (fi.test(k)) { // i 可以到达 k
                    fi |= f[k]; // i 也可以到 k 可以到达的点
                }
            }
        }

        size_t ans = 0;
        for (auto& s : f) {
            ans = max(ans, s.count()); // 集合大小的最大值
        }
        return ans;
}