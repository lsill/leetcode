mod char_demo;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use char_demo::*;
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
    #[test]
    fn test1() {
        count_points("G6B6B0G1B0G6G7G2G6R9G0G9B1G5".to_string());
    }
}
