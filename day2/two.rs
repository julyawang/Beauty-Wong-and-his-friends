use std::ptr::null;
use crate::one::List::Nil;
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Head<T> {
    //    len: usize,
    next: Option<Box<ListNode<T>>>,
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode<T> {
    pub value: T,
    pub next: Option<Box<ListNode<T>>>,
}


impl ListNode<u64> {

    pub fn new(elem: u64) -> Self {
        ListNode {
            value: elem,
            next: None,
        }
    }

    pub fn set_next(&mut self, node: Self) {
        self.next = Some(Box::new(node));
    }

    pub fn get_last<'a>(&'a mut self) -> &'a mut Self {
        if let Some(ref mut x) = self.next {
            return x.get_last();
        }
        self
    }

    pub fn push(&mut self, elem: u64) {
        let new_node = ListNode::new(elem);
        self.get_last().set_next(new_node);
    }
//    pub fn reverse_list(list: ListNode<u64>) -> ListNode<u64> {
//        let mut pre = ListNode{
//            value: 0,
//            next: None
//        };
//        let mut cur = list;
//        while cur.next != Nil {
//            let nextTemp = cur.clone();
//            cur.next = pre.next;
//            pre = cur;
//            cur = nextTemp;
//        }
//        pre
//
//    }
    pub fn reverse_list(head: Option<Box<ListNode<u64>>>) -> Option<Box<ListNode<u64>>> {
        let mut head = head;
        let mut tail = None;
        while let Some(mut n) = head.take() {
            head = n.next;
            n.next = tail;
            tail = Some(n);
        }
        tail
    }
//    pub fn merge_list(&mut self, list: ListNode<u64>) -> ListNode<u64> {
//        let one = self.clone();
//        let mut dummy_head = Some(Box::new(ListNode {
//            value: 0 as u64,
//            next: None,
//        }));
//        // 获取链表长度
//        let mut len = 0;
//        let mut p = dummy_head.as_ref();
//        while p.unwrap().next.is_some() {
//            len += 1;
//            p = p.unwrap().next.as_ref();
//        }
//        for _ in 0..(idx) {
//            p = p.unwrap().next;
//        }
//        let next = p.as_mut().unwrap().next.as_mut().unwrap().next.take();
//        p.as_mut().unwrap().next = next;
//        let next = list.next;
//        p.as_mut().unwrap().next = next.unwrap().next;
//        *(dummy_head.unwrap().next.unwrap())
//
//    }
    pub fn circle(&mut self, head: ListNode<u64>) -> bool {
        let mut fast = head.clone();
        let mut slow = head.clone();
        while (fast.next.is_none() == false) && (slow.next.is_none() == false) {
            if fast.value == slow.value {
                return true;
            }
            slow = *(slow.next.unwrap());
            fast = *(fast.next.unwrap());
            if fast.next.is_none() == false {
                fast = *(fast.next.unwrap());
            }
        }
        return false;
    }



//    pub fn circle_entry

}





