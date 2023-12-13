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
        //
        let ans = make_smallest_palindrome("seven".to_string());
        println!("{}", ans)
    }
}
