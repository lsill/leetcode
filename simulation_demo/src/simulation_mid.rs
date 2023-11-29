use std::{
    cmp::Reverse,
    collections::{BTreeMap, HashMap}
};
use std::arch::aarch64::vaddlv_s16;
use std::collections::{BinaryHeap, BTreeSet, HashSet};

/// 2034. 股票价格波动
/// 给你一支股票价格的数据流。数据流中每一条记录包含一个 时间戳 和该时间点股票对应的 价格 。
/// 不巧的是，由于股票市场内在的波动性，股票价格记录可能不是按时间顺序到来的。
/// 某些情况下，有的记录可能是错的。如果两个有相同时间戳的记录出现在数据流中，前一条记录视为错误记录，后出现的记录 更正 前一条错误的记录。
/// 请你设计一个算法，实现：
/// 更新 股票在某一时间戳的股票价格，如果有之前同一时间戳的价格，这一操作将 更正 之前的错误价格。
/// 找到当前记录里 最新股票价格 。最新股票价格 定义为时间戳最晚的股票价格。
/// 找到当前记录里股票的 最高价格 。
/// 找到当前记录里股票的 最低价格 。
/// 请你实现 StockPrice 类：
///
/// StockPrice() 初始化对象，当前无股票价格记录。
/// void update(int timestamp, int price) 在时间点 timestamp 更新股票价格为 price 。
/// int current() 返回股票 最新价格 。
/// int maximum() 返回股票 最高价格 。
/// int minimum() 返回股票 最低价格 。

struct StockPrice {
    timestamp_to_price: HashMap<i32, i32>,
    price_to_cnt: BTreeMap<i32, i32>,
    max_timestamp: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockPrice {
    fn new() -> Self {
        StockPrice {
            timestamp_to_price: HashMap::new(),
            price_to_cnt: BTreeMap::new(),
            max_timestamp: 0,
        }
    }

    fn update(&mut self, timestamp: i32, price: i32) {
        self.max_timestamp = self.max_timestamp.max(timestamp);
        if self.timestamp_to_price.contains_key(&timestamp) {
            if let Some(prev_price) = self.timestamp_to_price.get(&timestamp) {
                if let Some(prev_cnt) = self.price_to_cnt.get(prev_price) {
                    if *prev_cnt == 1 { self.price_to_cnt.remove(&prev_price); } else { self.price_to_cnt.insert(*prev_price, *prev_cnt - 1); }
                }
            }
        }
        self.timestamp_to_price.insert(timestamp, price);
        *self.price_to_cnt.entry(price).or_insert(0) += 1;
    }

    fn current(&self) -> i32 {
        *self.timestamp_to_price.get(&self.max_timestamp).unwrap()
    }

    fn maximum(&self) -> i32 {
        *self.price_to_cnt.iter().rev().next().unwrap().0
    }

    fn minimum(&self) -> i32 {
        *self.price_to_cnt.iter().next().unwrap().0
    }
}

/**
 * Your StockPrice object will be instantiated and called as such:
 * let obj = StockPrice::new();
 * obj.update(timestamp, price);
 * let ret_2: i32 = obj.current();
 * let ret_3: i32 = obj.maximum();
 * let ret_4: i32 = obj.minimum();
 */


/// 2336. 无限集中的最小数字
/// 现有一个包含所有正整数的集合 [1, 2, 3, 4, 5, ...] 。
/// 实现 SmallestInfiniteSet 类：
/// SmallestInfiniteSet() 初始化 SmallestInfiniteSet 对象以包含 所有 正整数。
/// int popSmallest() 移除 并返回该无限集中的最小整数。
/// void addBack(int num) 如果正整数 num 不 存在于无限集中，则将一个 num 添加 到该无限集最后。
///
/// 示例：
/// 输入
/// ["SmallestInfiniteSet", "addBack", "popSmallest", "popSmallest", "popSmallest", "addBack", "popSmallest", "popSmallest", "popSmallest"]
/// [[], [2], [], [], [], [1], [], [], []]
/// 输出
/// [null, null, 1, 2, 3, null, 1, 4, 5]
///
/// 解释
/// SmallestInfiniteSet smallestInfiniteSet = new SmallestInfiniteSet();
/// smallestInfiniteSet.addBack(2);    // 2 已经在集合中，所以不做任何变更。
/// smallestInfiniteSet.popSmallest(); // 返回 1 ，因为 1 是最小的整数，并将其从集合中移除。
/// smallestInfiniteSet.popSmallest(); // 返回 2 ，并将其从集合中移除。
/// smallestInfiniteSet.popSmallest(); // 返回 3 ，并将其从集合中移除。
/// smallestInfiniteSet.addBack(1);    // 将 1 添加到该集合中。
/// smallestInfiniteSet.popSmallest(); // 返回 1 ，因为 1 在上一步中被添加到集合中，
///                                    // 且 1 是最小的整数，并将其从集合中移除。
/// smallestInfiniteSet.popSmallest(); // 返回 4 ，并将其从集合中移除。
/// smallestInfiniteSet.popSmallest(); // 返回 5 ，并将其从集合中移除。
///
/// 提示：
/// 1 <= num <= 1000
/// 最多调用 popSmallest 和 addBack 方法 共计 1000 次

// 自己做的垃圾
struct SmallestInfiniteSet1 {
    kv:HashMap<i32, bool>
}

impl SmallestInfiniteSet1 {
    fn new() -> Self {
        SmallestInfiniteSet1{ kv: HashMap::new() }
    }

    fn pop_smallest(&mut self) ->i32 {
        let mut ans:i32 = 0;
        for i in 1..1001 {
            if self.kv.contains_key(&i) {
                continue
            }
            self.kv.insert(i, true);
            ans = i;
            break
        }
        ans
    }

    fn add_back(&mut self, num:i32) {
        self.kv.remove(&num);
    }
}

// 力扣的解
struct SmallestInfiniteSet {
    pop_nums:BTreeSet<i32>,
    water_level:i32
}

impl SmallestInfiniteSet {
    fn new() ->Self {
        Self {
            pop_nums:BTreeSet::new(),
            water_level:1
        }
    }
    fn pop_smallest(&mut self) -> i32 {
        if self.pop_nums.is_empty() {
            let ans = self.water_level;
            self.water_level += 1;
            return ans;
        }
        let mut ans = *self.pop_nums.iter().next().unwrap();
        self.pop_nums.remove(&ans);
        ans
    }

    fn add_back(&mut self, num:i32) {
        if num >= self.water_level {
            return;
        }
        self.pop_nums.insert(num);
    }
}