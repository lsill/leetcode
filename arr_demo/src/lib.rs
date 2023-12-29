mod simple_demo;
mod middle_dm;
mod typical_rust;

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
        buy_choco(vec![98,54,6,34,66,63,52,39], 62);
    }
}
