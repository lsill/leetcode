mod list_base;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use std::iter::Map;
    use std::option::IntoIter;
    use super::*;
    use list_base::*;
    #[test]
    fn it_works() {
        let mut node1_3 = Some(Box::new(ListNode{ val: 3, next: None }));
        let mut node1_2 = Some(Box::new(ListNode{ val: 4, next: node1_3 }));
        let mut node1_1 = Some(Box::new(ListNode{ val: 2, next: node1_2 }));
        let mut node1_0 = Some(Box::new(ListNode{ val: 7, next: node1_1 }));

        let mut node2_2 = Some(Box::new(ListNode{ val: 4, next: None }));
        let mut node2_1 = Some(Box::new(ListNode{ val: 6, next: node2_2 }));
        let mut node2_0 = Some(Box::new(ListNode{ val: 5, next: node2_1 }));

        //add_two_numbers_plus(node1_0, node2_0);
        reverse_list_no(&mut node1_0);
        while let Some(node) = node1_0{
            let x = node.val;
            println!("val {}",x);
            node1_0 = node.next;
        }
    }
}
