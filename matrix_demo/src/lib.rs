mod simple_matrix;
mod matrix_dm;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use simple_matrix::*;
    use crate::matrix_dm::find_peak_grid_1;

    #[test]
    fn it_works() {
        find_peak_grid_1(vec![vec![1,4],vec![3,2]]);
    }
}
