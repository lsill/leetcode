#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val:i32,
    pub next:Option<Box<ListNode>>
}

impl ListNode {
    #[inline]
    fn new(val:i32)->Self {
        ListNode{
            next:None,
            val
        }
    }
}

/// 2. 两数相加
/// 给你两个非空 的链表，表示两个非负的整数。它们每位数字都是按照逆序的方式存储的，并且每个节点只能存储一位数字。
/// 请你将两个数相加，并以相同形式返回一个表示和的链表。
/// 你可以假设除了数字 0 之外，这两个数都不会以 0开头。
/// 实例1
/// 输入：l1 = [2,4,3], l2 = [5,6,4]
/// 输出：[7,0,8]
/// 解释：342 + 465 = 807.

// 力扣递归做法
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
   carried(l1, l2, 0)
}
pub fn carried(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>, mut carry: i32) -> Option<Box<ListNode>> {
    if l1.is_none() && l2.is_none() && carry == 0 {
        None
    } else {
        Some(Box::new(ListNode {
            next: carried(
                l1.and_then(|x| {carry += x.val; x.next}),
                l2.and_then(|x| {carry += x.val; x.next}),
                carry / 10
            ),
            val: carry % 10
        }))
    }
}

