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

/// 445. 两数相加 II
/// 给你两个 非空 链表来代表两个非负整数。数字最高位位于链表开始位置。它们的每个节点只存储一位数字。将这两数相加会返回一个新的链表。
/// 示例1
/// 输入：l1 = [7,2,4,3], l2 = [5,6,4]
/// 输出：[7,8,0,7]

// lc 不用递归的做法
pub fn add_two_numbers_plus(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    let mut a = Vec::with_capacity(128);
    let mut b = Vec::with_capacity(128);
    let mut x = l1;
    while let Some(t) = x {
        x = t.next;
        a.push(t.val);
    }
    let mut x = l2;
    while let Some(t) = x {
        x = t.next;
        b.push(t.val);
    }
    let mut a = a.into_iter().rev();
    let mut b = b.into_iter().rev();
    let mut ans = None;
    let mut cur = 0;
    let mut done_a = false;
    let mut done_b = false;
    loop {
        let x = if let Some(t) = a.next() {t}
        else {done_a = true; 0};
        let y = if let Some(t) = b.next() {t}
        else {done_b = true; 0};
        cur += x + y;
        let t = cur % 10;
        cur /= 10;
        if done_a && done_b && cur + t == 0 {break}
        ans = Some(Box::new(ListNode{
            val: t, next: ans
        }));
    }
    if ans.is_none() {
        Some(Box::new(ListNode::new(0)))
    } else {ans}
}

pub fn reverse_list(head :Option<Box<ListNode>>)->Option<Box<ListNode>> {
    let mut pre = None;
    let mut head = head;
    while let Some(mut node) = head {
        head = node.next.take();
        node.next= pre;
        pre = Some(node);
    }
    pre
}

pub fn reverse_list_mut(mut head : Option<Box<ListNode>>)->Option<Box<ListNode>> {
    let mut res = None;
    while let Some(mut node) = head {
        head = node.next.take();
        node.next = res;
        res = Some(node);
    }
    res
}

pub fn reverse_list_no(head: &mut Option<Box<ListNode>>) {
    let mut prev = None;
    let mut current:Option<Box<ListNode>> = head.take();
    while let Some(mut node) = current {
        let next =node.next.take();
        node.next = prev;
        prev = Some(node);
        current = next;
    }
    *head = prev
}

/// 143. 重排链表
/// 给定一个单链表 L 的头节点 head ，单链表 L 表示为：
/// L0 → L1 → … → Ln - 1 → Ln
/// 请将其重新排列后变为：
/// L0 → Ln → L1 → Ln - 1 → L2 → Ln - 2 → …
/// 不能只是单纯的改变节点内部的值，而是需要实际的进行节点交换。
// 把链表存储到线性表中，然后用双指针依次从头尾取元素
pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
    let mut nodes:Vec<Option<Box<ListNode>>> = Vec::new();
    let mut curr = head.take();
    while let Some(mut node) = curr{
        curr = node.next.take();
        nodes.push(Some(node));
    }
    let len = nodes.len();
    let mid = len / 2;
    let mut dummy = Box::new(ListNode::new(i32::default()));
    let mut curr = &mut dummy;
    for i in 0..mid {
        curr.next = nodes[i].take();
        curr = curr.next.as_mut().unwrap();
        curr.next = nodes[len -i -1].take();
        curr = curr.next.as_mut().unwrap();
    }
    if mid < len - mid {
        curr.next = nodes[mid].take();
    }
    *head = dummy.next
}