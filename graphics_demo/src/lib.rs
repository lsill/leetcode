mod twoD_coordinates;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use twoD_coordinates::*;
    #[test]
    fn it_works() {
        check_overlap_1(1,0,0,1,-1,3,1);
    }
}
