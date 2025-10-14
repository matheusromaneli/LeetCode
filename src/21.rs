// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }

// impl ListNode {
//     #[inline]
//     fn new(val: i32) -> Self {
//         ListNode {
//             next: None,
//             val
//         }
//     }
// }
impl Solution {
    pub fn merge_two_lists(list1: Option<Box<ListNode>>, list2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let Some(mut l1) = list1 else {return list2;};
        let Some(mut l2) = list2 else {return Some(l1);};
        let next;
        if l1.val < l2.val{
            next = Self::merge_two_lists(l1.next, Some(l2));
            l1.next = next;
            return Some(l1);
        }
        else{
            next = Self::merge_two_lists(Some(l1), l2.next);
            l2.next = next;
            return Some(l2);
        }
    }
}