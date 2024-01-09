//! 动态规划

mod simple_dp;
mod classical_demo;
mod single_stack;
mod dp_hash;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use simple_dp::*;
    use crate::dp_hash::min_extra_char;

    #[test]
    fn it_works() {
        min_extra_char("leetscode".to_string(),vec!["leet".to_string(), "code".to_string(), "leetcode".to_string()] );
    }
}
