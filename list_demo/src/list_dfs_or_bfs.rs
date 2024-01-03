use crate::list_base::ListNode;

/// 2487. 从链表中移除节点
/// 给你一个链表的头节点 head 。
/// 移除每个右侧有一个更大数值的节点。
/// 返回修改后链表的头节点 head 。
/// 示例 1：
/// 输入：head = [5,2,13,3,8]
/// 输出：[13,8]
/// 解释：需要移除的节点是 5 ，2 和 3 。
/// - 节点 13 在节点 5 右侧。
/// - 节点 13 在节点 2 右侧。
/// - 节点 8 在节点 3 右侧。
/// 示例 2：
/// 输入：head = [1,1,1,1]
/// 输出：[1,1,1,1]
/// 解释：每个节点的值都是 1 ，所以没有需要移除的节点。
/// 提示：
/// 给定列表中的节点数目在范围 [1, 105] 内
/// 1 <= Node.val <= 105

// 别人写的比较难受的解
pub fn remove_nodes_1(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    match head {
        Some(mut boxed_head)=>{
            if let Some(mut next_node) = boxed_head.next.take(){
                let node = remove_nodes_1(Some(next_node));
                if boxed_head.val < node.as_ref().unwrap().val {
                    node
                } else {
                    boxed_head.next = node;
                    Some(boxed_head)
                }
            } else {
                Some(boxed_head)
            }

        }
        None=> None,
    }
}

// lc dfs 解
pub fn remove_nodes(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    fn dfs(mx:&mut i32, head:Option<Box<ListNode>>) ->Option<Box<ListNode>> {
        if let Some(mut x) = head {
            let nxt = dfs(mx, x.next.take());
            if x.val < *mx {
                return nxt;
            }
            *mx = x.val.max(*mx);
            x.next = nxt;
            return Some(x);
        }
        None
    }
    dfs(&mut 0, head)
}