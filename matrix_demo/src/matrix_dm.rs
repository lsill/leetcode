use std::cmp::max;
use std::collections::HashSet;
/// 874. 模拟行走机器人
/// 机器人在一个无限大小的 XY 网格平面上行走，从点(0, 0) 处开始出发，面向北方。该机器人可以接收以下三种类型的命令 commands ：
/// - -2 ：向左转90 度
/// - -1 ：向右转 90 度
/// 1 <= x <= 9 ：向前移动x个单位长度
/// 在网格上有一些格子被视为障碍物obstacles 。第 i个障碍物位于网格点 obstacles[i] = (xi, yi) 。
/// 机器人无法走到障碍物上，它将会停留在障碍物的前一个网格方块上，但仍然可以继续尝试进行该路线的其余部分。
/// 返回从原点到机器人所有经过的路径点（坐标为整数）的最大欧式距离的平方。（即，如果距离为 5 ，则返回 25 ）
/// 注意：
/// - 北表示 +Y 方向。
/// - 东表示 +X 方向。
/// - 南表示 -Y 方向。
/// - 西表示 -X 方向。

// lc题解优秀的解
pub fn robot_sim(commands: Vec<i32>, obstacles: Vec<Vec<i32>>) -> i32 {
    let mut ob_set = obstacles
        .iter()
        .map(|v|(v[0],v[1]))
        .collect::<HashSet<_>>();
    let mut x = 0;
    let mut y = 0;
    let mut dir = (0,1);
    let mut max_dis = 0;
    for com in commands {
        // two-dimensional rotation here is nice
        if com == -1 {
            dir = (dir.1, -dir.0);
        }else if com == -2 {
            dir = (-dir.1, dir.0);
        } else {
            // i didn't expect it to be simulated step by step.
            for _ in 0..com {
                if !ob_set.contains(&(x+dir.0, y + dir.1)) {
                    x += dir.0;
                    y += dir.1;
                }
            }
            max_dis = max_dis.max(x*x + y*y);
        }
    }
    max_dis
}

/// 1267. 统计参与通信的服务器
/// 这里有一幅服务器分布图，服务器的位置标识在 m * n 的整数矩阵网格 grid 中，1 表示单元格上有服务器，0 表示没有。
/// 如果两台服务器位于同一行或者同一列，我们就认为它们之间可以进行通信。
/// 请你统计并返回能够与至少一台其他服务器进行通信的服务器的数量。

// lc 比较好的解
pub fn count_servers(grid: Vec<Vec<i32>>) -> i32 {
    let mut rmp = std::collections::HashMap::new();
    let mut cmp = std::collections::HashMap::new();
    let mut st = std::collections::HashSet::new();
    (0..grid.len()).for_each(|i| (0..grid[0].len()).for_each(|j| {
        if grid[i][j] == 0 { return; }
        rmp.entry(i).or_insert(vec![]).push((i, j));
        cmp.entry(j).or_insert(vec![]).push((i, j));
    }));
    for (_, v) in rmp.into_iter().chain(cmp.into_iter()) {
        if v.len() == 1 { continue; }
        v.into_iter().for_each(|x| { st.insert(x); });
    }
    st.len() as i32
}

/// 1901. 寻找峰值 II
/// 一个 2D 网格中的 峰值 是指那些 严格大于 其相邻格子(上、下、左、右)的元素。
/// 给你一个 从 0 开始编号 的 m x n 矩阵 mat ，其中任意两个相邻格子的值都 不相同 。找出 任意一个 峰值 mat[i][j] 并 返回其位置 [i,j] 。
/// 你可以假设整个矩阵周边环绕着一圈值为 -1 的格子。
/// 要求必须写出时间复杂度为 O(m log(n)) 或 O(n log(m)) 的算法
/// 示例 1:
/// 输入: mat = [[1,4],[3,2]]
/// 输出: [0,1]
/// 解释: 3 和 4 都是峰值，所以[1,0]和[0,1]都是可接受的答案。
/// 示例 2:
/// 输入: mat = [[10,20,15],[21,30,14],[7,16,32]]
/// 输出: [1,1]
/// 解释: 30 和 32 都是峰值，所以[1,1]和[2,2]都是可接受的答案。
/// 提示：
/// m == mat.length
/// n == mat[i].length
/// 1 <= m, n <= 500
/// 1 <= mat[i][j] <= 105
/// 任意两个相邻元素均不相等.

// 自己做的垃圾解
pub fn find_peak_grid_1(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let mut ans:Vec<i32> = Vec::new();
    let m = mat.len();
    let n = mat[0].len();
    for i in 0..m {
       for j in 0..n {
            if i > 0 {
                if mat[i][j] < mat[i-1][j] {
                    continue
                }
            }
           if i < m-1 {
               if mat[i][j] < mat[i+1][j] {
                   continue
               }
           }
           if j > 0 {
               if mat[i][j] < mat[i][j-1] {
                   continue
               }
           }
           if j < n-1 {
               if mat[i][j] < mat[i][j+1] {
                   continue
               }
           }
           ans.push(i as i32);
           ans.push(j as i32);
           break
       }
        if !ans.is_empty() {
            break
        }
    }
    ans
}

//  力扣二分法，左闭右开区间
pub fn find_peak_grid(mat: Vec<Vec<i32>>) -> Vec<i32> {
    let mut left = 0;
    let mut right = mat.len() - 1;
    while left < right {
        let i = left + (right - left) / 2;
        let j = index_of_max(&mat[i]);
        if mat[i][j] > mat[i+1][j] {
            right = i;  // 峰顶行号<=i
        } else {
            left = i + 1;   // 峰顶行号>i
        }
    }
    vec![left as i32, index_of_max(&mat[left]) as i32]
}
fn index_of_max(a:&Vec<i32>)->usize {
    (0..a.len()).max_by_key(|&i| a[i]).unwrap()
}