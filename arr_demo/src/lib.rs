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
        first_complete_index(vec![1,3,4,2], vec![vec![1,4],vec![2,3]]);
    }
}
