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
        println!("{}",ways_to_buy_pens_pencils(20,10,5))
    }
}
