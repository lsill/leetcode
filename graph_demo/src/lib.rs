mod mid_graph;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;
    use mid_graph::*;

    #[test]
    fn it_works() {
        minimum_fuel_cost(vec![vec![3,1],vec![3,2],vec![1,0],vec![0,4],vec![0,5],vec![4,6]], 2);
    }
}
