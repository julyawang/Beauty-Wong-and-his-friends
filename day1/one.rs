
use List::*;
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum List {
    // Cons: 包含一个元素和一个指向下一个节点的指针的元组结构
    Cons(u32, Box<List>),
    // Nil: 表示一个链表节点的末端
    Nil,
}
impl List {
    // 创建一个空链表
    pub fn new() -> List {
        Nil
    }

    // 在前面加一个元素节点，链接旧的链表，返回新的链表
    pub fn prepend(self, elem: u32) -> List {
        Cons(elem, Box::new(self))
    }


    pub fn reverse_List(old: List, new: List) -> List {
        match old {
            Cons(val, tail) => {
                List::reverse_List(*tail, new.prepend(val))
            }
            Nil => new
        }
    }
    pub fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                // `format!` 和 `print!` 类似，但返回的是一个堆分配的字符串，而不是
                // 打印结果到控制台上
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }

    pub fn Is_circle(&mut self) -> bool {
        let mut fast = self.clone();
        let mut slow = self.clone();
        let mut fast_value = 0 as u32;
        let mut slow_value = 0 as u32;
        loop {
            match slow {
                Cons(value, mut tail) => {
                    println!("slow: {:?}, {:?}", value, tail);
                    slow_value = value.clone();
                    slow = *tail;
                    match fast {
                        Cons(value, tail) => {
                            fast_value = value.clone();
                            fast = *tail;
                            match fast {
                                Cons(value, tail) => {
                                    println!("fast: {:?}, {:?}", value, tail);
                                    fast_value = value.clone();
                                    fast = *tail;
                                    if fast_value == slow_value {
//                                        let meet = List::Cons(fast_value, tail.clone());

                                        return true;
                                    }
                                }
                                Nil => {
                                    return false;
                                }
                            }
                        },
                        Nil => {
                            return false;
                        }
                    }
                }
                Nil => {
                    return false;
                }
            }
        }
    }
//    pub fn circle_entrance(&mut self) -> List {
//        let mut start = self.clone();
//        let mut meet = self.clone();
//        let mut start_value = 0 as u32;
//        let mut meet_value = 0 as u32;
//        loop {
//            match start {
//                Cons(value, tail) => {
//
//                }
//                Nil => {
//
//                }
//
//            }
//        }
//
//    }

}



