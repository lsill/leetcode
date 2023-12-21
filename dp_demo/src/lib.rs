//! 动态规划

mod simple_dp;
mod classical_demo;
mod single_stack;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use simple_dp::*;
    #[test]
    fn it_works() {
        maximum_sum_bc(vec![1,-2,0,3]);
    }
}
