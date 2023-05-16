//

use std::{cell::{RefCell, Ref, RefMut}, rc::{Rc, Weak}, fmt::{Debug, format}, borrow::{BorrowMut, Borrow}, ops::Deref};

#[derive(Debug)]
enum LinkType<T> {
    StrongLink(Rc<RefCell<T>>), 
    WeakLink(Weak<RefCell<T>>)
}

impl<T: Debug> std::clone::Clone for LinkType<Node<T>> {
    fn clone(&self) -> Self {
        match self {
            Self::StrongLink(arg0) => Self::StrongLink(arg0.clone()),
            Self::WeakLink(arg0) => Self::WeakLink(arg0.clone()),
        }
    }
}

#[derive(Clone, Debug)]
struct Node<T: Debug> {
    next: Option<LinkType<Node<T>>>, 
    prev: Option<LinkType<Node<T>>>,
    data: T
}

impl<T: Debug> Node<T> {
    fn new(t: T) -> Node<T> {
        Self {
            next: None, 
            prev: None, 
            data: t
        }
    }
}

#[derive(Debug)]
pub struct CdlList<T: Debug> {
    head: Option<Rc<RefCell<Node<T>>>>,
    tail: Option<Rc<RefCell<Node<T>>>>,
    size: usize
}

impl<T: Debug> std::fmt::Display for Node<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let temp = format!("{:?}", &self.data);
        f.write_str(&temp)
    }
}

impl<T: Debug> CdlList<T> {
    pub fn new() -> CdlList<T> {
        CdlList { head: None, tail: None, size: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    pub fn size(&self) -> usize {
        self.size
    }

    pub fn push_front(&mut self, t: T) {
        self.push(t, true);
    }

    pub fn push_back(&mut self, t: T) {
        self.push(t, false);
    }

    fn push(&mut self, t : T, insert_front: bool) {
        let n = Node::new(t);
        let ref_n = Rc::new(RefCell::new(n));
        
        let ref_n_clone = Rc::clone(&ref_n);
        let mut ref_n_mut = ref_n_clone.as_ref().borrow_mut();

        if self.is_empty() {
            // pointer magic
            let weak_n = Rc::downgrade(&ref_n);
            let next = LinkType::WeakLink(weak_n);
            let prev = next.clone();

            ref_n_mut.next = Some(next);
            ref_n_mut.prev = Some(prev);

            // adjust list head/tail
            self.head = Some(Rc::clone(&ref_n));

            // save ourselves the headache later?
            self.tail = Some(Rc::clone(&ref_n));
            // self.tail = Some(ref_n);
            // self.tail = None;
        } else {
            let head_ref = Rc::clone(&self.head.as_ref().unwrap());
            let tail_ref = Rc::clone(&self.tail.as_ref().unwrap());

            // node->prev = current tail always
            // let weak_tail = Rc::downgrade(&self.tail.as_ref().unwrap());
            let weak_tail = Rc::downgrade(&tail_ref);
            ref_n_mut.prev = Some(LinkType::WeakLink(weak_tail));

            // Break and add node links based on insertion point: 
            // 1. node->next is either a strong or weak link to current head
            // 2. modify head->prev or tail->next
            // 3. change head/tail to point to new node
            match insert_front {
                true => {
                    // let head_ref = self.head.take().unwrap();
                    ref_n_mut.next = Some(LinkType::StrongLink(head_ref.clone()));

                    // let head_ref = Rc::clone(&self.head.as_ref().unwrap());
                    let mut head_ref_mut = head_ref.as_ref().borrow_mut();
                    let weak_n = Rc::downgrade(&ref_n);
                    head_ref_mut.prev = Some(LinkType::WeakLink(weak_n));
                    
                    if self.size() == 1 {
                        //head->next points to self right now, need to fix
                        let weak_n = Rc::downgrade(&ref_n);
                        head_ref_mut.next = Some(LinkType::WeakLink(weak_n));
                    }

                    let _ = self.head.take();
                    self.head = Some(ref_n);
                    // self.head = Some(Rc::clone(&ref_n));
                }, 
                false => {
                    // let head_ref = Rc::downgrade(&self.head.as_ref().unwrap());
                    let weak_head = Rc::downgrade(&head_ref);
                    ref_n_mut.next = Some(LinkType::WeakLink(weak_head));

                    // let tail_ref = Rc::clone(&self.tail.as_ref().unwrap());
                    // let tail_ref = self.tail.take().unwrap();
                    let mut tail_ref_mut = tail_ref.as_ref().borrow_mut();
                    // let weak_n = Rc::downgrade(&ref_n);
                    tail_ref_mut.next = Some(LinkType::StrongLink(Rc::clone(&ref_n)));

                    if self.size == 1 {
                        //tail->prev = tail, which is wrong
                        let weak_n = Rc::downgrade(&ref_n);
                        tail_ref_mut.prev = Some(LinkType::WeakLink(weak_n));
                    }

                    let _ = self.tail.take();
                    self.tail = Some(ref_n);
                }
            };
        }

        self.size += 1;
    }

    //only for testing
    pub fn print_strong_counts(&self) {
        let head_ref = Rc::clone(&self.head.as_ref().unwrap());
        let tail_ref = Rc::clone(&self.tail.as_ref().unwrap());

        if self.size == 1 {
            println!("SC:\n\tHead: {}\n\tTail: {}\n\n\n", Rc::strong_count(&head_ref)-2, Rc::strong_count(&tail_ref)-2);
        } else {
            println!("SC:\n\tHead: {}\n\tTail: {}\n\n\n", Rc::strong_count(&head_ref)-1, Rc::strong_count(&tail_ref)-1);
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.pop(true)
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.pop(false)
    }

    fn pop(&mut self, pop_front: bool) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        self.size -= 1;

        if self.size == 0 {
            let head = self.head.take().unwrap();
            drop(head);
            let tail = self.tail.take().unwrap();
            let val = Rc::try_unwrap(tail).ok().unwrap().into_inner().data;
            return Some(val);
        } else {
            match pop_front {
                true => {
                    //pop head
                    let head = self.head.take().unwrap();
                    let node = Rc::try_unwrap(head).ok().unwrap().into_inner();
                    let next = node.next.unwrap();

                    match next {
                        LinkType::StrongLink(sl) => {
                            // fix links: 
                            // 1. next->prev = tail
                            // 2. tail->next = next
                            {
                                let tail_ref = Rc::clone(&self.tail.as_ref().unwrap());
                                let mut tail_ref_mut = tail_ref.as_ref().borrow_mut();

                                // let sl2 = Rc::clone(&sl);
                                let weak_ref = Rc::downgrade(&sl);

                                tail_ref_mut.next = Some(LinkType::WeakLink(weak_ref));
                            }

                            {
                                let tail_ref = Rc::clone(&self.tail.as_ref().unwrap());
                                let mut next_ref_mut = sl.as_ref().borrow_mut();
                                let weak_tail = Rc::downgrade(&tail_ref);

                                next_ref_mut.prev = Some(LinkType::WeakLink(weak_tail));
                            }
                            

                            self.head = Some(sl);
                        }, 
                        _ => panic!("I'd like to see you reach this")
                    }

                    return Some(node.data);
                }, 
                false => {
                    //pop tail
                    // By design, tail will have more strong links than head
                    // ==> [H] <==> ... <==> [T] <==
                    // ||                         ||
                    // =============================
                    // Tail->next = head is weak, so strong_count(&head) = 1
                    // Strong tail links are tail and tail->prev->next   = 2
                    //
                    // Therefore, must break tail->prev->next link before consuming tail
                    {
                        let tail_ref = Rc::clone(&self.tail.as_ref().unwrap());
                        let tail_borrow = tail_ref.as_ref().borrow();
                        let p = tail_borrow.prev.clone().unwrap();
                        match p {
                            LinkType::WeakLink(wl) => {
                                let up = Weak::upgrade(&wl).unwrap();
                                let mut up_ref_mut = up.as_ref().borrow_mut();
                                let weak_head = Rc::downgrade(&self.head.as_ref().unwrap());

                                let _ = up_ref_mut.next.take();
                                up_ref_mut.next = Some(LinkType::WeakLink(weak_head));
                            }, 
                            _ => panic!("Nice try")
                        };
                    }

                    let tail = self.tail.take().unwrap();

                    let node = Rc::try_unwrap(tail).ok().unwrap().into_inner();
                    let prev = node.prev.unwrap();

                    match prev {
                        LinkType::WeakLink(wl) => {
                            // fix links: 
                            // 1. prev->next = head
                            // 2. head->prev = prev
                            
                            {
                                // Upgrade prev to strong link
                                let strong_ref = Weak::upgrade(&wl).unwrap();
                                let mut strong_ref_mut = strong_ref.as_ref().borrow_mut();
                                let weak_head = Rc::downgrade(&self.head.as_ref().unwrap());

                                strong_ref_mut.next = Some(LinkType::WeakLink(weak_head));
                            }

                            {
                                let head_ref = Rc::clone(&self.head.as_ref().unwrap());
                                let mut head_ref_mut = head_ref.as_ref().borrow_mut();

                                let wl2 = Weak::clone(&wl);
                                head_ref_mut.prev = Some(LinkType::WeakLink(wl2));
                            }

                            let strong_ref = Weak::upgrade(&wl).unwrap();
                            self.tail = Some(strong_ref);
                        }, 
                        _ => panic!("I reiterate.")
                    }

                    return Some(node.data);
                }
            }
        }
    }
}