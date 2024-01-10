//
// Created by lsill on 2022/8/17.
//

#include "tree_pra.h"

TreeNode::TreeNode() {
    val = 0;
    left = nullptr;
    right = nullptr;
}

TreeNode::TreeNode(int x) {
    val = x;
    left = nullptr;
    right = nullptr;
}

TreeNode::TreeNode(int x, TreeNode *left, TreeNode *right){
    val = x;
    this->left = left;
    this->right = right;
}

int TreeTest1::deepestLeavesSum(TreeNode *root) {
    dfs(root, 0);
    return sum;
}

void  TreeTest1::dfs(TreeNode* node, int level) {
    if (node == nullptr) {
        return;
    }
    if (level > maxLevel) {
        maxLevel = level;
        sum = node->val;
    } else if(level == maxLevel) {
        sum += node->val;
    }
    dfs(node->left, level+1);
    dfs(node->right, level+1);
}


TreeTest1::TreeTest1(){
    sum = 0;
    maxLevel = -1;
}