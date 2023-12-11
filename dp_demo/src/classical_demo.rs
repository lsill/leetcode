use std::cmp;
use std::collections::BinaryHeap;
/// 931. 下降路径最小和
/// 给你一个 n x n 的 方形 整数数组matrix ，请你找出并返回通过 matrix 的下降路径 的 最小和 。
/// 下降路径 可以从第一行中的任何元素开始，并从每一行中选择一个元素。
/// 在下一行选择的元素和当前行所选元素最多相隔一列（即位于正下方或者沿对角线向左或者向右的第一个元素）。
/// 具体来说，位置 (row, col) 的下一个元素应当是 (row + 1, col - 1)、(row + 1, col) 或者 (row + 1, col + 1) 。

/// 提示
/// n == matrix.length == matrix[i].length
/// 1 <= n <= 100
/// -100 <= matrix[i][j] <= 100
/// [来源](https://leetcode.cn/problems/minimum-falling-path-sum/)
// lc看不懂的解
pub fn min_falling_path_sum(mut matrix: Vec<Vec<i32>>) -> i32 {
    let (m, n) = (matrix.len(), matrix[0].len());
    for i in (1..m) {
        (0..n).for_each(|j| {
            matrix[i][j] += *matrix[i - 1][(j.max(1) - 1)..(j.min(n - 2) + 2)]
                .iter()
                .min()
                .unwrap()
        });
    }
    *matrix[m - 1].iter().min().unwrap()
}

// lc执行最优解
pub fn min_falling_path_sum_1(matrix: Vec<Vec<i32>>) -> i32 {
    let m=matrix.len();
    let mut res=i32::MAX;
    let mut memo:Vec<Vec<i32>>=vec![vec![66666;m];m];
    for i in 0..m{
        res=cmp::min(res,min_falling_path_sum_1_dp(&matrix,m-1,i,&mut memo));
    }
    res
}

fn min_falling_path_sum_1_dp(matrix:&Vec<Vec<i32>>,i:usize,j:usize,memo:&mut Vec<Vec<i32>>)->i32{
    if i<0 || j<0 || i>=matrix.len() || j>=matrix[0].len(){
        return 99999;
    }
    if i==0{
        return matrix[i][j];
    }
    if memo[i][j]!=66666{
        return memo[i][j];
    }
    memo[i][j]=cmp::min(memo(matrix,i-1,j,memo),cmp::min(memo(matrix,i-1,j-1,memo),memo(matrix,i-1,j+1,memo)))+matrix[i][j];
    return memo[i][j];
}

/// 2008. 出租车的最大盈利
/// 你驾驶出租车行驶在一条有 n 个地点的路上。这 n 个地点从近到远编号为 1 到 n ，你想要从 1 开到 n ，通过接乘客订单盈利。你只能沿着编号递增的方向前进，不能改变方向。
/// 乘客信息用一个下标从 0 开始的二维数组 rides 表示，其中 rides[i] = [starti, endi, tipi] 表示第 i 位乘客需要从地点 starti 前往 endi ，愿意支付 tipi 元的小费。
/// 每一位 你选择接单的乘客 i ，你可以 盈利 endi - starti + tipi 元。你同时 最多 只能接一个订单。
/// 给你 n 和 rides ，请你返回在最优接单方案下，你能盈利 最多 多少元。
/// 注意：你可以在一个地点放下一位乘客，并在同一个地点接上另一位乘客。
/// 示例 1：
///
/// 输入：n = 5, rides = [[2,5,4],[1,5,1]]
/// 输出：7
/// 解释：我们可以接乘客 0 的订单，获得 5 - 2 + 4 = 7 元。
/// 示例 2：
/// 输入：n = 20, rides = [[1,6,1],[3,10,2],[10,12,3],[11,12,2],[12,15,2],[13,18,1]]
/// 输出：20
/// 解释：我们可以接以下乘客的订单：
/// - 将乘客 1 从地点 3 送往地点 10 ，获得 10 - 3 + 2 = 9 元。
/// - 将乘客 2 从地点 10 送往地点 12 ，获得 12 - 10 + 3 = 5 元。
/// - 将乘客 5 从地点 13 送往地点 18 ，获得 18 - 13 + 1 = 6 元。
/// 我们总共获得 9 + 5 + 6 = 20 元。
/// 提示：
/// 1 <= n <= 105
/// 1 <= rides.length <= 3 * 104
/// rides[i].length == 3
/// 1 <= starti < endi <= n
/// 1 <= tipi <= 105
// 力扣动态规划做法
pub fn max_taxi_earnings(n: i32, mut rides: Vec<Vec<i32>>) -> i64 {
    let n = n as usize;
    let mut groups : Vec<Vec<(i32, i32)>> = vec![vec![];(n+1)];
    for r in &rides {
        let start = r[0];
        let end = r[1];
        let tip = r[2];
        groups[end as usize].push((start, end - start + tip));
    }
    let mut f:Vec<i64>= vec![0; (n+1)];
    for i in 2..=n {
        f[i] = f[i-1];
        for &(s, t) in &groups[i] {
            f[i] = f[i].max(f[s as usize] + t as i64);
        }
    }
    f[n]
}

// 递归+ 记录返回值 = 记忆化搜索
pub fn max_taxi_earnings_1(n: i32, rides: Vec<Vec<i32>>) -> i64 {
    let n = n as usize;
    let mut groups: Vec<Vec<(i32, i32)>> = vec![vec![]; (n + 1)];
    for r in &rides {
        let start = r[0];
        let end = r[1];
        let tip = r[2];
        groups[end as usize].push((start, end - start + tip));
    }

    let mut memo = vec![-1i64; n + 1]; // -1 表示没有计算过
    fn dfs(i: usize, memo: &mut Vec<i64>, groups: &Vec<Vec<(i32, i32)>>) -> i64 {
        if i == 1 {
            return 0;
        }
        if memo[i] != -1 { // 之前计算过
            return memo[i];
        }
        let mut res = dfs(i - 1, memo, groups);
        for &(s, t) in &groups[i] {
            res = res.max(dfs(s as usize, memo, groups) + t as i64);
        }
        memo[i] = res; // 记忆化
        res
    }
    dfs(n, &mut memo, &groups)
}


/// 1631. 最小体力消耗路径
/// 你准备参加一场远足活动。给你一个二维 rows x columns 的地图 heights ，其中 heights[row][col] 表示格子 (row, col) 的高度。
/// 一开始你在最左上角的格子 (0, 0) ，且你希望去最右下角的格子 (rows-1, columns-1) （注意下标从 0 开始编号）。你每次可以往 上，下，左，右 四个方向之一移动，你想要找到耗费 体力 最小的一条路径。
/// 一条路径耗费的 体力值 是路径上相邻格子之间 高度差绝对值 的 最大值 决定的。
/// 请你返回从左上角走到右下角的最小 体力消耗值 。
/// 示例 1：
/// 输入：heights = [[1,2,2],[3,8,2],[5,3,5]]
/// 输出：2
/// 解释：路径 [1,3,5,3,5] 连续格子的差值绝对值最大为 2 。
/// 这条路径比路径 [1,2,2,2,5] 更优，因为另一条路径差值最大值为 3 。
/// 示例 2：
/// 输入：heights = [[1,2,3],[3,8,4],[5,3,5]]
/// 输出：1
/// 解释：路径 [1,2,3,4,5] 的相邻格子差值绝对值最大为 1 ，比路径 [1,3,5,3,5] 更优。
/// 示例 3：
/// 输入：heights = [[1,2,1,1,1],[1,2,1,2,1],[1,2,1,2,1],[1,2,1,2,1],[1,1,1,2,1]]
/// 输出：0
/// 解释：上图所示路径不需要消耗任何体力。
/// 提示：
/// rows == heights.length
/// columns == heights[i].length
/// 1 <= rows, columns <= 100
/// 1 <= heights[i][j] <= 106

// 力扣符合rust的解
#[derive(PartialEq, Eq)]
struct Item (i32,i32,i32);

// 实现 Ord 特性用于比较内层 Vec 的第三个元素
impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.2.cmp(&self.2)
    }
}

// 实现 PartialOrd 特性用于比较内层 Vec 的第三个元素
impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub fn minimum_effort_path(heights: Vec<Vec<i32>>) -> i32 {
    let n = heights.len();
    if n == 0 { return 0;}
    let m = heights[0].len();
    if m == 0 { return 0; }
    let mut d = vec![vec![std::i32::MAX; m]; n];
    let mut seen = vec![vec![false; m]; n];
    let mut heap = BinaryHeap::new();
    let dirs: Vec<i32> = vec![0, 1, 0, -1, 0];
    heap.push(Item(0,0,0));
    d[0][0] = 0;
    while let Some(cur) = heap.pop(){
        if cur.0 == ((n-1) as i32) && cur.1 == ((m-1) as i32){
            return cur.2;
        }
        if seen[cur.0 as usize][cur.1 as usize]{
            continue;
        }
        seen[cur.0 as usize][cur.1 as usize] = true;
        dirs.windows(2).for_each(|dir|{
            let nx = cur.0 + dir[0];
            let ny = cur.1 + dir[1];
            if 0<=nx && nx<(n as i32) && 0<=ny && ny<(m as i32){
                let nd = std::cmp::max(cur.2,
                                       (heights[nx as usize][ny as usize] -
                                           heights[cur.0 as usize][cur.1 as usize]).abs()
                );
                if nd < d[nx as usize][ny as usize]{
                    d[nx as usize][ny as usize] = nd;
                    heap.push(Item(nx as i32,ny as i32,nd));
                }
            }
        });
    }
    0i32
}

