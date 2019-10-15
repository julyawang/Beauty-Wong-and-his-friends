
mod one;
use one::List;
mod two;
use two::ListNode;




fn main() {
    // one
//    let mut head = List::new();
//    let mut i=0;
//    while i < 10 {
//        i+=1;
//        head = head.prepend(i);
//    }
//    println!("{}", head.stringify());
//    let result = List::new();
//    let result = one::List::reverse_List(head, result);
//    println!("{}", result.stringify());

    // two
//    let mut list = ListNode::new( 1);
//    let mut i=2;
//    while i < 10 {
//        list.push(i);
//        i+=1;
//    }
//    println!("{:?}", list);
//    let result = two::ListNode::reverse_list(list.next.take());
//    println!("{:?}", result);

    // three
//    let mut circle = List::new();
//    // 创建带环的链表
//    let mut j= 0;
//    while j < 6 {
//        j += 1;
//        circle = circle.prepend(j);
//
//    }
//    println!("{}", circle.stringify());
//    let mut rukou = List::new();
//    match circle {
//        List::Cons(value, tail) => {
//            rukou = *tail;
//            println!("rukou: {}", rukou.stringify());
//
//        }
//        List::Nil => {
//
//        }
//    }

    let mut head = List::new();
    let mut i= 0;
    while i < 10 {
        i += 1;
        head = head.prepend(i);
    }

    println!("{:?}", head);
    let mut rukou = List::new();

    let mut copy_head = List::new();
    let mut i= 0;

    while i < 10 {
        i += 1;
        copy_head = copy_head.prepend(i);
    }
    println!("{:?}", copy_head);

    loop {
        match copy_head {
            List::Cons(value, mut tail) => {
                copy_head = *(tail.clone());
                if value == 6 {
                    rukou = *(tail.clone());
                    println!("tail: {}", rukou.clone().stringify());
                    break;
                }
            }
            List::Nil => {
            }
        }
    }

//    for index in head {
//
//    }

    loop {
        match head {
            List::Cons(value, mut tail) => {
                head = *(tail.clone());
                if value == 1 {
                    tail = Box::new(rukou.clone());
                    println!("head: {}", head.clone().stringify());
                    break;
                }
            }
            List::Nil => {
            }
        }
    }




//    let mut head2 = List::new();
//    let mut j= 0;
//    while j < 10 {
//        j += 1;
//        head2 = head2.prepend(j);
//    }
//    loop {
//        match head2 {
//            List::Cons(value, mut tail) => {
//                if value == 1 {
//                    *tail = rukou.clone();
//                    println!("{}", head2.clone().stringify());
//                }
//                break;
//            }
//            List::Nil => {
//            }
//        }
//    }
//
//
    let result = head.Is_circle();
    println!("{:?}", result);

//    let mut head_no_circle = List::new();
//    let mut i=0;
//    while i < 10 {
//        i+=1;
//        head_no_circle = head_no_circle.prepend(i);
//    }
//
//    println!("{}", head_no_circle.stringify());
//    print!("{}", head_no_circle.Is_circle());

//    let mut list2 = ListNode::new( 1);
//    let mut i= 0;
//    while i < 5 {
//        list2.push(i);
//        i += 1;
//    }
//    let merge =list.merge_list(list2);
//    println!("merge: {:?}", merge);

}


