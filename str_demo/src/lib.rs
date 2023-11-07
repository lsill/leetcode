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
        let num = vowel_strings(vec!["are".to_string(),"amy".to_string(),"u".to_string()], 0, 2);
    }
}
