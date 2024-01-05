mod simple_demo;
mod middle_dm;
mod typical_rust;
mod single_stack;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use simple_demo::*;
    use middle_dm::*;
    use crate::single_stack::can_see_persons_count;

    #[test]
    fn it_works() {
        can_see_persons_count(vec![10,6,8,5,11,9]);
    }
}
