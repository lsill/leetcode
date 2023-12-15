use std::rc::Rc;
use std::cell::{Ref, RefCell, RefMut};
use std::char::from_digit;
use std::collections::VecDeque;
use std::ptr::{addr_of_mut, NonNull, write};
use crate::tree_base::TreeNode;

/// 617. 合并二叉树
/// 给你两棵二叉树： root1 和 root2 。
/// 想象一下，当你将其中一棵覆盖到另一棵之上时，两棵树上的一些节点将会重叠（而另一些不会）。
/// 你需要将这两棵树合并成一棵新二叉树。合并的规则是：如果两个节点重叠，那么将这两个节点的值相加作为合并后节点的新值；否则，不为 null 的节点将直接作为新二叉树的节点。
/// 返回合并后的二叉树。
/// 注意: 合并过程必须从两个树的根节点开始。

// lc题解
pub fn merge_trees(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match (root1.clone(), root2.clone()) {
        (Some(r1), Some(r2)) => {
            let r1_child_left = r1.borrow_mut().left.take();
            let r1_child_right = r1.borrow_mut().right.take();

            let r2_child_left = r2.borrow_mut().left.take();
            let r2_child_right = r2.borrow_mut().right.take();

            Some(Rc::new(RefCell::new(TreeNode {
                val: r1.borrow().val + r2.borrow().val,
                left: merge_trees(r1_child_left, r2_child_left),
                right: merge_trees(r1_child_right, r2_child_right),
            })))
        }
        _ => root1.or(root2),
    }

}
// lc 比较优秀的解
pub fn merge_trees_1(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    match (root1, root2) {
        (Some(t1), Some(t2))=> match (t1.borrow_mut(), t2.borrow_mut()) {
            (mut t1, mut t2)=> Some(Rc::new(RefCell::new(TreeNode{
                val:t1.val + t2.val,
                left:merge_trees_1(t1.left.take(), t2.left.take()),
                right:merge_trees_1(t1.right.take(), t2.right.take()),
            })))
        }
        (None,None)=>None,
        (None,some)=>some,
        (some, None)=>some,
    }
}

/// 1038. 从二叉搜索树到更大和树
/// 给定一个二叉搜索树 root (BST)，请将它的每个节点的值替换成树中大于或者等于该节点值的所有节点值之和。
/// 提醒一下， 二叉搜索树 满足下列约束条件：
/// 节点的左子树仅包含键 小于 节点键的节点。
/// 节点的右子树仅包含键 大于 节点键的节点。
/// 左右子树也必须是二叉搜索树。
/// 示例 1：
/// 输入：[4,1,6,0,2,5,7,null,null,null,3,null,null,null,8]
/// 输出：[30,36,21,36,35,26,15,null,null,null,33,null,null,null,8]
/// 示例 2：
/// 输入：root = [0,null,1]
/// 输出：[1,null,1]
/// 提示：
/// 树中的节点数在 [1, 100] 范围内。
/// 0 <= Node.val <= 100
/// 树中的所有值均 不重复 。

pub fn bst_to_gst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    fn dfs(node:Option<&Rc<RefCell<TreeNode>>>, s: &mut i32) {
        if let Some(x) = node {
            let mut x = x.borrow_mut();
            dfs(x.right.as_ref(), s) ;
            *s += x.val;
            x.val = *s;
            dfs(x.left.as_ref(), s);
        }
    }
    let mut s = 0;
    dfs(root.as_ref(), &mut s);
    root
}

/// 2415. 反转二叉树的奇数层
/// 给你一棵 完美 二叉树的根节点 root ，请你反转这棵树中每个 奇数 层的节点值。
/// 例如，假设第 3 层的节点值是 [2,1,3,4,7,11,29,18] ，那么反转后它应该变成 [18,29,11,7,4,3,1,2] 。
/// 反转后，返回树的根节点。
/// 完美 二叉树需满足：二叉树的所有父节点都有两个子节点，且所有叶子节点都在同一层。
/// 节点的 层数 等于该节点到根节点之间的边数。
/// 示例 1：
/// 输入：root = [2,3,5,8,13,21,34]
/// 输出：[2,5,3,8,13,21,34]
/// 解释：
/// 这棵树只有一个奇数层。
/// 在第 1 层的节点分别是 3、5 ，反转后为 5、3 。
/// 示例 2：
/// 输入：root = [7,13,11]
/// 输出：[7,11,13]
/// 解释：
/// 在第 1 层的节点分别是 13、11 ，反转后为 11、13 。
/// 示例 3：
/// 输入：root = [0,1,2,0,0,0,0,1,1,1,1,2,2,2,2]
/// 输出：[0,2,1,0,0,0,0,2,2,2,2,1,1,1,1]
/// 解释：奇数层由非零值组成。
/// 在第 1 层的节点分别是 1、2 ，反转后为 2、1 。
/// 在第 3 层的节点分别是 1、1、1、1、2、2、2、2 ，反转后为 2、2、2、2、1、1、1、1 。
/// 提示：
/// 树中的节点数目在范围 [1, 214] 内
/// 0 <= Node.val <= 105
/// root 是一棵 完美 二叉树

// 力扣一种解
pub fn reverse_odd_levels_1(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    use std::ptr::null_mut;
    fn dfs(root:&Option<Rc<RefCell<TreeNode>>>, tab: &mut Vec<*mut i32>, idx:usize) -> usize {
        if let Some(root) = root {
            tab[idx] = &mut root.borrow_mut().val as *mut i32;
            dfs(&root.borrow().left, tab, idx *2);
            dfs(&root.borrow_mut().right, tab, idx * 2 + 1) + 1
        } else {
            0
        }
    }
    let mut tab = vec![null_mut(); (1 << 14) + 1];
    let hight = dfs(&root, &mut tab, 1);
    unsafe {
        for h in (1..hight).step_by(2) {
            let mut i = 2usize.pow(h as u32);
            let mut j = 2usize.pow(h as u32 + 1) - 1;
            while i < j {
                std::mem::swap(&mut (*tab[i]), &mut (*tab[j]));
                i += 1;
                j -= 1;
            }
        }
    }
    root
}

// 力扣最优解
pub fn reverse_odd_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut queue = VecDeque::new();
    if let Some(root) = root.clone() {
        queue.push_back(root)
    }
    let mut is_odd_level = false;
    while !queue.is_empty() {
        let level_size = queue.len();
        let mut level_nodes = Vec::new();
        let mut values = Vec::new();

        for _ in 0..level_size {
            if let Some(node) = queue.pop_front() {
                values.push(node.borrow().val);
                level_nodes.push(node.clone());
                if let Some(left) = node.borrow().left.clone() {
                    queue.push_back(left);
                }
                if let Some(right) = node.borrow().right.clone() {
                    queue.push_back(right);
                }
            }
        }
        if is_odd_level {
            values.reverse();
            for (i, node) in level_nodes.iter().enumerate() {
                node.borrow_mut().val = values[i];
            }
        }
        is_odd_level = !is_odd_level;
    }
    root
}