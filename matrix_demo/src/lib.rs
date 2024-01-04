mod simple_matrix;
mod matrix_dm;
mod matrix_middle;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use simple_matrix::*;
    use crate::matrix_dm::find_peak_grid_1;
    use crate::matrix_middle::maximum_rows;

    #[test]
    fn it_works() {
        maximum_rows(vec![vec![0,0,0],vec![1,0,1],vec![0,1,1],vec![0,0,1]],2);
    }
}
