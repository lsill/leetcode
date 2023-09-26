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
        return ways_to_buy_pens_pencils(total, cost2, cost1)
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

/// 2582. 递枕头
/// n 个人站成一排，按从 1 到 n 编号。
/// 最初，排在队首的第一个人拿着一个枕头。每秒钟，拿着枕头的人会将枕头传递给队伍中的下一个人。一旦枕头到达队首或队尾，传递方向就会改变，队伍会继续沿相反方向传递枕头。
/// 例如，当枕头到达第 n 个人时，TA 会将枕头传递给第 n - 1 个人，然后传递给第 n - 2 个人，依此类推。
/// 给你两个正整数 n 和 time ，返回 time 秒后拿着枕头的人的编号。
pub fn pass_the_pillow(n: i32, time: i32) -> i32 {
    let round = time / (n - 1);
    let step = time % (n - 1);

    if round & 1 == 1 {
        n - step
    } else {
        1 + step
    }
}