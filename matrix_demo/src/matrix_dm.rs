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

