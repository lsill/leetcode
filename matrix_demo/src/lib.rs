mod simple_matrix;
mod matrix_dm;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use simple_matrix::*;

    #[test]
    fn it_works() {
        let n = diagonal_sum(vec![vec![1,2,3],vec![4,5,6],vec![7,8,9]]);
        println!("{}",n);
    }
}
