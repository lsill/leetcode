//
// Created by lsill on 2024/1/15.
//

#ifndef LC_LIST_BASE_H
#define LC_LIST_BASE_H


 struct ListNode {
    int val;
    ListNode *next;
    ListNode() : val(0), next(nullptr) {}
    ListNode(int x) : val(x), next(nullptr) {}
    ListNode(int x, ListNode *next) : val(x), next(next) {}
};

class ListDemo {
public:
    ListNode* deleteDuplicates_1(ListNode* head);
    ListNode* deleteDuplicates(ListNode* head);
};


#endif //LC_LIST_BASE_H
