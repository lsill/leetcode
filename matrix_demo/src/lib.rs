mod simple_matrix;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use simple_matrix::*;

    #[test]
    fn it_works() {
        let n = matrix_sum(vec![vec![7,2,1],vec![6,4,2],vec![6,5,3],vec![3,2,1]]);
        println!("{}",n);
    }
}
