use std::rc::Rc;
use std::cell::{Ref, RefCell, RefMut};
use std::ptr::{addr_of_mut, NonNull};
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
