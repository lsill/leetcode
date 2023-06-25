mod two_ptr;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use two_ptr::*;
    #[test]
    fn it_works() {
        println!("res {}",num_subarray_product_less_than_k(vec![10,5,2,6],100));
    }
}
