mod sequence;
mod math_simu;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use sequence::pivot_integer;
    use math_simu::*;
    #[test]
    fn it_works() {
        println!("{}",find_the_array_conc_val(vec![7,52,2,4]))
    }
}
