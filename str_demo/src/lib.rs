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
        let num = max_product(vec!["abcw".to_string(),"baz".to_string(),"foo".to_string(),"bar".to_string(),"xtfn".to_string(),"abcdef".to_string()]);
        assert_eq!(num, 16)
    }
}
