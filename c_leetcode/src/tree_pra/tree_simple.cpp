//
// Created by lsill on 2024/2/10.
//

#include "tree_base.h"
#include <stack>

// 94. 二叉树的中序遍历
//给定一个二叉树的根节点 root ，返回 它的 中序 遍历 。
//示例 1：
//输入：root = [1,null,2,3]
//输出：[1,3,2]
//示例 2：
//输入：root = []
//输出：[]
//示例 3：
//输入：root = [1]
//输出：[1]
//提示：
//树中节点数目在范围 [0, 100] 内
//-100 <= Node.val <= 100
// 栈迭代算法
std::vector<int> TreeSimple::inorderTraversal_iter(TreeNode *root) {
    std::vector<int> res;
    std::stack<TreeNode*> stk;
    while (root != nullptr || !stk.empty()) {
        while (root != nullptr) {
            stk.push(root);
            root = root->left;
        }
        root = stk.top();
        stk.pop();
        res.push_back(root->val);
        root = root->right;
    }
    return res;
}

std::vector<int> TreeSimple::inorderTraversal(TreeNode *root) {
    std::vector<int> ans;
    inorderTraversal_dfs(ans, root);
    return ans;
}

void TreeSimple::inorderTraversal_dfs(std::vector<int> &ans, TreeNode *node) {
    if (node == nullptr) {
        return;
    }
    if (node->left) {
        inorderTraversal_dfs(ans, node->left);
    }
    ans.push_back(node->val);
    if (node->right) {
        inorderTraversal_dfs(ans, node->right);
    }
}

// 145. 二叉树的后序遍历
//给你一棵二叉树的根节点 root ，返回其节点值的 后序遍历 。
//示例 1：
//输入：root = [1,null,2,3]
//输出：[3,2,1]
//示例 2：
//输入：root = []
//输出：[]
//示例 3：
//输入：root = [1]
//输出：[1]
//提示：
//树中节点的数目在范围 [0, 100] 内
//-100 <= Node.val <= 100
//进阶：递归算法很简单，你可以通过迭代算法完成吗？
// 迭代算法
std::vector<int> TreeSimple::postorderTraversal(TreeNode *root) {
    std::vector<int> ans;
    std::stack<TreeNode*> stk;
    if (root == nullptr) {
        return ans;
    }
    TreeNode* prev = nullptr;
    while (root != nullptr || !stk.empty()) {
        while (root != nullptr) {
            stk.emplace(root);
            root = root->left;
        }
        root = stk.top();
        stk.pop();
        if (root->right == nullptr || root->right == prev) {
            ans.emplace_back(root->val);
            prev = root;
            root = nullptr;
        } else {
            stk.emplace(root);
            root = root->right;
        }
    }
    return ans;
}

// 144. 二叉树的前序遍历
//已解答
//简单
//相关标签
//相关企业
//给你二叉树的根节点 root ，返回它节点值的 前序 遍历。
//示例 1：
//输入：root = [1,null,2,3]
//输出：[1,2,3]
//示例 2：
//输入：root = []
//输出：[]
//示例 3：
//输入：root = [1]
//输出：[1]
//示例 4：
//输入：root = [1,2]
//输出：[1,2]
//示例 5：
//输入：root = [1,null,2]
//输出：[1,2]
//提示：
//树中节点数目在范围 [0, 100] 内
//-100 <= Node.val <= 100
//进阶：递归算法很简单，你可以通过迭代算法完成吗？
// 迭代算法
std::vector<int> preorderTraversal(TreeNode *root) {
    std::vector<int> res;
    if (root == nullptr) {
        return res;
    }

    std::stack<TreeNode*> stk;
    TreeNode* node = root;
    while (!stk.empty() || node != nullptr) {
        while (node != nullptr) {
            res.emplace_back(node->val);
            stk.emplace(node);
            node = node->left;
        }
        node = stk.top();
        stk.pop();
        node = node->right;
    }
    return res;
}

// 938. 二叉搜索树的范围和
//给定二叉搜索树的根结点 root，返回值位于范围 [low, high] 之间的所有结点的值的和。
//示例 1：
//输入：root = [10,5,15,3,7,null,18], low = 7, high = 15
//输出：32
//示例 2：
//输入：root = [10,5,15,3,7,13,18,1,null,6], low = 6, high = 10
//输出：23
//提示：
//树中节点数目在范围 [1, 2 * 104] 内
//1 <= Node.val <= 105
//1 <= low <= high <= 105
//所有 Node.val 互不相同

// 自己做的递归
int TreeSimple::rangeSumBSTSelf(TreeNode *root, int low, int high) {
    int ans = 0;
    std::function<void(TreeNode*)> fn = [&](TreeNode* node) {
        if (node == nullptr) {
            return ;
        }
        if (node->val >= low && node->val <= high) {
            ans += node->val;
        }
        fn(node->left);
        fn(node->right);
    };
    fn(root);
    return ans;
}

// lc写的好的递归
int TreeSimple::rangeSumBST(TreeNode* root, int low, int high) {

    if(root == nullptr)
        return 0;
    if(root->val < low){
        return rangeSumBST(root->right, low, high);
    }
    if(root->val > high)
        return rangeSumBST(root->left, low, high);
    // dfs(root);
    // return sum;
    return root->val + rangeSumBST(root->right, low, high)+rangeSumBST(root->left, low, high);
}

#include <queue>

// lc 广度优先，队列
int TreeSimple::rangeSumBSTBFS(TreeNode *root, int low, int high) {
    int sum = 0;
    std::queue<TreeNode*> q({root});
    while (!q.empty()) {
        auto node = q.front();
        q.pop();
        if (node == nullptr) {
            continue;
        }
        if (node ->val > high) {
            q.push(node->left);
        } else if (node->val < low) {
            q.push(node->right);
        } else {
            sum += node->val;
            q.push(node->left);
            q.push(node->right);
        }
    }
    return sum;
}