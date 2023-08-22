mod simple_demo;
mod middle_dm;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use simple_demo::*;
    use middle_dm::*;

    #[test]
    fn it_works() {
        let  num = max_dist_to_closest(vec![0,0,1]);
        println!("{}",num );
    }
}
