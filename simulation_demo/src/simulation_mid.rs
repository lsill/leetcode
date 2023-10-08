use std::collections::{BTreeMap, HashMap};

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
