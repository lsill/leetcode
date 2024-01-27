#include "list_base.h"
#include <unordered_map>

// 82. 删除排序链表中的重复元素 II
// 给定一个已排序的链表的头 head ， 删除原始链表中所有重复数字的节点，只留下不同的数字 。返回 已排序的链表 。
// 示例 1：
// 输入：head = [1,2,3,3,4,4,5]
// 输出：[1,2,5]
// 示例 2：
// 输入：head = [1,1,1,2,3]
// 输出：[2,3]
// 提示：
// 链表中节点数目在范围 [0, 300] 内
// -100 <= Node.val <= 100
// 题目数据保证链表已经按升序 排列

ListNode* ListDemo::deleteDuplicates_1(ListNode* head){
    if (!head || !head->next) return head;
    ListNode* preHead = new ListNode(0);
    preHead->next = head;
    ListNode* pre = preHead;
    ListNode* cur = head;
    while(cur) {
        while(cur->next && cur->val == cur->next->val) {
            cur = cur->next;
        }
        if (pre->next == cur) {
            pre = pre->next;
        } else {
            pre->next = cur->next;
        }
        cur = cur->next;
    }
    return preHead->next;
}

ListNode* ListDemo::deleteDuplicates(ListNode *head) {
    if (!head) {
        return head;
    }
    ListNode dummy(0,head);
    ListNode* cur = &dummy;
    while (cur->next && cur->next->next) {
        if (cur->next->val == cur->next->next->val) {
            int x = cur->next->val;
            while (cur->next && cur->next->val == x) {
                ListNode* tmp = cur->next; // 删除跳过的节点
                cur->next = cur->next->next;
                delete tmp;
            }
        }else {
            cur = cur->next;
        }
    }
    return dummy.next;
}