use std::collections::HashMap;

/// 2477. 到达首都的最少油耗
/// 给你一棵 n 个节点的树（一个无向、连通、无环图），每个节点表示一个城市，编号从 0 到 n - 1 ，且恰好有 n - 1 条路。
/// 0 是首都。给你一个二维整数数组 roads ，其中 roads[i] = [ai, bi] ，表示城市 ai 和 bi 之间有一条 双向路 。
/// 每个城市里有一个代表，他们都要去首都参加一个会议。
/// 每座城市里有一辆车。给你一个整数 seats 表示每辆车里面座位的数目。
/// 城市里的代表可以选择乘坐所在城市的车，或者乘坐其他城市的车。相邻城市之间一辆车的油耗是一升汽油。
/// 请你返回到达首都最少需要多少升汽油。
/// 示例 1：
/// 输入：roads = [[0,1],[0,2],[0,3]], seats = 5
/// 输出：3
/// 解释：
/// - 代表 1 直接到达首都，消耗 1 升汽油。
/// - 代表 2 直接到达首都，消耗 1 升汽油。
/// - 代表 3 直接到达首都，消耗 1 升汽油。
/// 最少消耗 3 升汽油。
/// 示例 2：
/// 输入：roads = [[3,1],[3,2],[1,0],[0,4],[0,5],[4,6]], seats = 2
/// 输出：7
/// 解释：
/// - 代表 2 到达城市 3 ，消耗 1 升汽油。
/// - 代表 2 和代表 3 一起到达城市 1 ，消耗 1 升汽油。
/// - 代表 2 和代表 3 一起到达首都，消耗 1 升汽油。
/// - 代表 1 直接到达首都，消耗 1 升汽油。
/// - 代表 5 直接到达首都，消耗 1 升汽油。
/// - 代表 6 到达城市 4 ，消耗 1 升汽油。
/// - 代表 4 和代表 6 一起到达首都，消耗 1 升汽油。
/// 最少消耗 7 升汽油。
/// 示例 3：
/// 输入：roads = [], seats = 1
/// 输出：0
/// 解释：没有代表需要从别的城市到达首都。
/// 提示：
/// 1 <= n <= 105
/// roads.length == n - 1
/// roads[i].length == 2
/// 0 <= ai, bi < n
/// ai != bi
/// roads 表示一棵合法的树。
/// 1 <= seats <= 105
///
/// 图解 ![](https://raw.githubusercontent.com/lsill/gitLink/main/document/photo/leetcode/2477_0.jpeg)
// 力扣题解
pub fn minimum_fuel_cost(roads: Vec<Vec<i32>>, seats: i32) -> i64 {
    let mut g = vec![vec![]; roads.len() + 1];
    for e in &roads {
        let x = e[0] as usize;
        let y = e[1] as usize;
        g[x].push(y);   // 记录每个点的邻居
        g[y].push(x);
    }
    let mut ans = 0i64;
    minimum_fuel_cost_dfs(0,0,&g, seats,&mut ans);
    ans
}

fn minimum_fuel_cost_dfs(x:usize, fa:usize, g:&Vec<Vec<usize>>, seats:i32, ans:&mut i64) -> i32 {
    let mut size = 1;
    for &y in &g[x] {
        if y != fa {    // 递归子节点，不能递归父节点
            size += minimum_fuel_cost_dfs(y, x, g, seats, ans); // 统计子树大小
        }
    }
    if x != 0 { // x非根节点
        *ans += ((size - 1)/ seats + 1) as i64;
    }
    size
}


/// 1466. 重新规划路线
/// n 座城市，从 0 到 n-1 编号，其间共有 n-1 条路线。因此，要想在两座不同城市之间旅行只有唯一一条路线可供选择（路线网形成一颗树）。去年，交通运输部决定重新规划路线，以改变交通拥堵的状况。
/// 路线用 connections 表示，其中 connections[i] = [a, b] 表示从城市 a 到 b 的一条有向路线。
/// 今年，城市 0 将会举办一场大型比赛，很多游客都想前往城市 0 。
/// 请你帮助重新规划路线方向，使每个城市都可以访问城市 0 。返回需要变更方向的最小路线数。
/// 题目数据 保证 每个城市在重新规划路线方向后都能到达城市 0 。
/// 示例 1：
/// 输入：n = 6, connections = [[0,1],[1,3],[2,3],[4,0],[4,5]]
/// 输出：3
/// 解释：更改以红色显示的路线的方向，使每个城市都可以到达城市 0 。
/// 示例 2：
/// 输入：n = 5, connections = [[1,0],[1,2],[3,2],[3,4]]
/// 输出：2
/// 解释：更改以红色显示的路线的方向，使每个城市都可以到达城市 0 。
/// 示例 3：
/// 输入：n = 3, connections = [[1,0],[2,0]]
/// 输出：0
/// 提示：
/// 2 <= n <= 5 * 10^4
/// connections.length == n-1
/// connections[i].length == 2
/// 0 <= connections[i][0], connections[i][1] <= n-1
/// connections[i][0] != connections[i][1]

// 力扣题解dfs
pub fn min_reorder(n: i32, connections: Vec<Vec<i32>>) -> i32 {
    let mut g: Vec<Vec<(i32, i32)>> = vec![vec![]; n as usize];
    for e in connections.iter() {
        let a = e[0] as usize;
        let b = e[1] as usize;
        g[a].push((b as i32, 1));
        g[b].push((a as i32, 0));
    }
    fn dfs(a: usize, fa: i32, g: &Vec<Vec<(i32, i32)>>) -> i32 {
        let mut ans = 0;
        for &(b, c) in g[a].iter() {
            if b != fa {
                ans += c + dfs(b as usize, a as i32, g);
            }
        }
        ans
    }
    dfs(0, -1, &g)
}














