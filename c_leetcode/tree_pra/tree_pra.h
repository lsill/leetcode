//
// Created by lsill on 2022/8/17.
//

#ifndef C_LEETCODE_TREE_PRA_H
#define C_LEETCODE_TREE_PRA_H


struct TreeNode {
    int val;
    TreeNode *left;
    TreeNode *right;
    TreeNode();
    TreeNode(int x);
    TreeNode(int x, TreeNode *left, TreeNode *right);
};


class TreeTest1 {
private:
    int maxLevel;
    int sum;
public:
    TreeTest1();
    int deepestLeavesSum(TreeNode *root);
    void dfs(TreeNode *node, int level);
};

#endif //C_LEETCODE_TREE_PRA_H
