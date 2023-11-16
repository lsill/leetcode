mod simple_demo;
mod middle_dm;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use simple_demo::*;
    use middle_dm::*;

    #[test]
    fn it_works() {
        longest_alternating_subarray(vec![4], 3);
    }
}
