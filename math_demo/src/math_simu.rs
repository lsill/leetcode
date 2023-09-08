/// 2240. 买钢笔和铅笔的方案数
/// 给你一个整数 total ，表示你拥有的总钱数。同时给你两个整数 cost1 和 cost2 ，分别表示一支钢笔和一支铅笔的价格。你可以花费你部分或者全部的钱，去买任意数目的两种笔。
/// 请你返回购买钢笔和铅笔的 不同方案数目 。
// lc 自己解
pub fn ways_to_buy_pens_pencils(total: i32, cost1: i32, cost2: i32) -> i64 {
    let mut ans:i64 = 0;
    let mut pen:i32 = 0;
    let c1 = cost1.max(cost2);
    let c2 = cost1.min(cost2);
    while total >= c1 * pen {
        let pencil = (total - pen * c1) / c2;
        ans += pencil as i64 + 1;
        pen += 1;
    }
    ans
}

// lc 比较优秀的解
pub fn ways_to_buy_pens_pencils_1(total: i32, cost1: i32, cost2: i32) -> i64 {
    if cost1 < cost2 {
        return Self::ways_to_buy_pens_pencils(total, cost2, cost1)
    }
    let mut res = 0_i64;
    let mut total = total;
    while total >= 0 {
        res += (total / cost2 + 1) as i64;
        total -= cost1;
    }
    res
}

/// 2651. 计算列车到站时间
/// 给你一个正整数 arrivalTime 表示列车正点到站的时间（单位：小时），另给你一个正整数 delayedTime 表示列车延误的小时数。
/// 返回列车实际到站的时间。
/// 注意，该问题中的时间采用 24 小时制。

pub fn find_delayed_arrival_time(arrival_time: i32, delayed_time: i32) -> i32 {
    (arrival_time + delayed_time) % 24
}